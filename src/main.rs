// CRATES
use gio::prelude::*;
use gtk4::prelude::GtkApplicationExt;
use gtk4::Application;
use loader::pipe_loader::deserialize_pipe;
use loader::util::{SherlockErrorType, SherlockFlags};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::OnceLock;
use std::{env, process, thread};

// MODS
mod actions;
mod application;
mod daemon;
mod g_subclasses;
mod launcher;
mod loader;
mod ui;

// IMPORTS
use application::lock;
use application::util::AppState;
use daemon::daemon::SherlockDaemon;
use loader::{
    util::{SherlockConfig, SherlockError},
    Loader,
};
use ui::util::show_stack_page;

const SOCKET_PATH: &str = "/tmp/sherlock_daemon.socket";
const LOCK_FILE: &str = "/tmp/sherlock.lock";

thread_local! {
    static APP_STATE: RefCell<Option<Rc<AppState>>> = RefCell::new(None);
}
static CONFIG: OnceLock<SherlockConfig> = OnceLock::new();
static FLAGS: OnceLock<SherlockFlags> = OnceLock::new();

#[tokio::main]
async fn main() {
    let mut non_breaking: Vec<SherlockError> = Vec::new();
    let mut startup_errors: Vec<SherlockError> = Vec::new();

    // Check for '.lock'-file to only start a single instance
    let _lock = lock::ensure_single_instance(LOCK_FILE).unwrap_or_else(|e| {
        eprintln!("{}", e);
        process::exit(1);
    });

    // Setup flags
    let sherlock_flags = Loader::load_flags()
        .map_err(|e| startup_errors.push(e))
        .unwrap_or_default();
    FLAGS
        .set(sherlock_flags.clone())
        .map_err(|_| {
            startup_errors.push(SherlockError {
                error: SherlockErrorType::ConfigError(None),
                traceback: format!("should never get to this"),
            });
        })
        .ok();

    // Parse configs from 'config.toml'
    let app_config = Loader::load_config().map_or_else(
        |e| {
            startup_errors.push(e);
            let defaults = loader::util::SherlockConfig::default();
            loader::Loader::apply_flags(&sherlock_flags, defaults)
        },
        |(app_config, n)| {
            non_breaking.extend(n);
            app_config
        },
    );

    CONFIG
        .set(app_config.clone())
        .map_err(|_| {
            startup_errors.push(SherlockError {
                error: SherlockErrorType::ConfigError(None),
                traceback: format!(""),
            });
        })
        .ok();

    Loader::load_resources()
        .map_err(|e| startup_errors.push(e))
        .ok();

    // Initialize launchers from 'fallback.json'
    let launcher_get = Loader::load_launchers();

    // Initialize application
    let application = Application::new(
        Some("dev.skxxtz.sherlock"),
        gio::ApplicationFlags::HANDLES_COMMAND_LINE,
    );

    if let Some(config) = CONFIG.get() {
        env::set_var("GSK_RENDERER", &config.appearance.gsk_renderer);
    }

    // Needed in order start Sherlock without glib flag handling
    application.connect_command_line(|app, _| {
        app.activate();
        0
    });

    // Await getters here
    let (launchers, n) = launcher_get
        .await
        .map_err(|e| startup_errors.push(e))
        .unwrap_or_default();
    non_breaking.extend(n);

    application.connect_activate(move |app| {
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
        let (window, stack) = ui::window::window(&app);

        // Add closing logic
        app.set_accels_for_action("win.close", &["<Ctrl>W", "Escape"]);

        // creating app state
        let state = Rc::new(AppState {
            window: Some(window),
            stack: Some(stack),
            search_bar: None,
        });

        APP_STATE.with(|app_state| *app_state.borrow_mut() = Some(state));

        // Either show user-specified content or show normal search
        let pipe = Loader::load_pipe_args();
        if pipe.is_empty() {
            ui::search::search(&launchers);
        } else {
            if sherlock_flags.display_raw {
                let pipe = String::from_utf8_lossy(&pipe);
                ui::user::display_raw(pipe, sherlock_flags.center_raw);
            } else {
                let parsed = deserialize_pipe(pipe);
                if let Some(c) = CONFIG.get() {
                    let method: &str = c.pipe.method.as_deref().unwrap_or("print");
                    ui::user::display_pipe(parsed, method)
                }
            }
        };

        // Logic for the Error-View
        if !app_config.debug.try_suppress_errors {
            let show_errors = !error_list.is_empty();
            let show_warnings = !app_config.debug.try_suppress_warnings && !non_breaking.is_empty();
            if show_errors || show_warnings {
                ui::error_view::errors(&error_list, &non_breaking);
                show_stack_page("error-page", None);
            }
        }

        // Logic for handling the daemonization
        if let Some(c) = CONFIG.get() {
            match c.behavior.daemonize {
                true => {
                    // Used to cache render
                    ui::window::show_window(false);
                    ui::window::hide_window(false);

                    thread::spawn(move || {
                        let _damon = SherlockDaemon::new(SOCKET_PATH);
                    });
                }
                false => {
                    // Show window without daemonizing
                    ui::window::show_window(false);
                }
            }
        }
    });
    application.run();
}
