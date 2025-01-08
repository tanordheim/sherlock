use gio::prelude::*;
use gtk4::gdk;
use gtk4::{prelude::*, Application, ApplicationWindow, EventControllerKey};
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use once_cell::sync::Lazy;

mod components;
mod helpers;
use helpers::config_loader::{read_config, Config};

static CONFIG: Lazy<Config> = Lazy::new(read_config);


fn activate(application: &Application) {
    // Initialize Layer
    let mut window = ApplicationWindow::new(application);
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);

    window.set_default_size(900, 589);
    window.set_resizable(false);

    helpers::load_css();

    let anchors = [
        (Edge::Left, false),
        (Edge::Right, false),
        (Edge::Top, false),
        (Edge::Bottom, false),
    ];
    for (anchor, state) in anchors {
        window.set_anchor(anchor, state);
    }

    // Important to let keyevents pass through
    let event_controller = EventControllerKey::new();
    window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::Exclusive);
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

    window = components::views::search(window);
    window.present();
}

fn main() {
    helpers::load_resources();
    let application = Application::new(Some("com.skxxtz.sherlock"), Default::default());

    application.connect_activate(move |app| {
        activate(app);
    });

    application.run();
}
