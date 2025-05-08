use gio::glib::object::ObjectExt;
use gio::glib::subclass::Signal;
use gtk4::subclass::prelude::*;
use gtk4::glib;
use std::cell::RefCell;
use std::sync::OnceLock;

/// ## Fields:
#[derive(Default)]
pub struct EmojiObject {
    pub name: RefCell<String>,
    pub emoji: RefCell<String>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for EmojiObject {
    const NAME: &'static str = "ContextAction";
    type Type = super::EmojiObject;
    type ParentType = glib::Object;
}

// Trait shared by all GObjects
impl ObjectImpl for EmojiObject {
    fn constructed(&self) {
        self.parent_constructed();
    }
    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
        // Signal used to activate actions connected to the Emoji
        SIGNALS.get_or_init(|| vec![Signal::builder("emoji-should-activate").build()])
    }
}
