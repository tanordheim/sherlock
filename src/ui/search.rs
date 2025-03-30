use gtk4::{
    self,
    gdk::{self, Key, ModifierType},
    prelude::*,
    Builder, EventControllerKey, Image,
};
use gtk4::glib;
use gtk4::{Box as HVBox, Entry, Label, ListBox, ScrolledWindow};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::tiles::util::AsyncLauncherTile;
use super::util::*;
use crate::actions::execute_from_attrs;
use crate::launcher::{construct_tiles, Launcher, ResultItem};
use crate::{AppState, APP_STATE, CONFIG};

#[allow(dead_code)]
struct SearchUI {
    result_viewport: ScrolledWindow,
    // will be later used for split view to display information about apps/commands
    preview_box: HVBox,
    search_bar: Entry,
    mode_title: Label,
}

pub fn search(launchers: Vec<Launcher>) {
    // Initiallize the view to show all apps
    let (mode, modes, vbox, ui, results) = construct_window(&launchers);
    ui.result_viewport
        .set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic);

    change_event(&ui, modes, &mode, &launchers, &results);
    let custom_binds = CONFIG.get().map_or(ConfKeys::empty(), |c| {
        let prev = c.binds.prev.clone().unwrap_or_default();
        let next = c.binds.next.clone().unwrap_or_default();
        ConfKeys::from(next, prev)
    });

    ui.search_bar.grab_focus();

    nav_event(results, ui, mode, custom_binds);
    APP_STATE.with(|state| {
        if let Some(ref state) = *state.borrow() {
            state.add_stack_page(vbox, "search-page");
        }
    });
}

fn construct_window(
    launchers: &Vec<Launcher>,
) -> (
    Rc<RefCell<String>>,
    HashMap<String, String>,
    HVBox,
    SearchUI,
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
    let results: Rc<ListBox> = Rc::new(builder.object("result-frame").unwrap());
    let ui = SearchUI {
        result_viewport: builder.object("scrolled-window").unwrap_or_default(),
        preview_box: builder.object("preview_box").unwrap_or_default(),
        search_bar: builder.object("search-bar").unwrap_or_default(),
        mode_title: builder.object("category-type-label").unwrap_or_default(),
    };
    if let Some(c) = CONFIG.get() {
        ui.result_viewport
            .set_size_request((c.appearance.width as f32 * 0.4) as i32, -1);
    }

    APP_STATE.with(|app_state| {
        let new_state = app_state.borrow_mut().take().map(|old_state| {
            Rc::new(AppState {
                window: old_state.window.clone(),
                stack: old_state.stack.clone(),
                search_bar: Some(ui.search_bar.clone()),
            })
        });
        *app_state.borrow_mut() = new_state;
    });
    (mode, modes, vbox, ui, results)
}

fn nav_event(
    results_ev_nav: Rc<ListBox>,
    ui: SearchUI,
    mode_ev_nav: Rc<RefCell<String>>,
    custom_binds: ConfKeys,
) {
    let conf_keys = ConfKeys::new();
    println!("{:?}", conf_keys);
    let event_controller = EventControllerKey::new();
    event_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);
    event_controller.connect_key_pressed(move |_, key, i, modifiers| {
        match key {
            k if Some(k) == custom_binds.prev
                && custom_binds
                    .prev_mod
                    .map_or(true, |m| modifiers.contains(m)) =>
            {
                results_ev_nav.focus_prev(&ui.result_viewport);
                return true.into();
            }
            k if Some(k) == custom_binds.next
                && custom_binds
                    .next_mod
                    .map_or(true, |m| modifiers.contains(m)) =>
            {
                results_ev_nav.focus_next(&ui.result_viewport);
                return true.into();
            }
            gdk::Key::Up => {
                results_ev_nav.focus_prev(&ui.result_viewport);
            }
            gdk::Key::Down => {
                results_ev_nav.focus_next(&ui.result_viewport);
                return true.into();
            }
            gdk::Key::BackSpace => {
                let ctext = &ui.search_bar.text();
                if conf_keys.shortcut_modifier.map_or(false, |modifier| modifiers.contains(modifier)){
                    let _ = &ui.search_bar.set_text("");
                } else {
                    if ctext.is_empty() {
                        set_mode(&ui.mode_title, &mode_ev_nav, "all", &"All".to_string());
                        // to trigger homescreen rebuild
                        let _ = &ui.search_bar.set_text("");
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
                if conf_keys.shortcut_modifier.map_or(false, |modifier| modifiers.contains(modifier)){
                    let key_index = match key {
                        Key::_1 => 1,
                        Key::_2 => 2,
                        Key::_3 => 3,
                        Key::_4 => 4,
                        Key::_5 => 5,
                        _ => return false.into(),
                    };
                    execute_by_index(&*results_ev_nav, key_index);
                    return  true.into();
                }
            }
            _ if i == 23 && modifiers.contains(ModifierType::SHIFT_MASK) => {
                let shift = Some(ModifierType::SHIFT_MASK);
                let tab = Some(Key::Tab);
                if conf_keys.prev_mod == shift && conf_keys.prev == tab {
                    results_ev_nav.focus_prev(&ui.result_viewport);
                    return true.into();
                } else if conf_keys.next_mod == shift && conf_keys.next == tab {
                    results_ev_nav.focus_next(&ui.result_viewport);
                    return true.into();
                }
            },
             _ => (),
        }
        false.into()
    });
    APP_STATE.with(|state| {
        if let Some(ref state) = *state.borrow() {
            state.add_event_listener(event_controller);
        }
    });
}

fn change_event(
    ui: &SearchUI,
    modes: HashMap<String, String>,
    mode: &Rc<RefCell<String>>,
    launchers: &Vec<Launcher>,
    results: &Rc<ListBox>,
) {
    //Cloning:
    let mode_title_ev_changed = ui.mode_title.clone();
    let launchers_ev_changed = launchers.clone();
    let mode_ev_changed = Rc::clone(mode);
    let results_ev_changed = Rc::clone(results);

    // Setting up async capabilities
    let current_task: Rc<RefCell<Option<glib::JoinHandle<()>>>> = Rc::new(RefCell::new(None));
    let cancel_flag = Rc::new(RefCell::new(false));

    async_calc(
        &cancel_flag,
        &current_task,
        &launchers_ev_changed,
        &mode_ev_changed,
        String::new(),
        &results_ev_changed,
        true,
    );

    ui.search_bar.connect_changed(move |search_bar| {
        let current_text = search_bar.text().to_string();
        if let Some(task) = current_task.borrow_mut().take() {
            task.abort();
        };
        *cancel_flag.borrow_mut() = true;
        let tmp = current_text.trim();
        if modes.contains_key(tmp) {
            // Logic to apply modes
            if let Some(mode_name) = modes.get(&current_text) {
                set_mode(
                    &mode_title_ev_changed,
                    &mode_ev_changed,
                    &current_text,
                    mode_name,
                );
                search_bar.set_text("");

                set_results(
                    "",
                    &mode_ev_changed.borrow(),
                    &*results_ev_changed,
                    &launchers_ev_changed,
                    None,
                    false,
                );
            }
        } else {
            async_calc(
                &cancel_flag,
                &current_task,
                &launchers_ev_changed,
                &mode_ev_changed,
                current_text,
                &results_ev_changed,
                false,
            );
        }
    });
}

pub fn async_calc(
    cancel_flag: &Rc<RefCell<bool>>,
    current_task: &Rc<RefCell<Option<glib::JoinHandle<()>>>>,
    launchers: &Vec<Launcher>,
    mode: &Rc<RefCell<String>>,
    current_text: String,
    results: &Rc<ListBox>,
    home: bool,
) {
    *cancel_flag.borrow_mut() = false;
    let cancel_flag = Rc::clone(&cancel_flag);
    let launchers = if home {
        let (show, _): (Vec<Launcher>, Vec<Launcher>) = launchers
            .clone()
            .into_iter()
            .partition(|launcher| launcher.home);
        show
    } else {
        launchers.clone()
    };
    let (async_launchers, non_async_launchers): (Vec<Launcher>, Vec<Launcher>) =
        launchers.into_iter().partition(|launcher| launcher.r#async);

    // Create loader widgets
    // TODO
    let current_mode = mode.borrow().trim().to_string();
    let widgets: Vec<AsyncLauncherTile> = async_launchers
        .iter()
        .filter_map(|launcher| {
            if launcher.priority == 0 && current_mode == launcher.alias.as_deref().unwrap_or("")
                || launcher.priority > 0
            {
                launcher.get_loader_widget(&current_text).map(
                    |(widget, title, body, async_opts, attrs)| AsyncLauncherTile {
                        launcher: launcher.clone(),
                        result_item: widget,
                        title,
                        body,
                        async_opts,
                        attrs,
                    },
                )
            } else {
                None
            }
        })
        .collect();

    set_results(
        &current_text,
        &mode.borrow(),
        &*results,
        &non_async_launchers,
        Some(&widgets),
        home,
    );
    results.focus_first();

    // Async widget execution
    let task = glib::MainContext::default().spawn_local(async move {
        if *cancel_flag.borrow() {
            return;
        }
        // get results for aysnc launchers
        for widget in widgets.iter() {
            if let Some((title, body, next_content)) =
                widget.launcher.get_result(&current_text).await
            {
                widget.title.as_ref().map(|t| t.set_text(&title));
                widget.body.as_ref().map(|b| b.set_text(&body));
                if let Some(next_content) = next_content {
                    let label =
                        Label::new(Some(format!("next_content | {}", next_content).as_str()));
                    widget.attrs.append(&label);
                }
            }
            if let Some(opts) = &widget.async_opts {
                // Replace one image with another
                if let Some(overlay) = &opts.icon_holder_overlay {
                    if let Some((image, was_cached)) = widget.launcher.get_image().await {
                        // Also check for animate key
                        if !was_cached {
                            overlay.add_css_class("image-replace-overlay");
                        }
                        let gtk_image = Image::from_pixbuf(Some(&image));
                        gtk_image.set_widget_name("album-cover");
                        gtk_image.set_pixel_size(50);
                        overlay.add_overlay(&gtk_image);
                    }
                }
            }
        }
    });
    *current_task.borrow_mut() = Some(task);
}

pub fn set_results(
    keyword: &str,
    mode: &str,
    results_frame: &ListBox,
    launchers: &Vec<Launcher>,
    async_launchers: Option<&Vec<AsyncLauncherTile>>,
    home: bool,
) {
    // Remove all elements inside to avoid duplicates
    let mut launcher_tiles = Vec::new();
    while let Some(row) = results_frame.last_child() {
        results_frame.remove(&row);
    }
    let widgets = construct_tiles(&keyword.to_string(), &launchers, &mode.to_string());
    launcher_tiles.extend(widgets);
    if let Some(a_wid) = async_launchers {
        let widgets: Vec<ResultItem> = a_wid.into_iter().map(|l| l.result_item.clone()).collect();
        launcher_tiles.extend(widgets);
    }

    launcher_tiles.sort_by(|a, b| a.priority.partial_cmp(&b.priority).unwrap());

    if let Some(c) = CONFIG.get() {
        let mut shortcut_index = 1;
        for widget in launcher_tiles {
            if home && c.behavior.animate {
                widget.row_item.add_css_class("animate");
            }
            if let Some(shortcut_holder) = widget.shortcut_holder {
                shortcut_index += shortcut_holder.apply_shortcut(shortcut_index);
            }
            results_frame.append(&widget.row_item);
        }
    }
    results_frame.focus_first();
}
