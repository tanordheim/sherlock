use gtk4::glib;
use gtk4::subclass::prelude::*;
use std::cell::Cell;
// Object holding the state
#[derive(Default)]
pub struct SherlockRow {
    pub spawn_focus: Cell<bool>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for SherlockRow {
    const NAME: &'static str = "MyGtkAppCustomButton";
    type Type = super::SherlockRow;
    type ParentType = gtk4::ListBoxRow;
}

// Trait shared by all GObjects
impl ObjectImpl for SherlockRow {}

// Trait shared by all widgets
impl WidgetImpl for SherlockRow {}

// Trait shared by all buttons
impl ListBoxRowImpl for SherlockRow {}
