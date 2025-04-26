mod imp;

use gdk_pixbuf::subclass::prelude::ObjectSubclassIsExt;
use glib::Object;
use gtk4::{glib, prelude::WidgetExt};

glib::wrapper! {
    pub struct SherlockRow(ObjectSubclass<imp::SherlockRow>)
        @extends gtk4::Box, gtk4::Widget;
}

impl SherlockRow {
    pub fn new() -> Self {
         Object::builder().build()
    }
    pub fn set_spawn_focus(&self, focus: bool) {
        self.imp().spawn_focus.set(focus);
    }
    pub fn set_shortcut(&self, shortcut: bool) {
        self.imp().shortcut.set(shortcut);
    }
}

impl Default for SherlockRow {
    fn default() -> Self {
        let row = Self::new();
        row.set_spawn_focus(true);
        row.set_css_classes(&["tile"]);
        row
    }
}
