use gio::prelude::*;
use gtk4::{prelude::*, Application};
use std::{env, process};


mod launcher;
mod ui;
mod actions;
mod loader;
mod lock;

use loader::{util::SherlockError, Loader};



#[tokio::main]
async fn main() {
    let mut startup_errors: Vec<SherlockError> = Vec::new();

    // Check for file lock to only start a single instance
    let lock_file = "/tmp/sherlock.lock";
    let _ = match lock::ensure_single_instance(lock_file) {
        Ok(lock) => lock,
        Err(msg) => {
            eprintln!("{}", msg);
            process::exit(1);
        }
    };

    // Parse configs
    let app_config = Loader::load_config()
        .map_err(|e| startup_errors.push(e))
        .unwrap_or_default();

    let sherlock_flags = Loader::load_flags()
        .map_err(|e| startup_errors.push(e))
        .unwrap_or_default();

    Loader::load_resources();


    // Initialize application
    let application = Application::new(Some("dev.skxxtz.sherlock"), Default::default());
    env::set_var("GSK_RENDERER", &app_config.appearance.gsk_renderer);

    application.connect_activate(move |app| {
        let mut error_list: Vec<SherlockError> = startup_errors.clone();

        let (launchers, launcher_errors) = Loader::load_launchers(&sherlock_flags, &app_config);
        error_list.extend(launcher_errors);
        
        Loader::load_icon_theme(&app_config.appearance.icon_paths)
            .map_err(|e| error_list.push(e))
            .ok();

        Loader::load_css(&sherlock_flags)
            .map_err(|e| error_list.push(e))
            .ok();
        


        let (mut window, stack) = ui::window::window(&app);
        window = ui::search::search(window, &stack, launchers, app_config.clone());
        if !error_list.is_empty(){
            window = ui::error_view::errors(window, &stack, &error_list);
            stack.set_visible_child_name("error-page");
        } 
        window.show();
    });

    application.run();
}





