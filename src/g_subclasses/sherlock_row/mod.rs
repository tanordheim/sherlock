mod imp;

use gdk_pixbuf::subclass::prelude::ObjectSubclassIsExt;
use glib::Object;
use gtk4::{glib, prelude::WidgetExt};

use crate::launcher::Launcher;

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
    pub fn set_search(&self, search: &str) {
        *self.imp().search.borrow_mut() = search.to_string();
    }
    pub fn set_priority(&self, prio: f32) {
        self.imp().priority.set(prio);
    }
    pub fn set_alias(&self, mode: &str) {
        *self.imp().alias.borrow_mut() = mode.to_string();
    }
    pub fn set_home(&self, home: bool) {
        self.imp().home.set(home);
    }
    pub fn set_only_home(&self, home: bool) {
        self.imp().only_home.set(home);
    }

    pub fn alias(&self) -> String {
        self.imp().alias.borrow().clone()
    }
    pub fn search(&self) -> String {
        self.imp().search.borrow().clone()
    }
    pub fn priority(&self) -> f32 {
        self.imp().priority.get()
    }
    pub fn home(&self) -> (bool, bool) {
        let only_home = self.imp().only_home.get();
        let home = self.imp().home.get();
        (home, only_home)
    }
    pub fn with_launcher(&self, launcher: &Launcher) {
        self.set_only_home(launcher.only_home);
        self.set_home(launcher.home);
        self.set_shortcut(launcher.shortcut);
        self.set_spawn_focus(launcher.spawn_focus);
        if let Some(alias) = &launcher.alias {
            self.set_alias(alias);
        }
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
