use gio::glib::WeakRef;
use gio::ListStore;
use gtk4::{self, gdk::Key, prelude::*, Builder, EventControllerKey};
use gtk4::{Box as GtkBox, GridView, ListView, SignalListItemFactory, SingleSelection};
use std::cell::RefCell;
use std::rc::Rc;

use crate::g_subclasses::emoji_item::EmojiObject;
use crate::g_subclasses::sherlock_row::SherlockRow;

pub struct EmojiPicker {
    emojies: Vec<EmojiObject>
}
impl EmojiPicker {
    pub fn new()-> Self {
        //load emojies here
        Self { emojies: vec![] }
    }
}

pub struct EmojiUI {
    model: WeakRef<ListStore>,
    view: WeakRef<GridView>,
}


pub fn emojies(stack_page: &Rc<RefCell<String>>) -> (GtkBox, WeakRef<ListStore>) {
    let emoji_picker = EmojiPicker::new();
    let (stack,  ui) = construct(emoji_picker.emojies);


    nav_event(&stack, ui.view.clone(), stack_page);
    return (stack, ui.model.clone());
}
fn nav_event(stack: &GtkBox, result_holder: WeakRef<GridView>, stack_page: &Rc<RefCell<String>>) {
    // Wrap the event controller in an Rc<RefCell> for shared mutability
    let event_controller = EventControllerKey::new();
    let stack_page = Rc::clone(&stack_page);

    event_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);
    event_controller.connect_key_pressed(move |_, key, _, _| {
        if stack_page.borrow().as_str() != "emoji-page" {
            return false.into();
        }
        match key {
            Key::Return => {
                let _ = result_holder.upgrade().map(|widget| {
                    widget.activate_action(
                        "win.switch-page",
                        Some(&String::from("error-page->search-page").to_variant()),
                    )
                });
                true.into()
            }
            _ => false.into(),
        }
    });
    stack.add_controller(event_controller);
}

fn construct(emojies: Vec<EmojiObject>) -> (GtkBox, EmojiUI) {
    // Initialize the builder with the correct path
    let builder = Builder::from_resource("/dev/skxxtz/sherlock/ui/error_view.ui");

    // Get the required object references
    let vbox: GtkBox = builder.object("vbox").unwrap();

    // Setup model and factory
    let model = ListStore::new::<EmojiObject>();
    let factory = make_factory();
    let selection = SingleSelection::new(Some(model.clone()));
    let view: GridView = GridView::new(Some(selection), Some(factory));
    view.set_focusable(false);

    // breaking_error_tiles
    //     .into_iter()
    //     .for_each(|tile| model.append(&tile));

    let ui = EmojiUI {
        model: model.downgrade(),
        view: view.downgrade(),
    };

    (vbox, ui)
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
struct ErrorUI {
    model: WeakRef<ListStore>,
    results: WeakRef<ListView>,
}
