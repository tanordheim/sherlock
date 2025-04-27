use futures::future::join_all;
use gio::{glib::WeakRef, ActionEntry, ListStore};
use gtk4::{
    self, gdk::{self, Key, ModifierType}, prelude::*, Builder, CustomFilter, CustomSorter, EventControllerKey, FilterListModel, Image, ListScrollFlags, ListView, Overlay, SignalListItemFactory, SingleSelection, SortListModel, Spinner
};
use gtk4::{glib, ApplicationWindow, Entry};
use gtk4::{Box as GtkBox, Label, ScrolledWindow};
use levenshtein::levenshtein;
use simd_json::prelude::ArrayTrait;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::tiles::util::{AsyncLauncherTile, SherlockSearch};
use super::util::*;
use crate::{actions::execute_from_attrs, g_subclasses::sherlock_row::SherlockRow};
use crate::launcher::{Launcher, ResultItem};
use crate::CONFIG;

#[allow(dead_code)]
struct SearchUI {
    result_viewport: WeakRef<ScrolledWindow>,
    results: WeakRef<ListView>,
    // will be later used for split view to display information about apps/commands
    preview_box: WeakRef<GtkBox>,
    search_bar: WeakRef<Entry>,
    search_icon_holder: WeakRef<GtkBox>,
    mode_title: WeakRef<Label>,
    spinner: WeakRef<Spinner>,
    selection: WeakRef<SingleSelection>,
    filter: WeakRef<CustomFilter>,
    sorter: WeakRef<CustomSorter>,
}
//     current_task: &Rc<RefCell<Option<glib::JoinHandle<()>>>>,
fn update(update_tiles: Vec<AsyncLauncherTile>, current_task: &Rc<RefCell<Option<glib::JoinHandle<()>>>>){
    let current_task_clone = Rc::clone(current_task);
    let task = glib::MainContext::default().spawn_local({
        async move {
            // Set spinner active
            let spinner_row = update_tiles.get(0).map(|t| t.row.clone());
            if let Some(row) = spinner_row.as_ref().and_then(|row| row.upgrade()) {
                let _ = row.activate_action("win.spinner-mode", Some(&true.to_variant()));
            }
            // Make async tiles update concurrently
            let futures: Vec<_> = update_tiles
                .into_iter()
                .map(|widget| {
                    let current_text = String::new();
                    async move {
                        let mut attrs = widget.attrs.clone();

                        // Process text tile
                        if let Some(opts) = &widget.text_tile {
                            attrs = opts.update(&widget, &current_text, attrs).await;
                        }

                        // Process image replacement
                        if let Some(opts) = &widget.image_replacement {
                            opts.update(&widget).await;
                        }

                        // Process weather tile
                        if let Some(wtr) = &widget.weather_tile {
                            wtr.update(&widget).await
                        }

                        // Connect row-should-activate signal
                        widget.row.upgrade().map(|row| {
                            row.connect("row-should-activate", false, move |row| {
                                let row = row.first().map(|f| f.get::<SherlockRow>().ok())??;
                                execute_from_attrs(&row, &attrs);
                                None
                            });
                        });
                    }
                })
            .collect();

            let _ = join_all(futures).await;
            // Set spinner inactive
            if let Some(row) = spinner_row.as_ref().and_then(|row| row.upgrade()) {
                let _ = row.activate_action("win.spinner-mode", Some(&true.to_variant()));
            }
            *current_task_clone.borrow_mut() = None;
        }
    });
    *current_task.borrow_mut() = Some(task);
}
pub fn search(
    launchers: &Vec<Launcher>,
    window: &ApplicationWindow,
    stack_page_ref: &Rc<RefCell<String>>,
) -> GtkBox {
    // Initialize the view to show all apps
    let (search_query, mode, modes, stack_page, ui) = construct_window(&launchers);
    ui.result_viewport
        .upgrade()
        .map(|view| view.set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic));

    let initial_mode = mode.borrow().clone();
    let modes_clone = modes.clone();
    let mode_clone = Rc::clone(&mode);

    let search_bar = ui.search_bar.clone();
    stack_page.connect_realize(move |_| {
        search_bar
            .upgrade()
            .map(|search_bar| search_bar.grab_focus());
    });

    let custom_binds = ConfKeys::new();
    nav_event(ui.selection, ui.results.clone(), ui.search_bar.clone(), custom_binds, stack_page_ref);
    change_event(ui.search_bar.clone(), modes, &mode, ui.filter, ui.sorter, &search_query);

    // Improved mode selection
    let search_bar = ui.search_bar.clone();
    let mode_action = ActionEntry::builder("switch-mode")
        .parameter_type(Some(&String::static_variant_type()))
        .state(initial_mode.to_variant())
        .activate(move |_, action, parameter| {
            let state = action.state().and_then(|s| s.get::<String>());
            let parameter = parameter.and_then(|p| p.get::<String>());

            if let (Some(mut state), Some(mut parameter)) = (state, parameter) {
                match parameter.as_str() {
                    "search" => {
                        ui.search_icon_holder
                            .upgrade()
                            .map(|holder| holder.set_css_classes(&["back"]));
                        ui.mode_title
                            .upgrade()
                            .map(|title| title.set_text("Search"));
                    }
                    _ => {
                        parameter.push_str(" ");
                        let mode_name = modes_clone.get(&parameter);
                        match mode_name {
                            Some(name) => {
                                ui.search_icon_holder
                                    .upgrade()
                                    .map(|holder| holder.set_css_classes(&["back"]));
                                ui.mode_title.upgrade().map(|title| {
                                    title.set_text(name.as_deref().unwrap_or_default())
                                });
                                *mode_clone.borrow_mut() = parameter.clone();
                                state = parameter;
                            }
                            _ => {
                                ui.search_icon_holder
                                    .upgrade()
                                    .map(|holder| holder.set_css_classes(&["search"]));
                                ui.mode_title.upgrade().map(|title| title.set_text("All"));
                                parameter = String::from("all ");
                                *mode_clone.borrow_mut() = parameter.clone();
                                state = parameter;
                            }
                        }
                        let search_bar = search_bar.clone();
                        glib::idle_add_local(move || {
                            // to trigger homescreen rebuild
                            search_bar.upgrade().map(|entry| {
                                entry.set_text("\n");
                                entry.set_text("");
                            });
                            glib::ControlFlow::Break
                        });
                        action.set_state(&state.to_variant());
                    }
                }
            }
        })
        .build();

    // Spinner action
    let spinner_clone = ui.spinner;
    let action_spinner = ActionEntry::builder("spinner-mode")
        .parameter_type(Some(&bool::static_variant_type()))
        .activate(move |_, _, parameter| {
            let parameter = parameter.and_then(|p| p.get::<bool>());
            parameter.map(|p| {
                if p {
                    spinner_clone
                        .upgrade()
                        .map(|spinner| spinner.set_css_classes(&["spinner-appear"]));
                } else {
                    spinner_clone
                        .upgrade()
                        .map(|spinner| spinner.set_css_classes(&["spinner-disappear"]));
                };
                spinner_clone
                    .upgrade()
                    .map(|spinner| spinner.set_spinning(p));
            });
        })
        .build();

    let search_bar = ui.search_bar.clone();
    let action_clear_win = ActionEntry::builder("clear-search")
        .activate(move |_: &ApplicationWindow, _, _| {
            let search_bar = search_bar.clone();
            glib::idle_add_local(move || {
                search_bar.upgrade().map(|entry| entry.set_text(""));
                glib::ControlFlow::Break
            });
        })
        .build();
    window.add_action_entries([mode_action, action_clear_win, action_spinner]);

    return stack_page;
}

fn construct_window(
    launchers: &Vec<Launcher>,
) -> (
    Rc<RefCell<String>>,
    Rc<RefCell<String>>,
    HashMap<String, Option<String>>,
    GtkBox,
    SearchUI,
) {
    // Collect Modes
    let original_mode = CONFIG
        .get()
        .and_then(|c| c.behavior.sub_menu.as_deref())
        .unwrap_or("all");
    let mode = Rc::new(RefCell::new(original_mode.to_string()));
    let search_text = Rc::new(RefCell::new(String::from("")));
    let modes: HashMap<String, Option<String>> = launchers
        .iter()
        .filter_map(|item| item.alias.as_ref().map(|alias| (alias, &item.name)))
        .map(|(alias, name)| (format!("{} ", alias), name.clone()))
        .collect();

    // Initialize the builder with the correct path
    let builder = Builder::from_resource("/dev/skxxtz/sherlock/ui/search.ui");

    // Get the required object references
    let vbox: GtkBox = builder.object("vbox").unwrap();
    let results: ListView = builder.object("result-frame").unwrap();
    results.set_focusable(false);

    let search_icon_holder: GtkBox = builder.object("search-icon-holder").unwrap_or_default();
    search_icon_holder.add_css_class("search");
    // Create the search icon
    let search_icon = Image::new();
    search_icon.set_icon_name(Some("system-search-symbolic"));
    search_icon.set_widget_name("search-icon");
    search_icon.set_halign(gtk4::Align::End);
    // Create the back arrow
    let search_icon_back = Image::new();
    search_icon_back.set_icon_name(Some("go-previous-symbolic"));
    search_icon_back.set_widget_name("search-icon-back");
    search_icon_back.set_halign(gtk4::Align::End);

    let sorter = CustomSorter::new({
        let search_text = search_text.clone();

        fn make_prio(prio: f32, edits: usize) -> f32 {
            let normalized = (1000.0 / edits as f32).round() / 1000.0;
            let counters = prio.fract() / 1000.0;
            prio.trunc() + 1.0 + counters - normalized.clamp(0.0, 1.0)

        }
        move |item_a, item_b| {
            let search_text = search_text.borrow();

            let item_a = item_a.downcast_ref::<SherlockRow>().unwrap();
            let item_b = item_b.downcast_ref::<SherlockRow>().unwrap();

            let mut priority_a = item_a.priority();
            let mut priority_b = item_b.priority();

            if !search_text.is_empty() {
                priority_a = make_prio(item_a.priority(), levenshtein(&search_text, &item_a.search()));
                priority_b = make_prio(item_b.priority(), levenshtein(&search_text, &item_b.search()));
            }

            priority_a.total_cmp(&priority_b).into()
        }
    });
    let filter = CustomFilter::new({
        let search_text = search_text.clone();
        let search_mode = mode.clone();
        move |entry| {
            let item = entry.downcast_ref::<SherlockRow>().unwrap();
            let (home, only_home) = item.home();
            let alias = item.alias();
            let priority = item.priority();

            let mode = search_mode.borrow().trim().to_string();
            let current_text = search_text.borrow().clone();
            let is_home = current_text.is_empty() && mode == "all";

            if is_home {
                if home || only_home {
                    return true;
                }
                return false;
            } else {
                if mode != "all" {
                    if only_home || mode != alias {
                        return false;
                    }
                    if current_text.is_empty() {
                        return true;
                    }
                } else if priority <= 1.0{
                    return false
                }
                item.search()
                    .fuzzy_match(&current_text)
            }
        }
    });
    let model = ListStore::new::<SherlockRow>();
    let filter_model = FilterListModel::new(Some(model.clone()), Some(filter.clone()));
    let sorted_model = SortListModel::new(Some(filter_model), Some(sorter.clone()));
    let selection = SingleSelection::new(Some(sorted_model));
    let factory = SignalListItemFactory::new();

    let (async_launchers, non_async_launchers): (Vec<Launcher>, Vec<Launcher>) = launchers
        .clone()
        .into_iter()
        .partition(|launcher| launcher.r#async);
    let mut patches: Vec<ResultItem> = non_async_launchers.into_iter().map(|launcher| launcher.get_patch("")).flatten().collect();
    let tile_updates: Vec<AsyncLauncherTile> = async_launchers.into_iter().filter_map(|launcher| launcher.get_loader_widget("")).map(|(update, tile)| {
        patches.push(tile);
        update
    }).collect();
    patches.sort_by(|a, b| a.priority.partial_cmp(&b.priority).unwrap());
    let current_task: Rc<RefCell<Option<glib::JoinHandle<()>>>> = Rc::new(RefCell::new(None));
    update(tile_updates, &current_task);

    for item in patches.iter(){
        model.append(&item.row_item);
    }
    results.set_model(Some(&selection));

    factory.connect_bind(|_, item| {
        let item = item.downcast_ref::<gtk4::ListItem>().expect("Item mut be a ListItem");
        let row = item.item().clone().and_downcast::<SherlockRow>().expect("Row should be SherlockRow");
        item.set_child(Some(&row));
    });
    results.set_factory(Some(&factory));

    let overlay = Overlay::new();
    overlay.set_child(Some(&search_icon));
    overlay.add_overlay(&search_icon_back);

    // Show notification-bar
    CONFIG.get().map(|c| {
        if !c.appearance.status_bar {
            let n: Option<GtkBox> = builder.object("status-bar");
            n.map(|n| n.set_visible(false));
        }
    });

    search_icon_holder.append(&overlay);

    let spinner: Spinner = builder.object("status-bar-spinner").unwrap_or_default();
    let preview_box: GtkBox = builder.object("preview_box").unwrap_or_default();
    let search_bar: Entry = builder.object("search-bar").unwrap_or_default();
    let result_viewport: ScrolledWindow = builder.object("scrolled-window").unwrap_or_default();
    let mode_title: Label = builder.object("category-type-label").unwrap_or_default();
    let ui = SearchUI {
        result_viewport: result_viewport.downgrade(),
        results: results.downgrade(),
        preview_box: preview_box.downgrade(),
        search_bar: search_bar.downgrade(),
        search_icon_holder: search_icon_holder.downgrade(),
        mode_title: mode_title.downgrade(),
        spinner: spinner.downgrade(),
        selection: selection.downgrade(),
        filter: filter.downgrade(),
        sorter: sorter.downgrade(),
    };
    CONFIG.get().map(|c| {
        ui.result_viewport.upgrade().map(|viewport| {
            viewport.set_size_request((c.appearance.width as f32 * 0.4) as i32, 10);
        });
        ui.search_icon_holder
            .upgrade()
            .map(|holder| holder.set_visible(c.appearance.search_icon));
        search_icon.set_pixel_size(c.appearance.icon_size);
        search_icon_back.set_pixel_size(c.appearance.icon_size);
    });

    (search_text, mode, modes, vbox, ui)
}

fn nav_event(
    selection: WeakRef<SingleSelection>,
    results: WeakRef<ListView>,
    search_bar: WeakRef<Entry>,
    custom_binds: ConfKeys,
    stack_page: &Rc<RefCell<String>>,
) {
    let stack_page = Rc::clone(stack_page);
    let event_controller = EventControllerKey::new();
    event_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);
    event_controller.connect_key_pressed({
        let search_bar = search_bar.clone();
        move |_, key, i, modifiers| {
        if stack_page.borrow().as_str() != "search-page" {
            return false.into();
        };
        match key {
            k if Some(k) == custom_binds.prev
                && custom_binds
                    .prev_mod
                    .map_or(true, |m| modifiers.contains(m)) =>
            {
                let new_index = selection
                    .upgrade()
                    .map_or(0, |results| results.focus_prev());
                results.upgrade()
                    .map(|results| results.scroll_to(new_index, ListScrollFlags::NONE, None));
                return true.into()
            }
            k if Some(k) == custom_binds.next
                && custom_binds
                    .next_mod
                    .map_or(true, |m| modifiers.contains(m)) =>
            {
                let new_index = selection
                    .upgrade()
                    .map_or(0, |results| results.focus_next());
                results.upgrade()
                    .map(|results| results.scroll_to(new_index, ListScrollFlags::NONE, None));
                return true.into()
            }
            gdk::Key::Up => {
                let new_index = selection
                    .upgrade()
                    .map_or(0, |results| results.focus_prev());
                results.upgrade()
                    .map(|results| results.scroll_to(new_index, ListScrollFlags::NONE, None));
                return true.into()
            }
            gdk::Key::Down => {
                let new_index = selection
                    .upgrade()
                    .map_or(0, |results| results.focus_next());
                results.upgrade()
                    .map(|results| results.scroll_to(new_index, ListScrollFlags::NONE, None));
                return true.into();
            }
            gdk::Key::BackSpace => {
                let mut ctext = search_bar
                    .upgrade()
                    .map_or(String::new(), |entry| entry.text().to_string());
                if custom_binds
                    .shortcut_modifier
                    .map_or(false, |modifier| modifiers.contains(modifier))
                {
                    search_bar.upgrade().map(|entry| entry.set_text(""));
                    ctext.clear();
                }
                if ctext.is_empty() {
                    let _ = search_bar.upgrade().map(|entry| {
                        entry.activate_action("win.switch-mode", Some(&"all".to_variant()))
                    });
                }
                selection.upgrade().map(|results| results.focus_first());
            }
            gdk::Key::Return => {
                if let Some(upgr) = selection.upgrade() {
                    if let Some(row) = upgr.selected_item().and_downcast::<SherlockRow>(){
                        row.emit_by_name::<()>("row-should-activate", &[]);
                    }
                }
            }
            _ if key.to_unicode().and_then(|c| c.to_digit(10)).is_some() => {
                if custom_binds
                    .shortcut_modifier
                    .map_or(false, |modifier| modifiers.contains(modifier))
                {
                    if let Some(index) = key.name().and_then(|name| name.parse::<u32>().ok()){
                        println!("{}", index);
                        selection.upgrade().map(|r| r.execute_by_index(index));
                        return true.into();
                    }
                }
            }
            // Pain - solution for shift-tab since gtk handles it as an individual event
            _ if i == 23 && modifiers.contains(ModifierType::SHIFT_MASK) => {
                let shift = Some(ModifierType::SHIFT_MASK);
                let tab = Some(Key::Tab);
                if custom_binds.prev_mod == shift && custom_binds.prev == tab {
                    let new_index = selection
                        .upgrade()
                        .map_or(0, |results| results.focus_prev());
                    results.upgrade()
                        .map(|results| results.scroll_to(new_index, ListScrollFlags::NONE, None));
                    
                    return true.into();
                } else if custom_binds.next_mod == shift && custom_binds.next == tab {
                    let new_index = selection
                        .upgrade()
                        .map_or(0, |results| results.focus_next());
                    results.upgrade()
                        .map(|results| results.scroll_to(new_index, ListScrollFlags::NONE, None));
                    return true.into();
                }
            }
            _ => (),
        }
        false.into()
    }});

    search_bar.upgrade().map(|entry| entry.add_controller(event_controller));
}

fn change_event(
    search_bar: WeakRef<Entry>,
    modes: HashMap<String, Option<String>>,
    mode: &Rc<RefCell<String>>,
    filter: WeakRef<CustomFilter>,
    sorter: WeakRef<CustomSorter>,
    search_query: &Rc<RefCell<String>>,
) -> Option<()> {
    let search_bar = search_bar.upgrade()?;
    search_bar.connect_changed({
        let mode_clone = Rc::clone(mode);
        let search_query_clone = Rc::clone(search_query);

        move |search_bar| {
            let mut current_text = search_bar.text().to_string();
            if current_text.len() == 1 && current_text != "\n" {
                let _ = search_bar.activate_action("win.switch-mode", Some(&"search".to_variant()));
            } else if current_text.len() == 0 && mode_clone.borrow().as_str() == "all" {
                let _ = search_bar.activate_action("win.switch-mode", Some(&"all".to_variant()));
            }
            let trimmed = current_text.trim();
            if !trimmed.is_empty() && modes.contains_key(&current_text) {
                // Logic to apply modes
                let _ = search_bar.activate_action("win.switch-mode", Some(&trimmed.to_variant()));
                current_text.clear();
            }
            *search_query_clone.borrow_mut() = current_text.clone();
            filter.upgrade().map(|filter| filter.changed(gtk4::FilterChange::Different));
            sorter.upgrade().map(|sorter| sorter.changed(gtk4::SorterChange::Different));
        }
    });
    Some(())
}

// pub fn populate(
//     keyword: &str,
//     mode: &str,
//     results_frame: &WeakRef<ListBox>,
//     launchers: &Vec<Launcher>,
//     async_widgets: Option<Vec<ResultItem>>,
//     animate: bool,
//     mod_str: &str,
// ) {
//     // Remove all elements inside to avoid duplicates
//     if let Some(frame) = results_frame.upgrade() {
//         while let Some(row) = frame.last_child() {
//             frame.remove(&row);
//             row.unrealize();
//         }
//     }
//     let mut launcher_tiles = construct_tiles(&keyword.to_string(), &launchers, &mode.to_string());
//     if let Some(widgets) = async_widgets {
//         launcher_tiles.extend(widgets);
//     }

//     launcher_tiles.sort_by(|a, b| a.priority.partial_cmp(&b.priority).unwrap());

//     if let Some(c) = CONFIG.get() {
//         let mut shortcut_index = 1;
//         if let Some(frame) = results_frame.upgrade() {
//             launcher_tiles.into_iter().for_each(|widget| {
//                 if animate && c.behavior.animate {
//                     widget.row_item.add_css_class("animate");
//                 }
//                 if let Some(shortcut_holder) = &widget.shortcut_holder {
//                     shortcut_index += shortcut_holder
//                         .upgrade()
//                         .map_or(0, |holder| holder.apply_shortcut(shortcut_index, mod_str));
//                 }
//                 frame.append(&widget.row_item);
//             });
//         }
//     }
//     results_frame.upgrade().map(|r| r.focus_first());
// }
