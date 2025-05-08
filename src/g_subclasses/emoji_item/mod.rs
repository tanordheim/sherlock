mod imp;

use gdk_pixbuf::subclass::prelude::ObjectSubclassIsExt;
use glib::Object;
use gtk4::glib;
use serde::Deserialize;

glib::wrapper! {
    pub struct EmojiObject(ObjectSubclass<imp::EmojiObject>);
}
/// For deserialization
#[derive(Deserialize)]
pub struct EmojiRaw {
    emoji: String,
    name: String,
}

impl EmojiObject {
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
