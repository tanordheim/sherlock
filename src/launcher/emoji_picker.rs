use gio::glib::WeakRef;
use gio::ListStore;
use gtk4::{self, gdk::Key, prelude::*, Builder, EventControllerKey};
use gtk4::{
    Box as GtkBox, Entry, GridView, Label, ScrolledWindow, SignalListItemFactory, SingleSelection,
};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use crate::g_subclasses::emoji_item::{EmojiObject, EmojiRaw};
use crate::sherlock_error;
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
            .take(100)
            .map(|emj| EmojiObject::from(emj))
            .collect();
        Ok(Self { emojies })
    }
}

pub struct EmojiUI {
    model: WeakRef<ListStore>,
    view: WeakRef<GridView>,
    search_bar: WeakRef<Entry>,
}

pub fn emojies(
    stack_page: &Rc<RefCell<String>>,
) -> Result<(GtkBox, WeakRef<ListStore>), SherlockError> {
    let emoji_picker = EmojiPicker::new()?;
    let (stack, ui) = construct(emoji_picker.emojies);

    nav_event(ui.search_bar.clone(), ui.view.clone(), stack_page);
    return Ok((stack, ui.model.clone()));
}
fn nav_event(
    search_bar: WeakRef<Entry>,
    view: WeakRef<GridView>,
    stack_page: &Rc<RefCell<String>>,
) {
    // Wrap the event controller in an Rc<RefCell> for shared mutability
    let event_controller = EventControllerKey::new();
    let stack_page = Rc::clone(&stack_page);

    event_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);
    event_controller.connect_key_pressed(move |_, key, _, _| {
        if stack_page.borrow().as_str() != "search-page" {
            return false.into();
        }
        match key {
            Key::Return => {
                if let Some(upgr) = view.upgrade() {
                    if let Some(selection) = upgr.model().and_downcast::<SingleSelection>() {
                        if let Some(row) = selection.selected_item().and_downcast::<EmojiObject>() {
                            row.emit_by_name::<()>("emoji-should-activate", &[]);
                        }
                    }
                }
                true.into()
            }
            _ => false.into(),
        }
    });
    search_bar
        .upgrade()
        .map(|entry| entry.add_controller(event_controller));
}

fn construct(emojies: Vec<EmojiObject>) -> (GtkBox, EmojiUI) {
    // Initialize the builder with the correct path
    let builder = Builder::from_resource("/dev/skxxtz/sherlock/ui/search.ui");

    // Get the required object references
    let vbox: GtkBox = builder.object("vbox").unwrap();
    let view_port: ScrolledWindow = builder.object("scrolled-window").unwrap();
    let search_bar: Entry = builder.object("search-bar").unwrap();

    // Setup model and factory
    let model = ListStore::new::<EmojiObject>();
    emojies.iter().for_each(|emj| model.append(emj));
    let factory = make_factory();
    let selection = SingleSelection::new(Some(model.clone()));
    let view: GridView = GridView::new(Some(selection), Some(factory));
    view.set_max_columns(999);

    view_port.set_child(Some(&view));

    let ui = EmojiUI {
        model: model.downgrade(),
        view: view.downgrade(),
        search_bar: search_bar.downgrade(),
    };

    (vbox, ui)
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

        emoji_label.set_text(&emoji_obj.emoji());
    });
    factory
}
