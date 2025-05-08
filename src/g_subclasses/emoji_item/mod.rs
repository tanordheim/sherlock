mod imp;

use glib::Object;
use gtk4::glib;

glib::wrapper! {
    pub struct EmojiObject(ObjectSubclass<imp::EmojiObject>);
}

impl EmojiObject {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
