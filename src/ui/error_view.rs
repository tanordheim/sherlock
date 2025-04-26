use gio::glib::WeakRef;
use gtk4::{self, gdk::Key, prelude::*, Builder, EventControllerKey};
use gtk4::{Box as HVBox, ListBox, ScrolledWindow};
use std::cell::RefCell;
use std::rc::Rc;

use super::util::*;

use crate::ui::tiles::Tile;
use crate::utils::errors::SherlockError;

pub fn errors(
    errors: &Vec<SherlockError>,
    non_breaking: &Vec<SherlockError>,
    stack_page: &Rc<RefCell<String>>,
) -> HVBox {
    let builder = Builder::from_resource("/dev/skxxtz/sherlock/ui/error_view.ui");

    let vbox: HVBox = builder.object("vbox").unwrap();
    let result_viewport: ScrolledWindow = builder.object("scrolled-window").unwrap();
    let results: ListBox = builder.object("result-frame").unwrap();

    let (_, breaking_error_tiles) = Tile::error_tile(0, errors, "🚨", "ERROR");
    let (_, error_tiles) = Tile::error_tile(0, non_breaking, "⚠️", "WARNING");

    breaking_error_tiles
        .into_iter()
        .for_each(|tile| results.append(&tile));
    error_tiles
        .into_iter()
        .for_each(|tile| results.append(&tile));

    // nav_event(&vbox, results, result_viewport.downgrade(), stack_page);
    return vbox;
}
// fn nav_event(
//     stack: &HVBox,
//     result_holder: ListBox,
//     result_viewport: WeakRef<ScrolledWindow>,
//     stack_page: &Rc<RefCell<String>>,
// ) {
//     // Wrap the event controller in an Rc<RefCell> for shared mutability
//     let event_controller = EventControllerKey::new();
//     let stack_page = Rc::clone(&stack_page);

//     event_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);
//     event_controller.connect_key_pressed(move |_, key, _, _| {
//         if stack_page.borrow().as_str() != "error-page" {
//             return false.into();
//         }
//         match key {
//             Key::Up => {
//                 result_holder.focus_prev(&result_viewport);
//                 true.into()
//             }
//             Key::Down => {
//                 result_holder.focus_next(&result_viewport);
//                 true.into()
//             }
//             Key::Return => {
//                 let _ = result_holder.activate_action(
//                     "win.switch-page",
//                     Some(&String::from("search-page").to_variant()),
//                 );
//                 true.into()
//             }
//             _ => false.into(),
//         }
//     });
//     stack.set_can_focus(true);
//     stack.grab_focus();
//     stack.add_controller(event_controller);
// }
