// CRATE
use gio::glib::{MainContext, WeakRef};
use gio::prelude::*;
use gtk4::prelude::{EntryExt, GtkApplicationExt, WidgetExt};
use gtk4::{glib, Application, ApplicationWindow};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::OnceLock;
use std::time::Instant;
use std::{env, process, thread};
use ui::tiles::Tile;
use ui::util::{SearchHandler, SearchUI};
use utils::config::SherlockFlags;

// MODS
mod actions;
mod application;
mod daemon;
mod g_subclasses;
mod launcher;
mod loader;
pub mod prelude;
mod ui;
mod utils;

// IMPORTS
use application::lock::{self, LockFile};
use daemon::daemon::SherlockDaemon;
use loader::pipe_loader::deserialize_pipe;
use loader::Loader;
use utils::{
    config::SherlockConfig,
    errors::{SherlockError, SherlockErrorType},
};

const SOCKET_PATH: &str = "/tmp/sherlock_daemon.socket";
const LOCK_FILE: &str = "/tmp/sherlock.lock";

struct Sherlock {
    pub window: Option<WeakRef<ApplicationWindow>>,
    pub search_ui: Option<SearchUI>,
    pub search_handler: Option<SearchHandler>,
}
impl Sherlock {
    pub fn new() -> Self {
        Self {
            window: None,
            search_ui: None,
            search_handler: None,
        }
    }
    pub fn obfuscate(&self, visibility: bool) {
        if let Some(search_bar) = self.search_ui.as_ref().and_then(|s| s.search_bar.upgrade()) {
            search_bar.set_visibility(visibility);
        }
    }
    pub fn clear_results(&self) -> Option<()> {
        let handler = self.search_handler.as_ref()?;
        if let Some(model) = handler.model.as_ref().and_then(|s| s.upgrade()) {
            model.remove_all();
        }
        Some(())
    }
    pub fn show_raw(&self) -> Option<()> {
        let ui = self.search_ui.as_ref()?;
        let handler = self.search_handler.as_ref()?;
        if let Some(title) = ui.mode_title_holder.upgrade() {
            title.set_visible(false);
            title.unparent();
        }
        if let Some(model) = handler.model.as_ref().and_then(|s| s.upgrade()) {
            model.remove_all();
        }
        if let Some(all) = ui.all.upgrade() {
            all.set_visible(false);
        }
        if let Some(status) = ui.status_bar.upgrade() {
            status.set_visible(false);
        }
        Some(())
    }
    pub fn display_pipe(&self, content: &str) -> Option<()> {
        let handler = self.search_handler.as_ref()?;
        let model = handler.model.as_ref().and_then(|s| s.upgrade())?;
        handler.clear();

        let buf = content.as_bytes().to_vec();
        let parsed = deserialize_pipe(buf);
        let data = Tile::pipe_data(&parsed, "print");
        println!("{:?}", data.len());
        data.into_iter().for_each(|elem| {
            model.append(&elem);
        });
        Some(())
    }
}

static CONFIG: OnceLock<SherlockConfig> = OnceLock::new();

#[tokio::main]
async fn main() {
    let (application, startup_errors, non_breaking, sherlock_flags, app_config, lock) =
        startup_loading();
    let t0 = Instant::now();
    application.connect_activate(move |app| {
        let sherlock = Rc::new(RefCell::new(Sherlock::new()));
        let t1 = Instant::now();
        if let Ok(timing_enabled) = std::env::var("TIMING") {
            if timing_enabled == "true" {
                println!("Activation took {:?}", t0.elapsed());
            }
        }
        let mut error_list = startup_errors.clone();
        let mut non_breaking = non_breaking.clone();

        // Load custom icons from icon path specified in 'config.toml'
        let n = Loader::load_icon_theme();
        non_breaking.extend(n);

        // Load CSS Stylesheet
        let n = Loader::load_css()
            .map_err(|e| error_list.push(e))
            .unwrap_or_default();
        non_breaking.extend(n);

        // Main logic for the Search-View
        let (window, stack, current_stack_page, open_win) = ui::window::window(app);
        sherlock.borrow_mut().window = Some(window.downgrade());

        // Add closing logic
        app.set_accels_for_action("win.close", &["<Ctrl>W"]);

        // Significantly better id done here
        if let Some(window) = open_win.upgrade(){
            let _ = gtk4::prelude::WidgetExt::activate_action(&window, "win.open", None);
        }

        glib::MainContext::default().spawn_local({
            let sherlock = Rc::clone(&sherlock);
            async move {
                // Either show user-specified content or show normal search
                let (error_stack, error_model) = ui::error_view::errors(&error_list, &non_breaking, &current_stack_page);
                let pipe = Loader::load_pipe_args();
                let (search_stack, _handler) = if pipe.is_empty() {
                    match ui::search::search(&window, &current_stack_page, error_model.clone(), Rc::clone(&sherlock)) {
                        Ok(r) => r,
                        Err(e) => {
                            error_model.upgrade().map(|stack| stack.append(&e.tile("ERROR")));
                            return
                        }
                    }
                } else {
                    if sherlock_flags.display_raw {
                        let pipe = String::from_utf8_lossy(&pipe);
                        ui::user::display_raw(pipe, sherlock_flags.center_raw, error_model)
                    } else {
                        let parsed = deserialize_pipe(pipe);
                        if let Some(c) = CONFIG.get() {
                            let method: &str = c.pipe.method.as_deref().unwrap_or("print");
                            ui::user::display_pipe(&window, parsed, method, error_model)
                        } else {
                            return;
                        }
                    }
                };
                stack.add_named(&search_stack, Some("search-page"));

                // Notify the user about the value not having any effect to avoid confusion
                if let Some(c) = CONFIG.get() {
                    let opacity = c.appearance.opacity;

                    if !(0.1..=1.0).contains(&opacity) {
                        non_breaking.push(sherlock_error!(
                                SherlockErrorType::ConfigError(Some(format!(
                                            "The opacity value of {} exceeds the allowed range (0.1 - 1.0) and will be automatically set to {}.",
                                            opacity,
                                            opacity.clamp(0.1, 1.0)
                                ))),
                                ""
                        ));
                    }
                }

                // Logic for the Error-View
                stack.add_named(&error_stack, Some("error-page"));
                if !app_config.debug.try_suppress_errors {
                    let show_errors = !error_list.is_empty();
                    let show_warnings = !app_config.debug.try_suppress_warnings && !non_breaking.is_empty();
                    if show_errors || show_warnings {
                        let _ = gtk4::prelude::WidgetExt::activate_action(
                            &window,
                            "win.switch-page",
                            Some(&String::from("->error-page").to_variant()),
                        );
                    } else {
                        let _ = gtk4::prelude::WidgetExt::activate_action(
                            &window,
                            "win.switch-page",
                            Some(&String::from("->search-page").to_variant()),
                        );
                    }
                }
            }
        });

        // Logic for handling the daemonization
        if let Some(c) = CONFIG.get() {
            if c.behavior.daemonize {
                // Used to cache render
                if let Some(window) = open_win.upgrade() {
                    let _ = gtk4::prelude::WidgetExt::activate_action(&window, "win.open", None);
                    let _ = gtk4::prelude::WidgetExt::activate_action(&window, "win.close", None);
                }
                if let Some(win) = sherlock.borrow().window.as_ref().and_then(|win| win.upgrade()){
                    win.connect_show({
                        move |_window|{
                        }
                    });
                }

                // Create async pipeline
                let (sender, receiver) = async_channel::bounded(1);
                thread::spawn(move || {
                    async_std::task::block_on(async {
                        let _daemon = SherlockDaemon::new(sender).await;
                    });
                });

                // Handle receiving using pipline
                let open_win_clone = open_win.clone();
                MainContext::default().spawn_local({
                    let sherlock = Rc::clone(&sherlock);
                    async move {
                        while let Ok(msg) = receiver.recv().await {
                            if let Some(window) = open_win_clone.upgrade() {
                                match msg.as_str() {
                                    "show" | "open-window" => {
                                        let _ = gtk4::prelude::WidgetExt::activate_action(
                                            &window, "win.open", None,
                                        );
                                    },
                                    "obfuscate" => {
                                        sherlock.borrow().obfuscate(false);
                                    },
                                    "deobfuscate" => {
                                        sherlock.borrow().obfuscate(true);
                                    },
                                    "clear" => {
                                        window.set_visible(true);
                                        sherlock.borrow().clear_results();
                                    },
                                    "input_only" => {
                                        window.set_visible(true);
                                        sherlock.borrow().show_raw();
                                    }
                                    _ => {
                                        window.set_visible(true);
                                        sherlock.borrow().display_pipe(&msg);
                                    }
                                }
                            }
                        }
                    }
                });
            }
        }
        if let Ok(timing_enabled) = std::env::var("TIMING") {
            if timing_enabled == "true" {
                println!("Window creation took {:?}", t1.elapsed());
            }
        }
    });
    application.run();
    drop(lock);
}

#[sherlock_macro::timing("\nContent loading")]
fn startup_loading() -> (
    Application,
    Vec<SherlockError>,
    Vec<SherlockError>,
    SherlockFlags,
    SherlockConfig,
    LockFile,
) {
    let mut non_breaking: Vec<SherlockError> = Vec::new();
    let mut startup_errors: Vec<SherlockError> = Vec::new();

    // Check for '.lock'-file to only start a single instance
    let lock = lock::ensure_single_instance(LOCK_FILE).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });

    // Setup flags
    let sherlock_flags = Loader::load_flags()
        .map_err(|e| startup_errors.push(e))
        .unwrap_or_default();

    // Parse configs from 'config.toml'
    let app_config = SherlockConfig::from_flags(&sherlock_flags).map_or_else(
        |e| {
            startup_errors.push(e);
            let defaults = utils::config::SherlockConfig::default();
            SherlockConfig::apply_flags(&sherlock_flags, defaults)
        },
        |(app_config, n)| {
            non_breaking.extend(n);
            app_config
        },
    );

    CONFIG
        .set(app_config.clone())
        .map_err(|_| {
            startup_errors.push(sherlock_error!(SherlockErrorType::ConfigError(None), ""));
        })
        .ok();

    // Load resources
    Loader::load_resources()
        .map_err(|e| startup_errors.push(e))
        .ok();

    // Initialize application
    let application = Application::builder()
        .application_id("dev.skxxtz.sherlock")
        .flags(gio::ApplicationFlags::NON_UNIQUE | gio::ApplicationFlags::HANDLES_COMMAND_LINE)
        .build();

    if let Some(config) = CONFIG.get() {
        env::set_var("GSK_RENDERER", &config.appearance.gsk_renderer);
    }

    // Needed in order start Sherlock without glib flag handling
    application.connect_command_line(|app, _| {
        app.activate();
        0
    });

    (
        application,
        startup_errors,
        non_breaking,
        sherlock_flags,
        app_config,
        lock,
    )
}
