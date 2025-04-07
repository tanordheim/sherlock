use std::fs;

use gtk4::{gdk, Builder, Stack};
use gtk4::{prelude::*, Application, ApplicationWindow, EventControllerKey};
use gtk4_layer_shell::{Layer, LayerShell};

use crate::actions::util::eval_exit;
use crate::application::util::reload_content;
use crate::{APP_STATE, CONFIG, LOCK_FILE};

pub fn window(application: &Application) -> (ApplicationWindow, Stack) {
    // 618 with, 591 without notification bar
    let (width, height) = CONFIG.get().map_or_else(
        || (900, 591),
        |config| (config.appearance.width, config.appearance.height),
    );

    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(application)
        .default_width(width)
        .default_height(height)
        .resizable(false)
        .build();

    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::Exclusive);

    //Build main fame here that holds logic for stacking
    let builder = Builder::from_resource("/dev/skxxtz/sherlock/ui/window.ui");
    let holder: Stack = builder.object("stack").unwrap();

    let event_controller = EventControllerKey::new();
    event_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);
    if let Some(c) = CONFIG.get() {
        let action = match c.behavior.daemonize {
            true => hide_app,
            false => eval_exit,
        };
        event_controller.connect_key_pressed(move |_, key, _, _| {
            match key {
                gdk::Key::Escape => action(),
                _ => (),
            }
            false.into()
        });
    }
    window.add_controller(event_controller);
    window.set_child(Some(&holder));
    return (window, holder);
}
fn hide_app() {
    hide_window(true);
}

pub fn show_window(reload: bool) {
    if reload {
        reload_content();
    };
    APP_STATE.with(|state| {
        if let Some(ref state) = *state.borrow() {
            state.window.as_ref().map(|window| window.present());
        }
    });
}
pub fn hide_window(clear_search: bool) {
    APP_STATE.with(|state| {
        if let Some(ref state) = *state.borrow() {
            state.window.as_ref().map(|window| window.hide());
            if clear_search {
                state
                    .search_bar
                    .as_ref()
                    .map(|search_bar| search_bar.set_text(""));
            } else {
                let _ = fs::remove_file(LOCK_FILE);
            }
        }
    });
}

pub fn destroy_window() {
    APP_STATE.with(|state| {
        if let Some(ref state) = *state.borrow() {
            state.window.as_ref().map(|window| window.destroy());
        } else {
            std::process::exit(0)
        }
    });
}
