use gio::prelude::*;
use gtk4::{prelude::*, Application};
use std::{env, process};

mod actions;
mod launcher;
mod loader;
mod lock;
mod ui;

use loader::{util::SherlockError, Loader};

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

    let _ = Loader::load_resources().map_err(|e| startup_errors.push(e));

    // Initialize application
    let application = Application::new(
        Some("dev.skxxtz.sherlock"),
        gio::ApplicationFlags::HANDLES_COMMAND_LINE,
    );
    env::set_var("GSK_RENDERER", &app_config.appearance.gsk_renderer);

    // Needed in order start Sherlock without glib flag handling
    application.connect_command_line(|app, _| {
        app.activate();
        0
    });

    application.connect_activate(move |app| {
        let mut error_list = startup_errors.clone();
        let mut non_breaking = non_breaking.clone();

        // Initialize launchers from 'fallback.json'
        let (launchers, n) = Loader::load_launchers(&sherlock_flags, &app_config)
            .map_err(|e| error_list.push(e))
            .unwrap_or_default();
        non_breaking.extend(n);

        // Load custom icons from icon path specified in 'config.toml'
        let n = Loader::load_icon_theme(&app_config.appearance.icon_paths);
        non_breaking.extend(n);

        // Load CSS Stylesheet
        let n = Loader::load_css(&sherlock_flags)
            .map_err(|e| error_list.push(e))
            .unwrap_or_default();
        non_breaking.extend(n);


        // Main logic for the Search-View
        let (mut window, stack) = ui::window::window(&app);
        window = ui::search::search(window, &stack, launchers, app_config.clone());

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
