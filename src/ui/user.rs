use gtk4::{
    self,
    gdk::{self, Key},
    prelude::*,
    ApplicationWindow, Builder, EventControllerKey, Stack,
};
use gtk4::{Box as HVBox, Entry, ListBox, ScrolledWindow};
use std::collections::HashMap;
use std::rc::Rc;

use super::util::*;
use crate::actions::execute_from_attrs;
use super::tiles::Tile;


pub fn display_pipe(window: ApplicationWindow, search_stack: &Stack, pipe_content: Vec<String> )->ApplicationWindow{

    // Initialize the builder with the correct path
    let builder = Builder::from_resource("/dev/skxxtz/sherlock/ui/search.ui");

    // Get the requred object references
    let vbox: HVBox = builder.object("vbox").unwrap();
    let search_bar: Entry = builder.object("search-bar").unwrap();
    let result_viewport: ScrolledWindow = builder.object("scrolled-window").unwrap();
    let results: Rc<ListBox> = Rc::new(builder.object("result-frame").unwrap());

    let keyword = search_bar.text();

    let (_, tiles) = Tile::simple_text_tile(0, &pipe_content, "", &keyword);
    for item in tiles{
        results.append(&item);
    }

    result_viewport.set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic);
    results.focus_first();
    search_bar.grab_focus();

    change_event(&search_bar, &results, pipe_content);

    nav_event(
        &window,
        results,
        result_viewport,
    );


    search_stack.add_named(&vbox, Some("search-page"));
    return window;
}



fn nav_event(
    window: &ApplicationWindow,
    results_ev_nav: Rc<ListBox>,
    result_viewport: ScrolledWindow,
) {
    let event_controller = EventControllerKey::new();
    event_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);
    event_controller.connect_key_pressed(move |_, key, _, modifiers| {
        match key {
            gdk::Key::Up => {
                results_ev_nav.focus_prev(&result_viewport);
            }
            gdk::Key::Down => {
                results_ev_nav.focus_next(&result_viewport);
                return true.into();
            }
            gdk::Key::Return => {
                if let Some(row) = results_ev_nav.selected_row() {
                    let attrs: HashMap<String, String> = get_row_attrs(row);
                    execute_from_attrs(attrs);
                }
            }
            Key::_1 | Key::_2 | Key::_3 | Key::_4 | Key::_5 => {
                if modifiers.contains(gdk::ModifierType::CONTROL_MASK) {
                    let key_index = match key {
                        Key::_1 => 1,
                        Key::_2 => 2,
                        Key::_3 => 3,
                        Key::_4 => 4,
                        Key::_5 => 5,
                        _ => return false.into(),
                    };
                    execute_by_index(&*results_ev_nav, key_index);
                }
            }
            _ => (),
        }
        false.into()
    });
    window.add_controller(event_controller);
}

fn change_event(
    search_bar: &Entry,
    results: &Rc<ListBox>,
    pipe_content: Vec<String>,
) {
    //Cloning:
    let results_ev_changed = Rc::clone(results);
    let pipe_content_clone = pipe_content.clone();


    search_bar.connect_changed(move |search_bar| {
        let current_text = search_bar.text();

        while let Some(row) = results_ev_changed.last_child() {
            results_ev_changed.remove(&row);
        }
        let (_, tiles) = Tile::simple_text_tile(0, &pipe_content_clone, "", &current_text);
        for item in tiles{
            results_ev_changed.append(&item);
        }

        results_ev_changed.focus_first();


    });
}

