use std::u32;

use gdk_pixbuf::subclass::prelude::ObjectSubclassIsExt;
use gio::glib::{self, WeakRef};
use gio::ListStore;
use gtk4::gdk::{Key, ModifierType};
use gtk4::{prelude::*, Box as HVBox, Label, ListScrollFlags, ListView, SingleSelection};

use crate::g_subclasses::sherlock_row::SherlockRow;
use crate::utils::config::default_modkey_ascii;
use crate::CONFIG;

pub trait ShortCut {
    fn apply_shortcut(&self, index: i32, mod_str: &str) -> i32;
    fn remove_shortcut(&self) -> i32;
}
impl ShortCut for HVBox {
    fn apply_shortcut(&self, index: i32, mod_str: &str) -> i32 {
        if index < 6 {
            if let Some(child) = self.first_child() {
                if let Some(label) = child.downcast_ref::<Label>() {
                    self.set_visible(true);
                    label.set_text(&format!("{}", mod_str));
                }
            }
            if let Some(child) = self.last_child() {
                if let Some(label) = child.downcast_ref::<Label>() {
                    self.set_visible(true);
                    label.set_text(&format!("{}", index));
                    return 1;
                }
            }
        }
        return 0;
    }
    fn remove_shortcut(&self) -> i32 {
        let r = if self.is_visible() { 1 } else { 0 };
        self.set_visible(false);
        r
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConfKeys {
    // Next
    pub next: Option<Key>,
    pub next_mod: Option<ModifierType>,
    // Previous
    pub prev: Option<Key>,
    pub prev_mod: Option<ModifierType>,
    // ContextMenu
    pub context: Option<Key>,
    pub context_mod: Option<ModifierType>,
    pub context_str: Option<String>,
    pub context_mod_str: String,
    // Shortcuts
    pub shortcut_modifier: Option<ModifierType>,
    pub shortcut_modifier_str: String,
}
impl ConfKeys {
    pub fn new() -> Self {
        if let Some(c) = CONFIG.get() {
            let (prev_mod, prev) = match &c.binds.prev {
                Some(prev) => ConfKeys::eval_bind_combination(prev),
                _ => (None, (None, None)),
            };
            let (next_mod, next) = match &c.binds.next {
                Some(next) => ConfKeys::eval_bind_combination(next),
                _ => (None, (None, None)),
            };
            let (context_mod, context) = match &c.binds.context {
                Some(context) => ConfKeys::eval_bind_combination(context),
                _ => (None, (None, None)),
            };
            let shortcut_modifier = match &c.binds.modifier {
                Some(shortcut) => ConfKeys::eval_mod(shortcut),
                _ => Some(ModifierType::CONTROL_MASK),
            };
            let shortcut_modifier_str = ConfKeys::get_mod_str(&shortcut_modifier);
            let context_mod_str = ConfKeys::get_mod_str(&context_mod);
            return ConfKeys {
                next: next.0,
                next_mod,
                prev: prev.0,
                prev_mod,
                context: context.0,
                context_mod,
                context_str: context.1,
                context_mod_str,
                shortcut_modifier,
                shortcut_modifier_str,
            };
        }
        ConfKeys::empty()
    }
    pub fn empty() -> Self {
        ConfKeys {
            next: None,
            next_mod: None,
            prev: None,
            prev_mod: None,
            context: None,
            context_mod: None,
            context_mod_str: String::new(),
            context_str: None,
            shortcut_modifier: None,
            shortcut_modifier_str: String::new(),
        }
    }
    fn eval_bind_combination<T: AsRef<str>>(
        key: T,
    ) -> (Option<ModifierType>, (Option<Key>, Option<String>)) {
        let key_str = key.as_ref();
        match key_str.split("-").collect::<Vec<&str>>().as_slice() {
            [modifier, key, ..] => (ConfKeys::eval_mod(modifier), ConfKeys::eval_key(key)),
            [key, ..] => (None, ConfKeys::eval_key(key)),
            _ => (None, (None, None)),
        }
    }
    fn eval_key<T: AsRef<str>>(key: T) -> (Option<Key>, Option<String>) {
        match key.as_ref().to_ascii_lowercase().as_ref() {
            "tab" => (Some(Key::Tab), Some(String::from("⇥"))),
            "up" => (Some(Key::Up), Some(String::from("↑"))),
            "down" => (Some(Key::Down), Some(String::from("↓"))),
            "left" => (Some(Key::Left), Some(String::from("←"))),
            "right" => (Some(Key::Right), Some(String::from("→"))),
            "pgup" => (Some(Key::Page_Up), Some(String::from("⇞"))),
            "pgdown" => (Some(Key::Page_Down), Some(String::from("⇟"))),
            "end" => (Some(Key::End), Some(String::from("End"))),
            "home" => (Some(Key::Home), Some(String::from("Home"))),
            // Alphabet
            k if k.len() == 1 && k.chars().all(|c| c.is_ascii_alphabetic()) => {
                (Key::from_name(k), Some(k.to_uppercase()))
            }
            _ => (None, None),
        }
    }
    fn eval_mod(key: &str) -> Option<ModifierType> {
        match key {
            "shift" => Some(ModifierType::SHIFT_MASK),
            "control" => Some(ModifierType::CONTROL_MASK),
            "alt" => Some(ModifierType::ALT_MASK),
            "super" => Some(ModifierType::SUPER_MASK),
            "lock" => Some(ModifierType::LOCK_MASK),
            "hypr" => Some(ModifierType::HYPER_MASK),
            "meta" => Some(ModifierType::META_MASK),
            _ => None,
        }
    }
    fn get_mod_str(mod_key: &Option<ModifierType>) -> String {
        let strings = CONFIG
            .get()
            .and_then(|c| {
                let s = &c.appearance.mod_key_ascii;
                if s.len() == 8 {
                    Some(s.clone())
                } else {
                    None
                }
            })
            .unwrap_or_else(default_modkey_ascii);

        let k = match mod_key {
            Some(ModifierType::SHIFT_MASK) => 0,
            Some(ModifierType::LOCK_MASK) => 1,
            Some(ModifierType::CONTROL_MASK) => 2,
            Some(ModifierType::META_MASK) => 3,
            Some(ModifierType::ALT_MASK) => 4,
            Some(ModifierType::SUPER_MASK) => 5,
            Some(ModifierType::HYPER_MASK) => 6,
            _ => 7,
        };
        strings.get(k).cloned().unwrap_or(String::from("modkey"))
    }
}

pub trait SherlockNav {
    fn focus_next(&self, context_model: Option<&WeakRef<ListStore>>) -> Option<()>;
    fn focus_prev(&self, context_model: Option<&WeakRef<ListStore>>) -> Option<()>;
    fn focus_first(&self, context_model: Option<&WeakRef<ListStore>>) -> Option<()>;
    fn execute_by_index(&self, index: u32);
    fn selected_item(&self) -> Option<glib::Object>;
    fn get_weaks(&self) -> Option<Vec<WeakRef<SherlockRow>>>;
}
impl SherlockNav for ListView {
    fn focus_next(&self, context_model: Option<&WeakRef<ListStore>>) -> Option<()> {
        let selection = self.model().and_downcast::<SingleSelection>()?;
        let index = selection.selected();
        if index == u32::MAX {
            return None;
        }
        let n_items = selection.n_items();
        let new_index = index + 1;
        if new_index < n_items {
            selection.set_selected(new_index);
            self.scroll_to(new_index, ListScrollFlags::NONE, None);
            let selected = selection.selected_item().and_downcast::<SherlockRow>()?;
            let _ = self.activate_action(
                "win.context-mode",
                Some(&(selected.num_actions() > 0).to_variant()),
            );
        }
        context_model
            .and_then(|tmp| tmp.upgrade())
            .map(|ctx| ctx.remove_all());
        None
    }
    fn focus_prev(&self, context_model: Option<&WeakRef<ListStore>>) -> Option<()> {
        let selection = self.model().and_downcast::<SingleSelection>()?;
        let index = selection.selected();
        let n_items = selection.n_items();
        let new_index = if index > 0 {
            selection.set_selected(index - 1);
            index - 1
        } else {
            index
        };
        if new_index != index {
            if new_index < n_items {
                self.scroll_to(new_index, ListScrollFlags::NONE, None);
                let selected = selection.selected_item().and_downcast::<SherlockRow>()?;
                let _ = self.activate_action(
                    "win.context-mode",
                    Some(&(selected.num_actions() > 0).to_variant()),
                );
            }
            context_model
                .and_then(|tmp| tmp.upgrade())
                .map(|ctx| ctx.remove_all());
        }
        None
    }
    fn focus_first(&self, context_model: Option<&WeakRef<ListStore>>) -> Option<()> {
        let selection = self.model().and_downcast::<SingleSelection>()?;
        let mut new_index = 0;
        let current_index = selection.selected();
        let n_items = selection.n_items();
        if n_items == 0 {
            return None;
        }
        while new_index < n_items {
            if let Some(item) = selection.item(new_index).and_downcast::<SherlockRow>() {
                if item.imp().spawn_focus.get() {
                    break;
                } else {
                    new_index += 1;
                }
            } else {
                break;
            }
        }
        let changed = new_index != current_index;
        if changed {
            selection.set_selected(new_index);
        }
        if new_index < n_items {
            self.scroll_to(new_index, ListScrollFlags::NONE, None);
        }
        // Update context mode shortcuts
        let selected = selection.selected_item().and_downcast::<SherlockRow>()?;
        let _ = self.activate_action(
            "win.context-mode",
            Some(&(selected.num_actions() > 0).to_variant()),
        );
        context_model
            .and_then(|tmp| tmp.upgrade())
            .map(|ctx| ctx.remove_all());

        changed.then_some(())
    }
    fn execute_by_index(&self, index: u32) {
        if let Some(selection) = self.model().and_downcast::<SingleSelection>() {
            for item in index..selection.n_items() {
                if let Some(row) = selection.item(item).and_downcast::<SherlockRow>() {
                    if row.imp().shortcut.get() {
                        row.emit_by_name::<()>("row-should-activate", &[]);
                        break;
                    }
                }
            }
        }
    }
    fn selected_item(&self) -> Option<glib::Object> {
        let selection = self.model().and_downcast::<SingleSelection>()?;
        selection.selected_item()
    }
    fn get_weaks(&self) -> Option<Vec<WeakRef<SherlockRow>>> {
        let selection = self.model().and_downcast::<SingleSelection>()?;
        let n_items = selection.n_items();
        let weaks = (0..n_items)
            .filter_map(|i| {
                selection
                    .item(i)
                    .and_downcast::<SherlockRow>()
                    .map(|row| row.downgrade())
            })
            .collect();
        Some(weaks)
    }
}
