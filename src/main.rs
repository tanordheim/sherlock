use gio::prelude::*;
use gtk4::gdk;
use gtk4::{prelude::*, Application, ApplicationWindow, EventControllerKey};
use gtk4_layer_shell::{Layer, LayerShell};
use once_cell::sync::Lazy;

mod components;
mod helpers;
use helpers::config_loader::{read_config, Config};

static CONFIG: Lazy<Config> = Lazy::new(read_config);



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
    return window

}

fn main() {
    helpers::load_resources();
    let application = Application::new(Some("com.skxxtz.sherlock"), Default::default());

    application.connect_activate(move |app| {
        let mut window = create_main_window(&app);
        window.show();

        window = components::views::search(window);
        helpers::load_css();
    });

    application.run();
}
