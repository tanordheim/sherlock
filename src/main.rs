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
    let mut non_breaking: Vec<SherlockError> = Vec::new();

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
    let (app_config, n) = Loader::load_config()
        .map_err(|e| startup_errors.push(e))
        .unwrap_or(loader::util::Config::default());
    non_breaking.extend(n);


    let sherlock_flags = Loader::load_flags()
        .map_err(|e| startup_errors.push(e))
        .unwrap_or_default();

    let _ = Loader::load_resources()
        .map_err(|e| startup_errors.push(e));


    // Initialize application
    let application = Application::new(Some("dev.skxxtz.sherlock"), Default::default());
    env::set_var("GSK_RENDERER", &app_config.appearance.gsk_renderer);

    application.connect_activate(move |app| {
        let mut error_list = startup_errors.clone();
        let mut non_breaking = non_breaking.clone();

        let (launchers, n) = Loader::load_launchers(&sherlock_flags, &app_config)
            .map_err(|e| error_list.push(e))
            .unwrap_or_default();
        non_breaking.extend(n);
        
        let n = Loader::load_icon_theme(&app_config.appearance.icon_paths);
        non_breaking.extend(n);

        Loader::load_css(&sherlock_flags)
            .map_err(|e| error_list.push(e))
            .ok();
        

        
        let (mut window, stack) = ui::window::window(&app);
        window = ui::search::search(window, &stack, launchers, app_config.clone());
        
    
        if !app_config.debug.try_surpress_errors{
            if !app_config.debug.try_surpress_warnings {
                if !error_list.is_empty() || !non_breaking.is_empty(){
                    println!("{:?}{:?}", error_list, non_breaking);
                    window = ui::error_view::errors(window, &stack, &error_list, &non_breaking);
                    stack.set_visible_child_name("error-page");
                }
            } else {
                if !error_list.is_empty() {
                    println!("{:?}{:?}", error_list, non_breaking);
                    window = ui::error_view::errors(window, &stack, &error_list, &non_breaking);
                    stack.set_visible_child_name("error-page");
                }
            }
        }
        window.present();
    });

    application.run();
}





