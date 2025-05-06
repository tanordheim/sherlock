mod imp;

use gdk_pixbuf::subclass::prelude::ObjectSubclassIsExt;
use gio::glib::{object::ObjectExt, SignalHandlerId};
use glib::Object;
use gtk4::glib;

use crate::actions::applaunch::applaunch;

glib::wrapper! {
    pub struct ContextAction(ObjectSubclass<imp::ContextAction>)
        @extends gtk4::Box, gtk4::Widget;
}

impl ContextAction {
    pub fn set_signal_id(&self, signal: SignalHandlerId) {
        // Take the previous signal if it exists and disconnect it
        if let Some(old_id) = self.imp().signal_id.borrow_mut().take() {
            self.disconnect(old_id);
        }
        *self.imp().signal_id.borrow_mut() = Some(signal);
    }
    pub fn new(mod_str:&str, title: &str, exec: String, terminal: bool) -> Self {
        let obj: Self = Object::builder().build();
        let imp = obj.imp();
        if let Some(modkey) = imp.modkey.get().and_then(|w| w.upgrade()){
            modkey.set_text(mod_str);
        }
        if let Some(title_label) = imp.title.get().and_then(|w| w.upgrade()){
            title_label.set_text(title);
        }
        *imp.exec.borrow_mut() = exec.clone();
        imp.terminal.set(terminal);
        let signal_id = obj.connect_local("context-action-should-activate", false, {
            move |_| {
                applaunch(&exec, terminal);
                None
            }});
        *imp.signal_id.borrow_mut() = Some(signal_id);

        obj
    }
}

impl Default for ContextAction {
    fn default() -> Self {
        let row = Self::new("", "", String::new(), false);
        row
    }
}
