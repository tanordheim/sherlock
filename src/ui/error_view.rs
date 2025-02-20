use gtk4::{self, gdk::Key, prelude::*, ApplicationWindow, Builder, EventControllerKey, Stack};
use gtk4::{Box as HVBox, ListBox, ScrolledWindow};
use std::cell::RefCell;
use std::rc::Rc;

use super::util::*;

use crate::{loader::util::SherlockError, ui::tiles::Tile};

pub fn errors(
    window: ApplicationWindow,
    stack: &Stack,
    errors: &Vec<SherlockError>,
    non_breaking: &Vec<SherlockError>,
) -> ApplicationWindow {
    let builder = Builder::from_resource("/dev/skxxtz/sherlock/ui/error_view.ui");

    let vbox: HVBox = builder.object("vbox").unwrap();
    let result_viewport: ScrolledWindow = builder.object("scrolled-window").unwrap();
    let results: ListBox = builder.object("result-frame").unwrap();

    let (_, breaking_error_tiles) = Tile::error_tile(0, errors, "🚨", "ERROR");
    let (_, error_tiles) = Tile::error_tile(0, non_breaking, "⚠️", "WARNING");

    breaking_error_tiles
        .iter()
        .for_each(|tile| results.append(tile));
    error_tiles.iter().for_each(|tile| results.append(tile));

    stack.add_named(&vbox, Some("error-page"));
    nav_event(&window, stack.clone(), results, result_viewport);

    window
}
fn nav_event(
    window: &ApplicationWindow,
    stack: Stack,
    result_holder: ListBox,
    result_viewport: ScrolledWindow,
) {
    // Wrap the event controller in an Rc<RefCell> for shared mutability
    let event_controller = Rc::new(RefCell::new(EventControllerKey::new()));

    // Clone Rc references for use in the closure
    let event_controller_clone = Rc::clone(&event_controller);
    let window_clone = window.clone();

    event_controller
        .borrow_mut()
        .set_propagation_phase(gtk4::PropagationPhase::Capture);
    event_controller
        .borrow_mut()
        .connect_key_pressed(move |_, key, _, _| {
            match key {
                Key::Up => {
                    result_holder.focus_prev(&result_viewport);
                    true.into()
                }
                Key::Down => {
                    result_holder.focus_next(&result_viewport);
                    true.into()
                }
                Key::Return => {
                    stack.set_transition_type(gtk4::StackTransitionType::SlideLeft);
                    stack.set_visible_child_name("search-page");

                    // Remove the event controller
                    if let Some(controller) = event_controller_clone
                        .borrow()
                        .clone()
                        .downcast_ref::<EventControllerKey>()
                    {
                        window_clone.remove_controller(controller);
                    }

                    true.into()
                }
                _ => false.into(),
            }
        });

    window.add_controller(event_controller.borrow().clone());
}
