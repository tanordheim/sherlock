use gtk4::{self, prelude::*, gdk, ApplicationWindow, Builder, EventControllerKey};
use gtk4::{Box as HVBox, Entry, Label, ListBox, ScrolledWindow};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use gtk4::glib;

use super::tiles::util::AsyncLauncherTile;
use super::util::*;
use crate::actions::execute_from_attrs;
use crate::launcher::Launcher;
use crate::loader::util::Config;

pub fn search(window: ApplicationWindow, launchers: Vec<Launcher>, app_config: Config) -> ApplicationWindow {
    let (mode, modes, vbox, search_bar, result_viewport, mode_title, results) = construct_window(&launchers);

    //RC cloning:
    let app_config = Rc::new(app_config);
    let app_config_ev_changed = Rc::clone(&app_config);

    let mode_ev_changed = Rc::clone(&mode);
    let mode_ev_nav = Rc::clone(&mode);

    let mode_title_clone = mode_title.clone();

    let results_ev_enter = Rc::clone(&results);
    let results_ev_nav = Rc::clone(&results);

    let launchers_ev_changed = launchers.clone();
    let launchers_ev_nav = launchers.clone();

    // Initiallize the view to show all apps
    set_home_screen("", "all", &*results, &launchers, &app_config);
    results.focus_first();

    // Setting search window to active
    //
    window.set_child(Some(&vbox));
    search_bar.grab_focus();

    let current_task: Rc<RefCell<Option<glib::JoinHandle<()>>>> = Rc::new(RefCell::new(None));
    let cancel_flag = Rc::new(RefCell::new(false));

    // Eventhandling for text change inside search bar
    // EVENT: Change
    search_bar.connect_changed(move |search_bar| {
        let current_text = search_bar.text().to_string();
        if let Some(task) = current_task.borrow_mut().take() {
            task.abort();
        };
        *cancel_flag.borrow_mut() = true;

        let launchers_ev_changed2 = launchers_ev_changed.clone();
        if modes.contains_key(&current_text) {
            if let Some(mode_name) = modes.get(&current_text) {
                set_mode(&mode_title_clone, &mode, &current_text, mode_name);
                search_bar.set_text("");
            }
        } else {
            *cancel_flag.borrow_mut() = false;
            let cancel_flag = Rc::clone(&cancel_flag);
            let (async_launchers, non_async_launchers): (Vec<Launcher>, Vec<Launcher>) =
                                                         launchers_ev_changed2
                                                         .into_iter()
                                                         .partition(|launcher| launcher.is_async());

            set_results(
                &current_text,
                &mode_ev_changed.borrow(),
                &*results,
                &non_async_launchers,
                &app_config_ev_changed,
            );
            let widgets: Vec<AsyncLauncherTile> = async_launchers
                .iter()
                .filter_map(|launcher| {
                    launcher.get_loader_widget(&current_text).map(
                        |(widget, title, body)| {
                            AsyncLauncherTile {
                                launcher: launcher.clone(),
                                widget,
                                title,
                                body,
                            }
                        },
                    )
                })
            .collect();
            for widget in widgets.iter() {
                if *mode_ev_changed.borrow().trim() == widget.launcher.alias() {
                    results.append(&widget.widget);
                } 
            }
            results.focus_first();

            let task = glib::MainContext::default().spawn_local(async move {
                if *cancel_flag.borrow() {
                    return;
                }
                for widget in widgets.iter() {
                    if let Some((title, body)) = widget.launcher.get_result(&current_text).await {
                        widget.title.set_text(&title);
                        widget.body.buffer().set_text(&body);
                    } 
                }
            });
            *current_task.borrow_mut() = Some(task);
        }
    });

    // Eventhandling for navigation
    // EVENT: Navigate
    let event_controller = EventControllerKey::new();
    event_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);
    event_controller.connect_key_pressed(move |_, key, _, modifiers| {
        match key {
            gdk::Key::Up => {
                results_ev_nav.focus_prev(&result_viewport);
            }
            gdk::Key::Down => {
                results_ev_nav.focus_next(&result_viewport);
                return true.into()
            }
            gdk::Key::BackSpace => {
                let ctext = &search_bar.text();

                if modifiers.contains(gdk::ModifierType::CONTROL_MASK) {
                    let _ = &search_bar.set_text("");
                } else {
                    if ctext.is_empty() {
                        set_mode(&mode_title, &mode_ev_nav, "all", &"All".to_string());
                        set_results(
                            &ctext,
                            &mode_ev_nav.borrow(),
                            &*results_ev_nav,
                            &launchers_ev_nav,
                            &app_config,
                        );
                    }
                }
                results_ev_nav.focus_first();
            }
            gdk::Key::Return => {
                if let Some(row) = results_ev_enter.selected_row() {
                    let attrs: HashMap<String, String> = get_row_attrs(row);
                    execute_from_attrs(attrs);
                }
            }
            gdk::Key::_1 => {
                if modifiers.contains(gdk::ModifierType::CONTROL_MASK) {
                    execute_by_index(&*results_ev_nav, 1);
                }
            }
            gdk::Key::_2 => {
                if modifiers.contains(gdk::ModifierType::CONTROL_MASK) {
                    execute_by_index(&*results_ev_nav, 2);
                }
            }
            gdk::Key::_3 => {
                if modifiers.contains(gdk::ModifierType::CONTROL_MASK) {
                    execute_by_index(&*results_ev_nav, 3);
                }
            }
            gdk::Key::_4 => {
                if modifiers.contains(gdk::ModifierType::CONTROL_MASK) {
                    execute_by_index(&*results_ev_nav, 4);
                }
            }
            gdk::Key::_5 => {
                if modifiers.contains(gdk::ModifierType::CONTROL_MASK) {
                    execute_by_index(&*results_ev_nav, 5);
                }
            }

            _ => (),
        }
        false.into()
    });

    window.add_controller(event_controller);

    return window;
}


fn construct_window(launchers: &Vec<Launcher>)
-> (Rc<RefCell<String>>, HashMap<String, String>, HVBox, Entry, ScrolledWindow, Label, Rc<ListBox>)
{
    // Collect Modes
    let mode = Rc::new(RefCell::new("all".to_string()));
    let mut modes: HashMap<String, String> = HashMap::new();
    for item in launchers.iter() {
        let alias = item.alias();
        if !alias.is_empty() {
            let name = item.name();
            modes.insert(format!("{} ", alias), name);
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

    (mode, modes, vbox, search_bar, result_viewport, mode_title, results)
}


