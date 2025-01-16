use gio::prelude::*;
use gtk4::{prelude::*, Application};
use std::{env, process};


mod launcher;
mod ui;
mod actions;
mod loader;
mod lock;

use loader::{util::Config, Loader};




#[tokio::main]
async fn main() {
    let mut startup_errors: Vec<String> = Vec::new();

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
    let app_config: Config = match Loader::load_config() {
        Ok(config) => config,
        Err(e) => {
            startup_errors.push(e);
            Config::default()
        }
    };


    let sherlock_flags = Loader::load_flags();
    Loader::load_resources();


    for i in startup_errors.iter(){
        println!("{i}");
    }
    // Initialize application
    let application = Application::new(Some("dev.skxxtz.sherlock"), Default::default());
    env::set_var("GSK_RENDERER", &app_config.appearance.gsk_renderer);

    application.connect_activate(move |app| {
        let mut runtime_errors: Vec<String> = Vec::new();

        let launchers = match Loader::load_launchers(&sherlock_flags, &app_config){
            Ok(value)=> value,
            Err(e) => {
                runtime_errors.push(e);
                Default::default()
            }
        };

        Loader::load_icon_theme(&app_config.appearance.icon_paths);
        Loader::load_css(&sherlock_flags);


        let app_clone = app.clone();
        let mut window = ui::window::window(&app_clone);
        window = ui::search::search(window, launchers, app_config.clone());
        window.show();
    });

    application.run();
}



