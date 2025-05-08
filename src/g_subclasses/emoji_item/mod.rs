mod imp;

use gdk_pixbuf::subclass::prelude::ObjectSubclassIsExt;
use gio::glib::{object::ObjectExt, SignalHandlerId, WeakRef};
use glib::Object;
use gtk4::{
    glib,
    prelude::{GestureSingleExt, WidgetExt},
    Box, GestureClick,
};
use serde::Deserialize;

use crate::actions::{execute_from_attrs, get_attrs_map};

glib::wrapper! {
    pub struct EmojiObject(ObjectSubclass<imp::EmojiObject>)
        @extends gtk4::Box;
}
/// For deserialization
#[derive(Deserialize)]
pub struct EmojiRaw {
    emoji: String,
    name: String,
}

impl EmojiObject {
    // Setters
    pub fn set_parent(&self, parent: WeakRef<Box>) {
        *self.imp().parent.borrow_mut() = parent;
    }
    pub fn set_signal_id(&self, signal: SignalHandlerId) {
        // Take the previous signal if it exists and disconnect it
        if let Some(old_id) = self.imp().signal_id.borrow_mut().take() {
            self.disconnect(old_id);
        }
        *self.imp().signal_id.borrow_mut() = Some(signal);
    }
    pub fn attach_event(&self) {
        let imp = self.imp();
        let parent = imp.parent.borrow().clone();
        let signal_id = self.connect_local("emoji-should-activate", false, {
            let parent = parent.clone();
            let emoji = self.emoji();
            move |_attrs| {
                let attrs = get_attrs_map(vec![("method", Some("copy")), ("result", Some(&emoji))]);

                let parent = parent.upgrade()?;
                execute_from_attrs(&parent, &attrs);
                None
            }
        });
        imp.gesture.get_or_init(|| {
            let gesture = GestureClick::new();
            let parent = parent.clone();
            let myself = self.downgrade();
            gesture.set_button(0);
            gesture.connect_pressed({
                move |_, n_clicks, _, _| {
                    if n_clicks >= 2 {
                        if let Some(obj) = myself.upgrade() {
                            obj.emit_by_name::<()>("emoji-should-activate", &[]);
                        }
                    }
                }
            });
            parent
                .upgrade()
                .map(|tmp| tmp.add_controller(gesture.clone()));
            gesture
        });
        self.set_signal_id(signal_id);
    }

    // Getters
    pub fn title(&self) -> String {
        self.imp().title.borrow().to_string()
    }
    pub fn emoji(&self) -> String {
        self.imp().emoji.borrow().to_string()
    }

    pub fn from(emoji_data: EmojiRaw) -> Self {
        let obj: Self = Object::builder().build();
        let imp = obj.imp();

        *imp.title.borrow_mut() = emoji_data.name;
        *imp.emoji.borrow_mut() = emoji_data.emoji;
        obj
    }
    pub fn new() -> Self {
        Object::builder().build()
    }
}
