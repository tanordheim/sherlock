use gdk_pixbuf::subclass::prelude::ObjectSubclassIsExt;
use gio::glib::WeakRef;
use gio::ListStore;
use gtk4::{self, gdk::Key, prelude::*, Builder, EventControllerKey};
use gtk4::{
    Box as GtkBox, CustomFilter, CustomSorter, Entry, FilterListModel, GridView, Label, Ordering,
    ScrolledWindow, SignalListItemFactory, SingleSelection, SortListModel,
};
use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::rc::Rc;

use crate::g_subclasses::emoji_item::{EmojiObject, EmojiRaw};
use crate::prelude::{SherlockNav, SherlockSearch};
use crate::sherlock_error;
use crate::ui::util::ConfKeys;
use crate::utils::errors::{SherlockError, SherlockErrorType};

pub struct EmojiPicker {
    emojies: Vec<EmojiObject>,
}
impl EmojiPicker {
    pub fn new() -> Result<Self, SherlockError> {
        // Loads default fallback.json file and loads the launcher configurations within.
        let data = gio::resources_lookup_data(
            "/dev/skxxtz/sherlock/emojies.json",
            gio::ResourceLookupFlags::NONE,
        )
        .map_err(|e| {
            sherlock_error!(
                SherlockErrorType::ResourceLookupError("emojies.json".to_string()),
                e.to_string()
            )
        })?;
        let string_data = std::str::from_utf8(&data)
            .map_err(|e| {
                sherlock_error!(
                    SherlockErrorType::FileParseError(PathBuf::from("emojies.json")),
                    e.to_string()
                )
            })?
            .to_string();
        let emojies: Vec<EmojiRaw> = serde_json::from_str(&string_data).map_err(|e| {
            sherlock_error!(
                SherlockErrorType::FileParseError(PathBuf::from("emojies.json")),
                e.to_string()
            )
        })?;
        let emojies: Vec<EmojiObject> = emojies
            .into_iter()
            .map(|emj| EmojiObject::from(emj))
            .collect();
        Ok(Self { emojies })
    }
}

pub struct EmojiUI {
    model: WeakRef<ListStore>,
    view: WeakRef<GridView>,
    search_bar: WeakRef<Entry>,
    sorter: WeakRef<CustomSorter>,
    filter: WeakRef<CustomFilter>,
}

pub fn emojies(
    stack_page: &Rc<RefCell<String>>,
) -> Result<(GtkBox, WeakRef<ListStore>), SherlockError> {
    let (search_query, stack, ui) = construct()?;

    stack.connect_realize({
        let search_bar = ui.search_bar.clone();
        move |_| {
            // Focus search bar as soon as it's visible
            search_bar
                .upgrade()
                .map(|search_bar| search_bar.grab_focus());
        }
    });

    let custom_binds = ConfKeys::new();
    nav_event(
        ui.search_bar.clone(),
        ui.view.clone(),
        stack_page,
        custom_binds,
    );
    change_event(
        ui.search_bar.clone(),
        &search_query,
        ui.sorter.clone(),
        ui.filter.clone(),
        ui.view.clone(),
    );
    return Ok((stack, ui.model.clone()));
}
fn nav_event(
    search_bar: WeakRef<Entry>,
    view: WeakRef<GridView>,
    stack_page: &Rc<RefCell<String>>,
    custom_binds: ConfKeys,
) {
    // Wrap the event controller in an Rc<RefCell> for shared mutability
    let event_controller = EventControllerKey::new();
    let stack_page = Rc::clone(&stack_page);

    event_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);
    event_controller.connect_key_pressed({
        fn move_prev(view: &WeakRef<GridView>) {
            view.upgrade().map(|view| view.focus_prev(None));
        }
        fn move_next(view: &WeakRef<GridView>) {
            view.upgrade().map(|view| view.focus_next(None));
        }
        fn move_up(view: &WeakRef<GridView>) {
            view.upgrade().map(|view| {
                let width = view.width();
                let offset = (width / 50).min(10);
                view.focus_offset(None, -offset)
            });
        }
        fn move_down(view: &WeakRef<GridView>) {
            view.upgrade().map(|view| {
                let width = view.width();
                let offset = (width / 50).min(10);
                view.focus_offset(None, offset)
            });
        }
        move |_, key, _, modifiers| {
            if stack_page.borrow().as_str() != "search-page" {
                return false.into();
            }
            match key {
                Key::Tab => {
                    if let Some(view) = view.upgrade() {
                        let first = view.selected_item().and_downcast::<EmojiObject>();
                        if let Some(first) = first {
                            if let Some(parent) = first.imp().parent.take().and_then(|p| p.upgrade()) {
                                view.set_model(None::<SingleSelection>.as_ref());
                                view.set_factory(None::<SignalListItemFactory>.as_ref());
                                println!("refcount after clearing {:?} - should be 1", parent.ref_count());
                            }

                        }


                    }
                    true.into()
                }
                // Custom previous key
                k if Some(k) == custom_binds.prev
                    && custom_binds
                        .prev_mod
                        .map_or(true, |m| modifiers.contains(m)) =>
                {
                    move_prev(&view);
                    return true.into();
                }
                // Custom next key
                k if Some(k) == custom_binds.next
                    && custom_binds
                        .next_mod
                        .map_or(true, |m| modifiers.contains(m)) =>
                {
                    move_next(&view);
                    return true.into();
                }
                Key::Up => {
                    move_up(&view);
                    return true.into();
                }
                Key::Down => {
                    move_down(&view);
                    return true.into();
                }
                Key::Left => {
                    move_prev(&view);
                    return true.into();
                }
                Key::Right => {
                    move_next(&view);
                    return true.into();
                }
                Key::Return => {
                    if let Some(upgr) = view.upgrade() {
                        if let Some(selection) = upgr.model().and_downcast::<SingleSelection>() {
                            if let Some(row) =
                                selection.selected_item().and_downcast::<EmojiObject>()
                            {
                                row.emit_by_name::<()>("emoji-should-activate", &[]);
                            }
                        }
                    }
                    true.into()
                }
                _ => false.into(),
            }
        }
    });
    search_bar
        .upgrade()
        .map(|entry| entry.add_controller(event_controller));
}

fn construct() -> Result<(Rc<RefCell<String>>, GtkBox, EmojiUI), SherlockError> {
    let emojies = EmojiPicker::new()?.emojies;
    let search_text = Rc::new(RefCell::new(String::new()));
    // Initialize the builder with the correct path
    let builder = Builder::from_resource("/dev/skxxtz/sherlock/ui/search.ui");

    // Get the required object references
    let vbox: GtkBox = builder.object("vbox").unwrap();
    let view_port: ScrolledWindow = builder.object("scrolled-window").unwrap();
    let search_bar: Entry = builder.object("search-bar").unwrap();

    // Setup model and factory
    let model = ListStore::new::<EmojiObject>();
    let model_ref = model.downgrade();

    let sorter = make_sorter(&search_text);
    let filter = make_filter(&search_text);
    let filter_model = FilterListModel::new(Some(model.clone()), Some(filter.clone()));
    let sorted_model = SortListModel::new(Some(filter_model), Some(sorter.clone()));

    let factory = make_factory();
    let selection = SingleSelection::new(Some(sorted_model));
    let view: GridView = GridView::new(Some(selection), Some(factory));

    model.extend_from_slice(&emojies);

    view.set_max_columns(10);
    view_port.set_child(Some(&view));

    let ui = EmojiUI {
        model: model_ref.clone(),
        view: view.downgrade(),
        search_bar: search_bar.downgrade(),
        sorter: sorter.downgrade(),
        filter: filter.downgrade(),
    };

    Ok((search_text, vbox, ui))
}
fn make_factory() -> SignalListItemFactory {
    let factory = SignalListItemFactory::new();
    factory.connect_setup(move |_factory, item| {
        let list_item = item
            .downcast_ref::<gtk4::ListItem>()
            .expect("Should be a list item");
        let box_ = GtkBox::new(gtk4::Orientation::Vertical, 0);
        box_.set_size_request(50, 50);

        let emoji_label = Label::new(Some(""));
        emoji_label.set_valign(gtk4::Align::Center);
        emoji_label.set_halign(gtk4::Align::Center);
        emoji_label.set_vexpand(true);
        box_.append(&emoji_label);

        list_item.set_child(Some(&box_));
    });
    factory.connect_bind(|_, item| {
        let item = item
            .downcast_ref::<gtk4::ListItem>()
            .expect("Item mut be a ListItem");
        let emoji_obj = item
            .item()
            .and_downcast::<EmojiObject>()
            .expect("Inner should be an EmojiObject");
        let box_ = item
            .child()
            .and_downcast::<GtkBox>()
            .expect("The child should be a Box");
        emoji_obj.set_parent(box_.downgrade());
        emoji_obj.attach_event();

        let emoji_label = box_
            .first_child()
            .and_downcast::<Label>()
            .expect("First child should be a label");

        emoji_label.set_label(&emoji_obj.emoji());
    });
    factory.connect_unbind(move |_, item| {
        let item = item
            .downcast_ref::<gtk4::ListItem>()
            .expect("Item mut be a ListItem");
        let emoji_obj = item
            .item()
            .and_downcast::<EmojiObject>()
            .expect("Inner should be an EmojiObject");
        let box_ = item
            .child()
            .and_downcast::<GtkBox>()
            .expect("The child should be a Box");
        emoji_obj.clean();

        let emoji_label = box_
            .first_child()
            .and_downcast::<Label>()
            .expect("First child should be a label");

        emoji_label.set_label("");
    });
    factory
}

fn change_event(
    search_bar: WeakRef<Entry>,
    search_query: &Rc<RefCell<String>>,
    sorter: WeakRef<CustomSorter>,
    filter: WeakRef<CustomFilter>,
    view: WeakRef<GridView>,
) -> Option<()> {
    let search_bar = search_bar.upgrade()?;
    search_bar.connect_changed({
        let search_query_clone = Rc::clone(search_query);

        move |search_bar| {
            let current_text = search_bar.text().to_string();
            *search_query_clone.borrow_mut() = current_text.clone();
            sorter
                .upgrade()
                .map(|sorter| sorter.changed(gtk4::SorterChange::Different));
            filter
                .upgrade()
                .map(|filter| filter.changed(gtk4::FilterChange::Different));
            view.upgrade().map(|view| view.focus_first(None));
        }
    });
    Some(())
}
fn make_filter(search_text: &Rc<RefCell<String>>) -> CustomFilter {
    let counter: Rc<Cell<u16>> = Rc::new(Cell::new(0));
    let filter = CustomFilter::new({
        let search_text = Rc::clone(search_text);
        let counter = Rc::clone(&counter);
        move |entry| {
            let current = counter.get();
            if current >= 80 {
                return false;
            }
            let item = entry.downcast_ref::<EmojiObject>().unwrap();
            let current_text = search_text.borrow().clone();
            if item.title().fuzzy_match(&current_text) {
                counter.set(current + 1);
                return true;
            }
            false
        }
    });
    filter.connect_changed({
        let counter = Rc::clone(&counter);
        move |_, _| counter.set(0)
    });
    filter
}
fn make_sorter(search_text: &Rc<RefCell<String>>) -> CustomSorter {
    CustomSorter::new({
        let search_text = Rc::clone(search_text);
        move |item_a, item_b| {
            let search_text = search_text.borrow();

            let item_a = item_a.downcast_ref::<EmojiObject>().unwrap();
            let item_b = item_b.downcast_ref::<EmojiObject>().unwrap();

            let priority_a = levenshtein::levenshtein(&item_a.title(), &search_text) as f32;
            let priority_b = levenshtein::levenshtein(&item_b.title(), &search_text) as f32;

            if !search_text.is_empty() {
                return Ordering::Equal;
            }

            priority_a.total_cmp(&priority_b).into()
        }
    })
}
