use gtk4::glib;
use gtk4::{
    self,
    gdk::{self, Key},
    prelude::*,
    ApplicationWindow, Builder, EventControllerKey, Stack,
};
use gtk4::{Box as HVBox, Entry, Label, ListBox, ScrolledWindow};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::tiles::util::AsyncLauncherTile;
use super::util::*;
use crate::actions::execute_from_attrs;
use crate::launcher::Launcher;

pub fn search(
    window: ApplicationWindow,
    search_stack: &Stack,
    launchers: Vec<Launcher>,
) -> ApplicationWindow {
    // Initiallize the view to show all apps
    let (mode, modes, vbox, search_bar, result_viewport, mode_title, results) =
        construct_window(&launchers);
    result_viewport.set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic);
    set_home_screen("", "all", &*results, &launchers);
    results.focus_first();
    search_bar.grab_focus();

    change_event(&search_bar, modes, &mode_title, &mode, &launchers, &results);

    nav_event(
        &window,
        results,
        result_viewport,
        search_bar,
        mode_title,
        mode,
        launchers,
    );

    search_stack.add_named(&vbox, Some("search-page"));
    return window;
}

fn construct_window(
    launchers: &Vec<Launcher>,
) -> (
    Rc<RefCell<String>>,
    HashMap<String, String>,
    HVBox,
    Entry,
    ScrolledWindow,
    Label,
    Rc<ListBox>,
) {
    // Collect Modes
    let mode = Rc::new(RefCell::new("all".to_string()));
    let mut modes: HashMap<String, String> = HashMap::new();
    for item in launchers.iter() {
        let alias = item.alias.clone();
        if !alias.is_none() {
            modes.insert(format!("{} ", alias.unwrap()), item.name.clone());
        }
    }

    // Initialize the builder with the correct path
    let builder = Builder::from_resource("/dev/skxxtz/sherlock/ui/search.ui");

    // Get the requred object references
    let vbox: HVBox = builder.object("vbox").unwrap();
    let search_bar: Entry = builder.object("search-bar").unwrap();
    let result_viewport: ScrolledWindow = builder.object("scrolled-window").unwrap();
    let mode_title: Label = builder.object("category-type-label").unwrap();
    let results: Rc<ListBox> = Rc::new(builder.object("result-frame").unwrap());

    (
        mode,
        modes,
        vbox,
        search_bar,
        result_viewport,
        mode_title,
        results,
    )
}

fn nav_event(
    window: &ApplicationWindow,
    results_ev_nav: Rc<ListBox>,
    result_viewport: ScrolledWindow,
    search_bar: Entry,
    mode_title_ev_nav: Label,
    mode_ev_nav: Rc<RefCell<String>>,
    launchers_ev_nav: Vec<Launcher>,
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
            gdk::Key::BackSpace => {
                let ctext = &search_bar.text();
                if modifiers.contains(gdk::ModifierType::CONTROL_MASK) {
                    let _ = &search_bar.set_text("");
                } else {
                    if ctext.is_empty() {
                        set_mode(&mode_title_ev_nav, &mode_ev_nav, "all", &"All".to_string());
                        set_results(
                            &ctext,
                            &mode_ev_nav.borrow(),
                            &*results_ev_nav,
                            &launchers_ev_nav,
                        );
                    }
                }
                results_ev_nav.focus_first();
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
    modes: HashMap<String, String>,
    mode_title: &Label,
    mode: &Rc<RefCell<String>>,
    launchers: &Vec<Launcher>,
    results: &Rc<ListBox>,
) {
    //Cloning:
    let mode_title_ev_changed = mode_title.clone();
    let launchers_ev_changed = launchers.clone();
    let mode_ev_changed = Rc::clone(mode);
    let results_ev_changed = Rc::clone(results);

    let current_task: Rc<RefCell<Option<glib::JoinHandle<()>>>> = Rc::new(RefCell::new(None));
    let cancel_flag = Rc::new(RefCell::new(false));

    search_bar.connect_changed(move |search_bar| {
        let current_text = search_bar.text().to_string();
        if let Some(task) = current_task.borrow_mut().take() {
            task.abort();
        };
        *cancel_flag.borrow_mut() = true;
        if modes.contains_key(&current_text){
            // Logic to apply modes
            if let Some(mode_name) = modes.get(&current_text) {
                set_mode(
                    &mode_title_ev_changed,
                    &mode_ev_changed,
                    &current_text,
                    mode_name,
                );
                search_bar.set_text("");
            }
        } else {
            *cancel_flag.borrow_mut() = false;
            let cancel_flag = Rc::clone(&cancel_flag);
            let (async_launchers, non_async_launchers): (Vec<Launcher>, Vec<Launcher>) =
                launchers_ev_changed
                    .clone()
                    .into_iter()
                    .partition(|launcher| launcher.r#async);

            set_results(
                &current_text,
                &mode_ev_changed.borrow(),
                &*results_ev_changed,
                &non_async_launchers,
            );

            // Create loader widgets
            // TODO
            let current_mode = mode_ev_changed.borrow().trim().to_string();
            let widgets: Vec<AsyncLauncherTile> = async_launchers
                .iter()
                .filter_map(|launcher| {
                    if current_mode == launcher.alias.as_deref().unwrap_or("") {
                        launcher
                            .get_loader_widget(&current_text)
                            .map(|(widget, title, body)| AsyncLauncherTile {
                                launcher: launcher.clone(),
                                widget,
                                title,
                                body,
                            })
                    } else {
                        None
                    }
                })
                .collect();

            for widget in widgets.iter() {
                results_ev_changed.append(&widget.widget);
            }
            results_ev_changed.focus_first();

            // Async widget execution
            let task = glib::MainContext::default().spawn_local(async move {
                if *cancel_flag.borrow() {
                    return;
                }
                for widget in widgets.iter() {
                    if let Some((title, body)) = widget.launcher.get_result(&current_text).await {
                        widget.title.set_text(&title);
                        widget.body.set_text(&body);
                    }
                }
            });
            *current_task.borrow_mut() = Some(task);
        }
    });
}
