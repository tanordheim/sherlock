use gtk4::glib;
use gtk4::subclass::prelude::*;

#[derive(Default)]
pub struct SherlockInput {}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for SherlockInput {
    const NAME: &'static str = "SherlockCustomSearchInput";
    type Type = super::SherlockInput;
    type ParentType = gtk4::Entry;
}

impl ObjectImpl for SherlockInput {}
impl WidgetImpl for SherlockInput {}
impl EditableImpl for SherlockInput {}
impl EntryImpl for SherlockInput {}
