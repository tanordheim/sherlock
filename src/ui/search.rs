use gdk_pixbuf::subclass::prelude::ObjectSubclassIsExt;
use gio::{glib::WeakRef, ActionEntry, ListStore};
use gtk4::{
    self,
    gdk::{self, Key, ModifierType},
    prelude::*,
    Builder, CustomFilter, CustomSorter, EventControllerKey, FilterListModel, Image, ListView,
    Overlay, SignalListItemFactory, SingleSelection, SortListModel, Spinner,
};
use gtk4::{glib, ApplicationWindow, Entry};
use gtk4::{Box as GtkBox, Label, ScrolledWindow};
use levenshtein::levenshtein;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::context::make_context;
use super::util::*;
use crate::{
    g_subclasses::sherlock_row::SherlockRow,
    prelude::{SherlockNav, SherlockSearch, ShortCut},
    ui::key_actions::KeyActions,
    Sherlock,
};
use crate::{
    sherlock_error,
    utils::errors::{SherlockError, SherlockErrorType},
    CONFIG,
};

#[sherlock_macro::timing("Search Window Creation")]
pub fn search(
    window: &ApplicationWindow,
    stack_page_ref: &Rc<RefCell<String>>,
    error_model: WeakRef<ListStore>,
    sherlock: Rc<RefCell<Sherlock>>,
) -> Result<(Overlay, SearchHandler), SherlockError> {
    // Initialize the view to show all apps
    let (search_query, mode, stack_page, ui, handler, context) = construct_window(error_model)?;
    ui.result_viewport
        .upgrade()
        .map(|view| view.set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic));

    {
        let mut sherlock = sherlock.borrow_mut();
        sherlock.search_ui = Some(ui.clone());
        sherlock.search_handler = Some(handler.clone());
    }

    // Mode setup - used to decide which tiles should be shown
    let initial_mode = mode.borrow().clone();

    // Initial setup on show
    stack_page.connect_realize({
        let search_bar = ui.search_bar.clone();
        let results = ui.results.clone();
        let context_model = context.model.clone();
        let current_mode = Rc::clone(&mode);
        move |_| {
            // Focus search bar as soon as it's visible
            search_bar
                .upgrade()
                .map(|search_bar| search_bar.grab_focus());
            // Show or hide context menu shortcuts whenever stack shows
            results
                .upgrade()
                .map(|r| r.focus_first(Some(&context_model), Some(current_mode.clone())));
        }
    });

    nav_event(
        ui.results.clone(),
        ui.search_bar.clone(),
        ui.filter.clone(),
        ui.sorter.clone(),
        ui.binds,
        stack_page_ref,
        &mode,
        context.clone(),
    );
    change_event(
        ui.search_bar.clone(),
        ui.results.clone(),
        Rc::clone(&handler.modes),
        &mode,
        &search_query,
    );

    // Improved mode selection
    let mode_action = ActionEntry::builder("switch-mode")
        .parameter_type(Some(&String::static_variant_type()))
        .state(initial_mode.to_variant())
        .activate({
            let mode_clone = Rc::clone(&mode);
            let modes_clone = Rc::clone(&handler.modes);
            move |_, action, parameter| {
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
                            let mode_name = modes_clone.borrow().get(&parameter).cloned();
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
                            action.set_state(&state.to_variant());
                        }
                    }
                }
            }
        })
        .build();

    // Action to update filter and sorter
    let sorter_actions = ActionEntry::builder("update-items")
        .parameter_type(Some(&bool::static_variant_type()))
        .activate({
            let filter = ui.filter.clone();
            let sorter = ui.sorter.clone();
            let results = ui.results.clone();
            let current_task = handler.task.clone();
            let current_text = search_query.clone();
            let context_model = context.model.clone();
            let current_mode = Rc::clone(&mode);
            move |_: &ApplicationWindow, _, parameter| {
                if let Some(focus_first) = parameter.and_then(|p| p.get::<bool>()) {
                    filter
                        .upgrade()
                        .map(|filter| filter.changed(gtk4::FilterChange::Different));
                    sorter
                        .upgrade()
                        .map(|sorter| sorter.changed(gtk4::SorterChange::Different));
                    if let Some(results) = results.upgrade() {
                        let weaks = results.get_weaks().unwrap_or(vec![]);
                        if focus_first {
                            if results
                                .focus_first(Some(&context_model), Some(current_mode.clone()))
                                .is_some()
                            {
                                update_async(weaks, &current_task, current_text.borrow().clone());
                            }
                        } else {
                            update_async(weaks, &current_task, current_text.borrow().clone());
                        }
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

    // Action to display or hide context menu shortcut
    let context_action = ActionEntry::builder("context-mode")
        .parameter_type(Some(&bool::static_variant_type()))
        .activate({
            let desc = ui.context_menu_desc.clone();
            let first = ui.context_menu_first.clone();
            let second = ui.context_menu_second.clone();
            move |_, _, parameter| {
                let parameter = parameter.and_then(|p| p.get::<bool>());
                parameter.map(|p| {
                    if p {
                        desc.upgrade().map(|tmp| tmp.set_css_classes(&["active"]));
                        first.upgrade().map(|tmp| tmp.set_css_classes(&["active"]));
                        second.upgrade().map(|tmp| tmp.set_css_classes(&["active"]));
                    } else {
                        desc.upgrade().map(|tmp| tmp.set_css_classes(&["inactive"]));
                        first
                            .upgrade()
                            .map(|tmp| tmp.set_css_classes(&["inactive"]));
                        second
                            .upgrade()
                            .map(|tmp| tmp.set_css_classes(&["inactive"]));
                    };
                });
            }
        })
        .build();

    let search_bar = ui.search_bar.clone();
    let action_clear_win = ActionEntry::builder("clear-search")
        .activate(move |_: &ApplicationWindow, _, _| {
            let search_bar = search_bar.clone();
            glib::idle_add_local(move || {
                if let Some(entry) = search_bar.upgrade() {
                    entry.set_text("");
                }
                glib::ControlFlow::Break
            });
        })
        .build();
    window.add_action_entries([
        mode_action,
        action_clear_win,
        action_spinner,
        context_action,
        sorter_actions,
    ]);

    return Ok((stack_page, handler));
}

fn construct_window(
    error_model: WeakRef<ListStore>,
) -> Result<
    (
        Rc<RefCell<String>>,
        Rc<RefCell<String>>,
        Overlay,
        SearchUI,
        SearchHandler,
        ContextUI,
    ),
    SherlockError,
> {
    // Collect Modes
    let custom_binds = ConfKeys::new();
    let config = CONFIG
        .get()
        .ok_or_else(|| sherlock_error!(SherlockErrorType::ConfigError(None), ""))?;
    let original_mode = config.behavior.sub_menu.as_deref().unwrap_or("all");
    let mode = Rc::new(RefCell::new(original_mode.to_string()));
    let search_text = Rc::new(RefCell::new(String::from("")));

    // Initialize the builder with the correct path
    let builder = Builder::from_resource("/dev/skxxtz/sherlock/ui/search.ui");

    // Get the required object references
    let vbox: GtkBox = builder.object("vbox").unwrap();
    let results: ListView = builder.object("result-frame").unwrap();
    results.set_focusable(false);

    let (context, revealer) = make_context();
    let main_overlay = Overlay::new();
    main_overlay.set_child(Some(&vbox));
    main_overlay.add_overlay(&revealer);

    let search_icon_holder: GtkBox = builder.object("search-icon-holder").unwrap_or_default();
    search_icon_holder.add_css_class("search");
    // Create the search icon
    let search_icon = Image::new();
    search_icon.set_icon_name(Some(&config.appearance.search_bar_icon));
    search_icon.set_widget_name("search-icon");
    search_icon.set_halign(gtk4::Align::End);
    search_icon.set_pixel_size(config.appearance.search_icon_size);
    // Create the back arrow
    let search_icon_back = Image::new();
    search_icon_back.set_icon_name(Some(&config.appearance.search_bar_icon_back));
    search_icon_back.set_widget_name("search-icon-back");
    search_icon_back.set_halign(gtk4::Align::End);
    search_icon_back.set_pixel_size(config.appearance.search_icon_size);
    // Set search icons
    let overlay = Overlay::new();
    overlay.set_child(Some(&search_icon));
    overlay.add_overlay(&search_icon_back);

    // Setup model and factory
    let model = ListStore::new::<SherlockRow>();
    let factory = make_factory();
    results.set_factory(Some(&factory));

    // Setup selection
    let sorter = make_sorter(&search_text);
    let filter = make_filter(&search_text, &mode);
    let filter_model = FilterListModel::new(Some(model.clone()), Some(filter.clone()));
    let sorted_model = SortListModel::new(Some(filter_model), Some(sorter.clone()));

    // Set and update `modkey + num` shortcut ui
    sorted_model.connect_items_changed({
        let mod_str = custom_binds.shortcut_modifier_str.clone();
        move |myself, _, removed, added| {
            // Early exit if nothing changed
            if added == 0 && removed == 0 {
                return;
            }
            let mut added_index = 0;
            for i in 0..myself.n_items() {
                if let Some(item) = myself.item(i).and_downcast::<SherlockRow>() {
                    if item.imp().shortcut.get() {
                        if let Some(shortcut_holder) = item.shortcut_holder() {
                            if added_index < 5 {
                                added_index +=
                                    shortcut_holder.apply_shortcut(added_index + 1, &mod_str);
                            } else {
                                shortcut_holder.remove_shortcut();
                            }
                        }
                    }
                }
            }
        }
    });

    let selection = SingleSelection::new(Some(sorted_model));
    results.set_model(Some(&selection));

    // Add tiles to the view and create modes
    let handler = SearchHandler::new(model.downgrade(), error_model);
    handler.populate();

    results.set_model(Some(&selection));
    results.set_factory(Some(&factory));

    search_icon_holder.append(&overlay);

    let all: GtkBox = builder.object("split-view").unwrap_or_default();
    let spinner: Spinner = builder.object("status-bar-spinner").unwrap_or_default();
    let preview_box: GtkBox = builder.object("preview_box").unwrap_or_default();
    let search_bar: Entry = builder.object("search-bar").unwrap_or_default();
    let result_viewport: ScrolledWindow = builder.object("scrolled-window").unwrap_or_default();
    let mode_title_holder: GtkBox = builder.object("category-type-holder").unwrap_or_default();
    let mode_title: Label = builder.object("category-type-label").unwrap_or_default();
    let context_action_desc: Label = builder.object("context-menu-desc").unwrap_or_default();
    let context_action_first: Label = builder.object("context-menu-first").unwrap_or_default();
    let context_action_second: Label = builder.object("context-menu-second").unwrap_or_default();
    let status_bar: GtkBox = builder.object("status-bar").unwrap_or_default();

    if let Some(context_str) = &custom_binds.context_str {
        context_action_first.set_text(&custom_binds.context_mod_str);
        context_action_second.set_text(context_str);
    } else {
        context_action_first.set_visible(false);
        context_action_second.set_visible(false);
    }

    if config.expand.enable {
        result_viewport.set_max_content_height(config.appearance.height);
        result_viewport.set_propagate_natural_height(true);
    }

    let ui = SearchUI {
        all: all.downgrade(),
        result_viewport: result_viewport.downgrade(),
        results: results.downgrade(),
        preview_box: preview_box.downgrade(),
        status_bar: status_bar.downgrade(),
        search_bar: search_bar.downgrade(),
        search_icon_holder: search_icon_holder.downgrade(),
        mode_title_holder: mode_title_holder.downgrade(),
        mode_title: mode_title.downgrade(),
        spinner: spinner.downgrade(),
        filter: filter.downgrade(),
        sorter: sorter.downgrade(),
        binds: custom_binds,
        context_menu_desc: context_action_desc.downgrade(),
        context_menu_first: context_action_first.downgrade(),
        context_menu_second: context_action_second.downgrade(),
    };
    // disable status bar
    if !config.appearance.status_bar {
        status_bar.set_visible(false);
    }
    // enable or disable search bar icons
    ui.search_icon_holder
        .upgrade()
        .map(|holder| holder.set_visible(config.appearance.search_icon));

    Ok((search_text, mode, main_overlay, ui, handler, context))
}
fn make_factory() -> SignalListItemFactory {
    let factory = SignalListItemFactory::new();
    factory.connect_bind(|_, item| {
        let item = item
            .downcast_ref::<gtk4::ListItem>()
            .expect("Item mut be a ListItem");
        let row = item
            .item()
            .clone()
            .and_downcast::<SherlockRow>()
            .expect("Row should be SherlockRow");
        item.set_child(Some(&row));
    });
    factory
}
fn make_filter(search_text: &Rc<RefCell<String>>, mode: &Rc<RefCell<String>>) -> CustomFilter {
    CustomFilter::new({
        let search_text = Rc::clone(search_text);
        let search_mode = Rc::clone(mode);
        move |entry| {
            let item = entry.downcast_ref::<SherlockRow>().unwrap();
            let (home, only_home) = item.home();

            let mode = search_mode.borrow().trim().to_string();
            let current_text = search_text.borrow().clone();
            let is_home = current_text.is_empty() && mode == "all";

            let update_res = item.update(&current_text);

            if is_home {
                if home || only_home {
                    return true;
                }
                return false;
            } else {
                let alias = item.alias();
                let priority = item.priority();
                if mode != "all" {
                    if only_home || mode != alias {
                        return false;
                    }
                    if current_text.is_empty() {
                        return true;
                    }
                } else if priority <= 1.0 {
                    return false;
                }
                if item.is_keyword_aware() {
                    return true;
                }

                if update_res {
                    return true;
                }
                item.search().fuzzy_match(&current_text)
            }
        }
    })
}
fn make_sorter(search_text: &Rc<RefCell<String>>) -> CustomSorter {
    CustomSorter::new({
        let search_text = Rc::clone(search_text);
        fn search_score(query: &str, match_in: &str) -> f32 {
            if match_in.len() == 0 {
                return 0.0;
            }
            let distance = levenshtein(query, match_in) as f32;
            let normed = (distance / match_in.len() as f32).clamp(0.2, 1.0);
            let starts_with = if match_in.starts_with(query) {
                -0.2
            } else {
                0.0
            };
            normed + starts_with
        }

        fn make_prio(prio: f32, query: &str, match_in: &str) -> f32 {
            let score = search_score(query, match_in);
            // shift counts 3 to right; 1.34 → 1.00034 to make room for levenshtein
            let counters = prio.fract() / 1000.0;
            prio.trunc() + (counters + score).min(0.99)
        }
        move |item_a, item_b| {
            let search_text = search_text.borrow();

            let item_a = item_a.downcast_ref::<SherlockRow>().unwrap();
            let item_b = item_b.downcast_ref::<SherlockRow>().unwrap();

            let mut priority_a = item_a.priority();
            let mut priority_b = item_b.priority();

            if !search_text.is_empty() {
                priority_a = make_prio(item_a.priority(), &search_text, &item_a.search());
                priority_b = make_prio(item_b.priority(), &search_text, &item_b.search());
            }

            priority_a.total_cmp(&priority_b).into()
        }
    })
}

fn nav_event(
    results: WeakRef<ListView>,
    search_bar: WeakRef<Entry>,
    filter: WeakRef<CustomFilter>,
    sorter: WeakRef<CustomSorter>,
    binds: ConfKeys,
    stack_page: &Rc<RefCell<String>>,
    current_mode: &Rc<RefCell<String>>,
    context: ContextUI,
) {
    let stack_page = Rc::clone(stack_page);
    let event_controller = EventControllerKey::new();
    event_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);
    event_controller.connect_key_pressed({
        let search_bar = search_bar.clone();
        let current_mode = Rc::clone(current_mode);
        let key_actions = KeyActions::new(results, search_bar, context);
        move |_, key, i, mods| {
            if stack_page.borrow().as_str() != "search-page" {
                return false.into();
            };
            let matches = |comp: Option<Key>, comp_mod: Option<ModifierType>| {
                let key_matches = Some(key) == comp;
                let mod_matches = comp_mod.map_or(false, |m| mods.contains(m));
                key_matches && mod_matches
            };

            match key {
                // Inplace execution of commands
                _ if matches(binds.exec_inplace, binds.exec_inplace_mod) => key_actions.on_return(key_actions.context.open.get(), Some(false)),

                // Context menu opening
                _ if matches(binds.context, binds.context_mod) => {key_actions.open_context();},

                // Custom previous key
                _ if matches(binds.prev, binds.prev_mod) => {key_actions.on_prev();},
                Key::Up => key_actions.on_prev(),

                // Custom next key
                _ if matches(binds.next, binds.next_mod) => {key_actions.on_next();},
                Key::Down => key_actions.on_next(),

                Key::BackSpace => {
                    let mut ctext = key_actions
                        .search_bar
                        .upgrade()
                        .map_or(String::new(), |entry| entry.text().to_string());
                    if binds
                        .shortcut_modifier
                        .map_or(false, |modifier| mods.contains(modifier))
                    {
                        key_actions
                            .search_bar
                            .upgrade()
                            .map(|entry| entry.set_text(""));
                        ctext.clear();
                    }
                    if ctext.is_empty() && current_mode.borrow().as_str() != "all" {
                        let _ = key_actions.search_bar.upgrade().map(|entry| {
                            let _ =
                                entry.activate_action("win.switch-mode", Some(&"all".to_variant()));
                            // apply filter and sorter
                            filter
                                .upgrade()
                                .map(|filter| filter.changed(gtk4::FilterChange::Different));
                            sorter
                                .upgrade()
                                .map(|sorter| sorter.changed(gtk4::SorterChange::Different));
                        });
                    }
                    // Focus first item and check for overflow
                    key_actions.results.upgrade().map(|results| {
                        results.focus_first(
                            Some(&key_actions.context.model),
                            Some(current_mode.clone()),
                        )
                    });
                }
                Key::Return => {
                    key_actions.on_return(key_actions.context.open.get(), None);
                }
                Key::Escape if key_actions.context.open.get() => {
                    key_actions.close_context();
                }
                _ if key.to_unicode().and_then(|c| c.to_digit(10)).is_some() => {
                    if binds
                        .shortcut_modifier
                        .map_or(false, |modifier| mods.contains(modifier))
                    {
                        if let Some(index) = key
                            .name()
                            .and_then(|name| name.parse::<u32>().ok().map(|v| v - 1))
                        {
                            key_actions
                                .results
                                .upgrade()
                                .map(|r| r.execute_by_index(index));
                        }
                    }
                }
                // Pain - solution for shift-tab since gtk handles it as an individual event
                _ if i == 23 && mods.contains(ModifierType::SHIFT_MASK) => {
                    let shift = Some(ModifierType::SHIFT_MASK);
                    let tab = Some(Key::Tab);
                    if binds.prev_mod == shift && binds.prev == tab {
                        key_actions.on_prev();
                    } else if binds.next_mod == shift && binds.next == tab {
                        key_actions.on_next();
                    }
                }
                _ => return false.into(),
            }
            true.into()
        }
    });

    search_bar
        .upgrade()
        .map(|entry| entry.add_controller(event_controller));
}

fn change_event(
    search_bar: WeakRef<Entry>,
    results: WeakRef<ListView>,
    modes: Rc<RefCell<HashMap<String, Option<String>>>>,
    mode: &Rc<RefCell<String>>,
    search_query: &Rc<RefCell<String>>,
) -> Option<()> {
    let search_bar = search_bar.upgrade()?;
    search_bar.connect_changed({
        let mode_clone = Rc::clone(mode);
        let search_query_clone = Rc::clone(search_query);

        move |search_bar| {
            let mut current_text = search_bar.text().to_string();
            // logic to switch to search mode with respective icons
            if current_text.len() == 1 {
                let _ = search_bar.activate_action("win.switch-mode", Some(&"search".to_variant()));
            } else if current_text.len() == 0 && mode_clone.borrow().as_str().trim() == "all" {
                let _ = search_bar.activate_action("win.switch-mode", Some(&"all".to_variant()));
            }
            let trimmed = current_text.trim();
            if !trimmed.is_empty() && modes.borrow().contains_key(&current_text) {
                // Logic to apply modes
                let _ = search_bar.activate_action("win.switch-mode", Some(&trimmed.to_variant()));
                let _ = search_bar.activate_action("win.clear-search", None);
                current_text.clear();
            }
            *search_query_clone.borrow_mut() = current_text.clone();
            // filter and sort
            if let Some(res) = results.upgrade() {
                // To reload ui according to mode
                let _ = res.activate_action("win.update-items", Some(&true.to_variant()));
            }
        }
    });
    Some(())
}
