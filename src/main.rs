use gio::prelude::*;
use gtk4::{prelude::*, Application};
use loader::flag_loader::print_help;
use std::collections::HashSet;
use std::sync::OnceLock;
use std::{env, process};
use std::io::{self, Read};

mod actions;
mod launcher;
mod loader;
mod lock;
mod ui;


use loader::{
    util::{Config, SherlockError},
    Loader,
};

use std::os::unix::io::AsRawFd;
use nix::fcntl::{fcntl, FcntlArg, OFlag};


fn pipe_content() -> String {
    let mut stdin = io::stdin();
    let mut buffer = String::new();

    let fd = stdin.lock().as_raw_fd();  

    match fcntl(fd, FcntlArg::F_GETFL) {
        Ok(flags) => {
            let flags = OFlag::from_bits_truncate(flags);
            if let Ok(_) = fcntl(fd, FcntlArg::F_SETFL(OFlag::O_NONBLOCK | flags)) {

                let _  = stdin.read_to_string(&mut buffer);
                
                return buffer;
            }
        }
        Err(_) => {}
    }
    String::new()
}

static CONFIG: OnceLock<Config> = OnceLock::new();

#[tokio::main]
async fn main() {
    let mut startup_errors: Vec<SherlockError> = Vec::new();
    let mut non_breaking: Vec<SherlockError> = Vec::new();


    // Check for '.lock'-file to only start a single instance
    let lock_file = "/tmp/sherlock.lock";
    let _ = match lock::ensure_single_instance(lock_file) {
        Ok(lock) => lock,
        Err(msg) => {
            eprintln!("{}", msg);
            process::exit(1);
        }
    };

    let sherlock_flags = Loader::load_flags()
        .map_err(|e| startup_errors.push(e))
        .unwrap_or_default();

    // Parse configs from 'config.toml'
    let (app_config, n) = Loader::load_config(&sherlock_flags)
        .map_err(|e| startup_errors.push(e))
        .unwrap_or(loader::util::Config::default());
    non_breaking.extend(n);

    match CONFIG.set(app_config.clone()) {
        Ok(_) => {}
        Err(_) => {
            startup_errors.push(SherlockError {
                name: format!("Missing Config"),
                message: format!("It should never come to this."),
                traceback: format!(""),
            });
        }
    };


    let _ = Loader::load_resources().map_err(|e| startup_errors.push(e));


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

    application.connect_activate(move |app| {
        let mut error_list = startup_errors.clone();
        let mut non_breaking = non_breaking.clone();

        // Initialize launchers from 'fallback.json'
        let (launchers, n) = Loader::load_launchers(&sherlock_flags)
            .map_err(|e| error_list.push(e))
            .unwrap_or_default();
        non_breaking.extend(n);

        // Load custom icons from icon path specified in 'config.toml'
        let n = Loader::load_icon_theme();
        non_breaking.extend(n);

        // Load CSS Stylesheet
        let n = Loader::load_css(&sherlock_flags)
            .map_err(|e| error_list.push(e))
            .unwrap_or_default();
        non_breaking.extend(n);

        // Main logic for the Search-View
        let (mut window, stack) = ui::window::window(&app);

        // Either show user-specified content or show normal search
        window = {
            let pipe = pipe_content();
            if pipe.is_empty(){
                ui::search::search(window, &stack, launchers)
            } else {
                let lines:Vec<String> = pipe.split("\n").filter(|s| !s.is_empty()).map(|s| s.to_string()).collect();
                ui::user::display_pipe(window, &stack, lines)
            }
        };

        // Logic for the Error-View
        if !app_config.debug.try_surpress_errors {
            if !app_config.debug.try_surpress_warnings {
                if !error_list.is_empty() || !non_breaking.is_empty() {
                    window = ui::error_view::errors(window, &stack, &error_list, &non_breaking);
                    stack.set_visible_child_name("error-page");
                }
            } else {
                if !error_list.is_empty() {
                    window = ui::error_view::errors(window, &stack, &error_list, &non_breaking);
                    stack.set_visible_child_name("error-page");
                }
            }
        }
        window.present();
    });

    application.run();
}

trait SherlockSearch {
    fn fuzzy_match<T>(&self, substring: T)->bool
    where
        Self: AsRef<str>,
        T: AsRef<str>
    {
        let char_pattern: HashSet<char> = substring.as_ref().chars().collect();
        let concat_str: String = self.as_ref().chars().filter(|s| char_pattern.contains(s)).collect();
        concat_str.contains(substring.as_ref())
    }
}
impl SherlockSearch for String{}
