mod imp;

use glib::Object;
use gtk4::glib;

glib::wrapper! {
    pub struct SherlockInput(ObjectSubclass<imp::SherlockInput>)
        @extends gtk4::Entry, gtk4::Widget,
        @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget, gtk4::Editable;
}

impl SherlockInput {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
