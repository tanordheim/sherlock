use gio::prelude::*;
use gtk4::{prelude::*, Application, StackTransitionType};
use std::{env, process};
use gtk4::glib::{self, ControlFlow};


mod launcher;
mod ui;
mod actions;
mod loader;
mod lock;

use loader::{util::{Config, SherlockError}, Loader};




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
        println!("{:?}", i);
    }
    // Initialize application
    let application = Application::new(Some("dev.skxxtz.sherlock"), Default::default());
    env::set_var("GSK_RENDERER", &app_config.appearance.gsk_renderer);

    application.connect_activate(move |app| {
        let mut error_list: Vec<SherlockError> = startup_errors.clone();

        let launchers = match Loader::load_launchers(&sherlock_flags, &app_config){
            Ok(value)=> value,
            Err(e) => {
                error_list.push(e);
                Default::default()
            }
        };
        Loader::load_icon_theme(&app_config.appearance.icon_paths);
        Loader::load_css(&sherlock_flags);
        


        let app_clone = app.clone();
        let (mut window, stack) = ui::window::window(&app_clone);
        if !error_list.is_empty(){
            ui::error_view::errors(&window, &error_list);
        };
        window = ui::search::search(window, &stack, launchers, app_config.clone());
        
        glib::timeout_add_seconds_local(2, move || {
            stack.set_transition_type(StackTransitionType::SlideLeftRight);
            stack.set_visible_child_name("search-stack");
            ControlFlow::Continue
        });
        window.show();
    });

    application.run();
}



