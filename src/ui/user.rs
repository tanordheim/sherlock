use gtk4::{
    self,
    gdk::{self, Key, ModifierType},
    prelude::*,
    ApplicationWindow, Builder, Entry, EventControllerKey, Justification,
};

use gtk4::{Box as HVBox, ListBox, ScrolledWindow};
use std::rc::Rc;

use super::tiles::{util::TextViewTileBuilder, Tile};
use super::util::*;
use crate::g_subclasses::sherlock_row::SherlockRow;
use crate::loader::pipe_loader::PipeData;

pub fn display_pipe(
    _window: &ApplicationWindow,
    pipe_content: Vec<PipeData>,
    method: &str,
) -> HVBox {
    // Initialize the builder with the correct path
    let builder = Builder::from_resource("/dev/skxxtz/sherlock/ui/search.ui");

    // Get the requred object references
    let stack_page: HVBox = builder.object("vbox").unwrap();
    let search_bar: Entry = builder.object("search-bar").unwrap_or_default();
    let result_viewport: ScrolledWindow = builder.object("scrolled-window").unwrap();
    let results: Rc<ListBox> = Rc::new(builder.object("result-frame").unwrap());

    let keyword = search_bar.text();

    let tiles = Tile::pipe_data(&pipe_content, &method, &keyword);
    for item in tiles {
        results.append(&item);
    }

    result_viewport.set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic);
    results.focus_first();

    let search_bar_clone = search_bar.clone();
    stack_page.connect_realize(move |_| {
        search_bar_clone.grab_focus();
    });

    let custom_binds = ConfKeys::new();

    change_event(&search_bar, &results, pipe_content, &method);

    nav_event(
        &stack_page,
        results,
        search_bar,
        result_viewport,
        custom_binds,
    );
    return stack_page;
}
pub fn display_raw<T: AsRef<str>>(content: T, center: bool) -> HVBox {
    let builder = TextViewTileBuilder::new("/dev/skxxtz/sherlock/ui/text_view_tile.ui");
    let buffer = builder.content.buffer();
    builder.content.add_css_class("raw_text");
    builder.content.set_monospace(true);
    let sanitized: String = content.as_ref().chars().filter(|&c| c != '\0').collect();
    buffer.set_text(&sanitized);
    if center {
        builder.content.set_justification(Justification::Center);
    }
    return builder.object;
}

fn nav_event(
    stack: &HVBox,
    results: Rc<ListBox>,
    search_bar: Entry,
    result_viewport: ScrolledWindow,
    custom_binds: ConfKeys,
) {
    let event_controller = EventControllerKey::new();
    event_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);
    event_controller.connect_key_pressed(move |_, key, i, modifiers| {
        match key {
            k if Some(k) == custom_binds.prev
                && custom_binds
                    .prev_mod
                    .map_or(true, |m| modifiers.contains(m)) =>
            {
                results.focus_prev(&result_viewport);
                return true.into();
            }
            k if Some(k) == custom_binds.next
                && custom_binds
                    .next_mod
                    .map_or(true, |m| modifiers.contains(m)) =>
            {
                results.focus_next(&result_viewport);
                return true.into();
            }
            gdk::Key::Up => {
                results.focus_prev(&result_viewport);
            }
            gdk::Key::Down => {
                results.focus_next(&result_viewport);
                return true.into();
            }
            gdk::Key::BackSpace => {
                let mut ctext = search_bar.text().to_string();
                if custom_binds
                    .shortcut_modifier
                    .map_or(false, |modifier| modifiers.contains(modifier))
                {
                    let _ = search_bar.set_text("");
                    ctext.clear();
                }
                if ctext.is_empty() {
                    let _ =
                        search_bar.activate_action("win.switch-mode", Some(&"all".to_variant()));
                }
                results.focus_first();
            }
            gdk::Key::Return => {
                if let Some(row) = results.selected_row().and_downcast_ref::<SherlockRow>() {
                    row.emit_by_name::<()>("row-should-activate", &[]);
                }
            }
            Key::_1 | Key::_2 | Key::_3 | Key::_4 | Key::_5 => {
                if custom_binds
                    .shortcut_modifier
                    .map_or(false, |modifier| modifiers.contains(modifier))
                {
                    let key_index = match key {
                        Key::_1 => 1,
                        Key::_2 => 2,
                        Key::_3 => 3,
                        Key::_4 => 4,
                        Key::_5 => 5,
                        _ => return false.into(),
                    };
                    execute_by_index(&*results, key_index);
                    return true.into();
                }
            }
            // Pain - solution for shift-tab since gtk handles it as an individual event
            _ if i == 23 && modifiers.contains(ModifierType::SHIFT_MASK) => {
                let shift = Some(ModifierType::SHIFT_MASK);
                let tab = Some(Key::Tab);
                if custom_binds.prev_mod == shift && custom_binds.prev == tab {
                    results.focus_prev(&result_viewport);
                    return true.into();
                } else if custom_binds.next_mod == shift && custom_binds.next == tab {
                    results.focus_next(&result_viewport);
                    return true.into();
                }
            }
            _ => (),
        }
        false.into()
    });

    stack.add_controller(event_controller);
}

fn change_event(
    search_bar: &Entry,
    results: &Rc<ListBox>,
    pipe_content: Vec<PipeData>,
    method: &str,
) {
    //Cloning:
    let results_ev_changed = Rc::clone(results);
    let pipe_content_clone = pipe_content.clone();
    let method = method.to_string();

    search_bar.connect_changed(move |search_bar| {
        let current_text = search_bar.text();

        while let Some(row) = results_ev_changed.last_child() {
            results_ev_changed.remove(&row);
        }
        let tiles = Tile::pipe_data(&pipe_content_clone, &method, &current_text);
        for item in tiles {
            results_ev_changed.append(&item);
        }

        results_ev_changed.focus_first();
    });
}
