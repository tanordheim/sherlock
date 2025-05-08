use gio::glib::subclass::Signal;
use gio::glib::{SignalHandlerId, WeakRef};
use gtk4::glib;
use gtk4::subclass::prelude::*;
use std::cell::RefCell;
use std::sync::OnceLock;

/// ## Fields:
#[derive(Default)]
pub struct EmojiObject {
    pub title: RefCell<String>,
    pub emoji: RefCell<String>,
    pub signal_id: RefCell<Option<SignalHandlerId>>,
    pub parent: RefCell<WeakRef<gtk4::Box>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for EmojiObject {
    const NAME: &'static str = "EmojiObject";
    type Type = super::EmojiObject;
    type ParentType = gtk4::Widget;
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
impl WidgetImpl for EmojiObject {}
impl BoxImpl for EmojiObject {}
