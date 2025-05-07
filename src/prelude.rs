use std::{collections::HashSet, fmt::Debug};

use gdk_pixbuf::subclass::prelude::ObjectSubclassIsExt;
use gio::{
    glib::{
        self,
        object::{Cast, CastNone, ObjectExt},
        variant::ToVariant,
        WeakRef,
    },
    prelude::ListModelExt,
    ListStore,
};
use gtk4::{
    prelude::WidgetExt, Box as GtkBox, Image, Label, ListScrollFlags, ListView, SingleSelection,
};

use crate::{g_subclasses::sherlock_row::SherlockRow, loader::pipe_loader::PipeData};

/// Custom string matching
pub trait SherlockSearch {
    fn fuzzy_match<T: AsRef<str> + Debug>(&self, substring: T) -> bool;
}

impl SherlockSearch for String {
    fn fuzzy_match<T>(&self, substring: T) -> bool
    where
        Self: AsRef<str>,
        T: AsRef<str> + Debug,
    {
        let lowercase = substring.as_ref().to_lowercase();
        let char_pattern: HashSet<char> = lowercase.chars().collect();
        let concat_str: String = self
            .to_lowercase()
            .chars()
            .filter(|s| char_pattern.contains(s) || *s == ';')
            .collect();
        concat_str.contains(&lowercase)
    }
}
impl SherlockSearch for PipeData {
    fn fuzzy_match<T>(&self, substring: T) -> bool
    where
        T: AsRef<str>,
    {
        // check which value to use
        let search_in = match self.title {
            Some(_) => &self.title,
            None => &self.description,
        };
        if let Some(search_in) = search_in {
            let lowercase = substring.as_ref().to_lowercase();
            let char_pattern: HashSet<char> = lowercase.chars().collect();
            let concat_str: String = search_in
                .to_lowercase()
                .chars()
                .filter(|s| char_pattern.contains(s) || *s == ';')
                .collect();
            return concat_str.contains(&lowercase);
        }
        return false;
    }
}
/// Apply icon by name or by path if applicable
pub trait IconComp {
    fn set_icon(
        &self,
        icon_name: &Option<String>,
        icon_class: &Option<String>,
        fallback: &Option<String>,
    );
}
impl IconComp for Image {
    fn set_icon(
        &self,
        icon_name: &Option<String>,
        icon_class: &Option<String>,
        fallback: &Option<String>,
    ) {
        if let Some(icon_name) = icon_name.as_ref().or_else(|| fallback.as_ref()) {
            if icon_name.starts_with("/") {
                self.set_from_file(Some(icon_name));
            } else {
                self.set_icon_name(Some(icon_name));
            }
        } else {
            self.set_visible(false);
        }
        icon_class.as_ref().map(|c| self.add_css_class(c));
    }
}
pub trait ShortCut {
    fn apply_shortcut(&self, index: i32, mod_str: &str) -> i32;
    fn remove_shortcut(&self) -> i32;
}
impl ShortCut for GtkBox {
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

/// Navigation for elements within a ListView
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
            self.scroll_to(0, ListScrollFlags::NONE, None);
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
