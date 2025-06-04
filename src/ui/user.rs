use std::{cell::RefCell, rc::Rc};

use gdk_pixbuf::subclass::prelude::ObjectSubclassIsExt;
use gio::{glib::WeakRef, ListStore};
use gtk4::{
    self,
    gdk::{self, Key, ModifierType},
    prelude::*,
    ApplicationWindow, CustomFilter, CustomSorter, Entry, EventControllerKey, FilterListModel,
    Justification, ListView, Overlay, SignalListItemFactory, SingleSelection, SortListModel,
};
use levenshtein::levenshtein;

use super::{search::SearchUiObj, util::*};
use crate::{
    g_subclasses::sherlock_row::SherlockRow,
    prelude::{SherlockNav, SherlockSearch, ShortCut},
    CONFIG,
};

use super::tiles::{util::TextViewTileBuilder, Tile};
use crate::loader::pipe_loader::PipeData;

pub fn display_pipe(
    _window: &ApplicationWindow,
    pipe_content: Vec<PipeData>,
    method: &str,
    error_model: WeakRef<ListStore>,
) -> (Overlay, SearchHandler) {
    let (search_text, stack_page, ui, handler) = construct(pipe_content, method);
    let imp = ui.imp();

    imp.result_viewport
        .set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic);

    let search_bar = imp.search_bar.downgrade();
    let results = imp.results.downgrade();

    stack_page.connect_realize({
        let search_bar = search_bar.clone();
        move |_| {
            if let Some(entry) = search_bar.upgrade() {
                entry.grab_focus();
            }
        }
    });

    change_event(
        search_bar.clone(),
        results.clone(),
        handler.filter.clone(),
        handler.sorter.clone(),
        &search_text,
    );
    nav_event(results.clone(), search_bar.clone(), handler.binds);

    let handler = SearchHandler::empty(error_model);
    return (stack_page, handler);
}
pub fn display_raw<T: AsRef<str>>(
    content: T,
    center: bool,
    error_model: WeakRef<ListStore>,
) -> (Overlay, SearchHandler) {
    let builder = TextViewTileBuilder::new("/dev/skxxtz/sherlock/ui/text_view_tile.ui");
    builder
        .content
        .as_ref()
        .and_then(|tmp| tmp.upgrade())
        .map(|ctx| {
            let buffer = ctx.buffer();
            ctx.add_css_class("raw_text");
            ctx.set_monospace(true);
            let sanitized: String = content.as_ref().chars().filter(|&c| c != '\0').collect();
            buffer.set_text(&sanitized);
            if center {
                ctx.set_justification(Justification::Center);
            }
        });
    let handler = SearchHandler::empty(error_model);
    let row = builder.object.unwrap_or_default();

    let overlay = Overlay::new();
    overlay.set_child(Some(&row));
    (overlay, handler)
}
fn construct(
    pipe_content: Vec<PipeData>,
    method: &str,
) -> (Rc<RefCell<String>>, Overlay, SearchUiObj, SearchHandler) {
    // Collect Modes
    let custom_binds = ConfKeys::new();
    let search_text = Rc::new(RefCell::new(String::new()));

    // Initialize the builder with the correct path
    let ui = SearchUiObj::new();
    let imp = ui.imp();

    // Setup model and factory
    let model = ListStore::new::<SherlockRow>();
    let factory = make_factory();
    imp.results.set_factory(Some(&factory));

    // Setup selection
    let sorter = make_sorter(&search_text);
    let filter = make_filter(&search_text);
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
    imp.results.set_model(Some(&selection));

    let tiles = Tile::pipe_data(&pipe_content, &method);

    for item in tiles.iter() {
        model.append(item);
    }
    imp.results.set_model(Some(&selection));
    imp.results.set_factory(Some(&factory));
    imp.results.focus_first(None, None);

    // Disable status-bar
    CONFIG.get().map(|c| {
        if !c.appearance.status_bar {
            imp.status_bar.set_visible(false);
        }
    });

    CONFIG.get().map(|c| {
        imp.result_viewport
            .set_size_request((c.appearance.width as f32 * 0.4) as i32, 10);
        imp.search_icon_holder.set_visible(c.appearance.search_icon);
        imp.search_icon.set_pixel_size(c.appearance.icon_size);
        imp.search_icon_back.set_pixel_size(c.appearance.icon_size);
    });

    let handler = SearchHandler::new(
        model.downgrade(),
        WeakRef::new(),
        filter.downgrade(),
        sorter.downgrade(),
        ConfKeys::new(),
    );
    let main_overlay = Overlay::new();
    main_overlay.set_child(Some(&ui));
    (search_text, main_overlay, ui, handler)
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
fn make_filter(search_text: &Rc<RefCell<String>>) -> CustomFilter {
    CustomFilter::new({
        let search_text = Rc::clone(search_text);
        move |entry| {
            let item = entry.downcast_ref::<SherlockRow>().unwrap();
            let current_text = search_text.borrow().clone();
            item.search().fuzzy_match(&current_text)
        }
    })
}
fn make_sorter(search_text: &Rc<RefCell<String>>) -> CustomSorter {
    CustomSorter::new({
        let search_text = Rc::clone(search_text);
        move |item_a, item_b| {
            let search_text = search_text.borrow();

            let item_a = item_a.downcast_ref::<SherlockRow>().unwrap();
            let item_b = item_b.downcast_ref::<SherlockRow>().unwrap();

            let (prio_a, prio_b) = if !search_text.is_empty() {
                (
                    levenshtein(&search_text, &item_a.search()) as f32,
                    levenshtein(&search_text, &item_b.search()) as f32,
                )
            } else {
                (0.0, 0.0)
            };

            prio_a.total_cmp(&prio_b).into()
        }
    })
}
fn nav_event(results: WeakRef<ListView>, search_bar: WeakRef<Entry>, custom_binds: ConfKeys) {
    let event_controller = EventControllerKey::new();
    event_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);
    event_controller.connect_key_pressed({
        let search_bar = search_bar.clone();
        let multi = CONFIG.get().map_or(false, |c| c.runtime.multi);
        fn move_prev(results: &WeakRef<ListView>) {
            results.upgrade().map(|results| results.focus_prev(None));
        }
        fn move_next(results: &WeakRef<ListView>) {
            results.upgrade().map(|results| results.focus_next(None));
        }
        move |_, key, i, modifiers| {
            match key {
                k if Some(k) == custom_binds.prev
                    && custom_binds
                        .prev_mod
                        .map_or(true, |m| modifiers.contains(m)) =>
                {
                    move_prev(&results);
                    return true.into();
                }
                k if Some(k) == custom_binds.next
                    && custom_binds
                        .next_mod
                        .map_or(true, |m| modifiers.contains(m)) =>
                {
                    move_next(&results);
                    return true.into();
                }
                gdk::Key::Up => {
                    move_prev(&results);
                    return true.into();
                }
                gdk::Key::Down => {
                    move_next(&results);
                    return true.into();
                }
                gdk::Key::BackSpace => {
                    if custom_binds
                        .shortcut_modifier
                        .map_or(false, |modifier| modifiers.contains(modifier))
                    {
                        search_bar.upgrade().map(|entry| entry.set_text(""));
                        // Focus first item and check for overflow
                        results
                            .upgrade()
                            .map(|results| results.focus_first(None, None));
                    }
                }
                Key::Return if multi => {
                    if let Some(actives) = results
                        .upgrade()
                        .and_then(|r| r.get_actives::<SherlockRow>())
                    {
                        actives.into_iter().for_each(|row| {
                            let exit: u8 = 0;
                            row.emit_by_name::<()>("row-should-activate", &[&exit]);
                        });
                    }
                }
                gdk::Key::Return => {
                    // Activate action
                    if let Some(upgr) = results.upgrade() {
                        if let Some(row) = upgr.selected_item().and_downcast::<SherlockRow>() {
                            let exit: u8 = 0;
                            row.emit_by_name::<()>("row-should-activate", &[&exit]);
                        }
                    }
                }
                _ if key.to_unicode().and_then(|c| c.to_digit(10)).is_some() => {
                    if custom_binds
                        .shortcut_modifier
                        .map_or(false, |modifier| modifiers.contains(modifier))
                    {
                        if let Some(index) = key
                            .name()
                            .and_then(|name| name.parse::<u32>().ok().map(|v| v - 1))
                        {
                            results.upgrade().map(|r| r.execute_by_index(index));
                            return true.into();
                        }
                    }
                }
                Key::Tab if multi => {
                    results.upgrade().map(|r| r.mark_active());
                    return true.into();
                }
                // Pain - solution for shift-tab since gtk handles it as an individual event
                _ if i == 23 && modifiers.contains(ModifierType::SHIFT_MASK) => {
                    let shift = Some(ModifierType::SHIFT_MASK);
                    let tab = Some(Key::Tab);
                    if custom_binds.prev_mod == shift && custom_binds.prev == tab {
                        move_prev(&results);
                        return true.into();
                    } else if custom_binds.next_mod == shift && custom_binds.next == tab {
                        move_next(&results);
                        return true.into();
                    }
                }
                _ => (),
            }
            false.into()
        }
    });

    search_bar
        .upgrade()
        .map(|entry| entry.add_controller(event_controller));
}

fn change_event(
    search_bar: WeakRef<Entry>,
    results: WeakRef<ListView>,
    filter: WeakRef<CustomFilter>,
    sorter: WeakRef<CustomSorter>,
    search_query: &Rc<RefCell<String>>,
) -> Option<()> {
    let search_bar = search_bar.upgrade()?;
    search_bar.connect_changed({
        let search_query_clone = Rc::clone(search_query);

        move |search_bar| {
            let current_text = search_bar.text().to_string();
            // logic to switch to search mode with respective icons
            if current_text.len() == 1 {
                let _ = search_bar.activate_action("win.switch-mode", Some(&"search".to_variant()));
            } else if current_text.len() == 0 {
                let _ = search_bar.activate_action("win.switch-mode", Some(&"all".to_variant()));
            }

            // filter and sort
            {
                *search_query_clone.borrow_mut() = current_text;
            }
            filter
                .upgrade()
                .map(|filter| filter.changed(gtk4::FilterChange::Different));
            sorter
                .upgrade()
                .map(|sorter| sorter.changed(gtk4::SorterChange::Different));
            // focus first item
            results
                .upgrade()
                .map(|results| results.focus_first(None, None));
        }
    });
    Some(())
}
