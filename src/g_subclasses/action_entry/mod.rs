mod imp;

use gdk_pixbuf::subclass::prelude::ObjectSubclassIsExt;
use gio::glib::{object::ObjectExt, SignalHandlerId};
use glib::Object;
use gtk4::glib;

use crate::{
    actions::{execute_from_attrs, get_attrs_map},
    loader::util::ApplicationAction,
    ui::tiles::util::IconComp,
};

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
    pub fn new(mod_str: &str, action: &ApplicationAction, terminal: bool) -> Self {
        let obj: Self = Object::builder().build();
        let imp = obj.imp();
        if let Some(modkey) = imp.modkey.get().and_then(|w| w.upgrade()) {
            modkey.set_text(mod_str);
        }
        if let Some(title_label) = imp.title.get().and_then(|w| w.upgrade()) {
            if let Some(title) = &action.name {
                title_label.set_text(&title);
            }
        }
        imp.icon
            .get()
            .and_then(|tmp| tmp.upgrade())
            .map(|icon| icon.set_icon(&action.icon, &None, &None));
        if let Some(exec) = &action.exec {
            let signal_id = obj.connect_local("context-action-should-activate", false, {
                let exec = exec.clone();
                move |row| {
                    let row = row.first().map(|f| f.get::<ContextAction>().ok())??;
                    let attrs = get_attrs_map(vec![
                        ("method", "app_launcher"),
                        ("exec", &exec),
                        ("term", &terminal.to_string()),
                    ]);
                    execute_from_attrs(&row, &attrs);
                    None
                }
            });
            *imp.signal_id.borrow_mut() = Some(signal_id);
        }

        obj
    }
}
