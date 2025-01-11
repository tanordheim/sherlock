use gio::prelude::*;
use gtk4::Settings;
use gtk4::gdk;
use gtk4::{prelude::*, Application, ApplicationWindow, EventControllerKey };
use gtk4_layer_shell::{Layer, LayerShell};
use once_cell::sync::Lazy;
use std::env;

mod launcher;
mod ui;
mod actions;
mod loader;

use loader::{Loader, util::Config};


static CONFIG: Lazy<Config> = Lazy::new(|| {
    Loader::load_config()
});


fn create_main_window(application: &Application)-> ApplicationWindow{
    let window:ApplicationWindow = ApplicationWindow::builder()
        .application(application)
        .default_width(900)
        .default_height(589)
        .resizable(false)
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::Exclusive);


    let event_controller = EventControllerKey::new();
    event_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);
    event_controller.connect_key_pressed(move |_, key, _, _| {
        match key {
            gdk::Key::Escape => {
                std::process::exit(0);
            },
            _ => (),
        }
        false.into()
    });
    window.add_controller(event_controller);
    if let Some(settings) = Settings::default(){
        settings.set_gtk_application_prefer_dark_theme(true);
    }
    return window

}

#[tokio::main]
async fn main() {
    Loader::load_resources();


    env::set_var("GSK_RENDERER", "cairo");
    let application = Application::new(Some("com.skxxtz.sherlock"), Default::default());

    application.connect_activate(|app| {
        let launchers = Loader::load_launchers();
        Loader::load_icon_theme(&CONFIG.appearance.icon_paths);
        Loader::load_css();

        // Move the async block to GTK's main thread
        let app_clone = app.clone();
        let mut window = create_main_window(&app_clone);
        window = ui::search::search(window, launchers);
        window.show();
    });

    application.run();
}

