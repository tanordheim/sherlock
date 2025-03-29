use gtk4::gdk::{Key, ModifierType, Rectangle};
use gtk4::{prelude::*, Label, ListBox, ListBoxRow, ScrolledWindow, StackTransitionType, Widget};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::actions::execute_from_attrs;
use crate::launcher::{construct_tiles, Launcher};
use crate::{APP_STATE, CONFIG};

pub fn show_stack_page<T: AsRef<str>>(page_name: T, transition: Option<StackTransitionType>) {
    APP_STATE.with(|state| {
        if let Some(ref state) = *state.borrow() {
            state.stack.as_ref().map(|stack| {
                if let Some(transition) = transition {
                    stack.set_transition_type(transition);
                };
                stack.set_visible_child_name(page_name.as_ref());
            });
        }
    });
}

pub fn execute_by_index(results: &ListBox, index: i32) {
    if let Some(row) = results.row_at_index(index - 1) {
        let attrs = get_row_attrs(row);
        execute_from_attrs(attrs);
    }
}
pub fn get_row_attrs(selected_row: ListBoxRow) -> HashMap<String, String> {
    let mut attrs: HashMap<String, String> = Default::default();
    if let Some(main_holder) = selected_row.first_child() {
        if let Some(attrs_holder) = main_holder.first_child() {
            if let Some(first_label_obj) = attrs_holder.first_child() {
                if let Some(text) = read_from_label(&first_label_obj) {
                    attrs.insert(text.0, text.1);
                }
                let mut current_label_obj = first_label_obj;
                while let Some(next_label_obj) = current_label_obj.next_sibling() {
                    if let Some(text) = read_from_label(&next_label_obj) {
                        attrs.insert(text.0, text.1);
                    }
                    current_label_obj = next_label_obj;
                }
            }
        }
    }
    attrs
}

pub fn set_mode(mode_title: &Label, mode_c: &Rc<RefCell<String>>, ctext: &str, mode_name: &str) {
    let new_mode = ctext.to_string();
    mode_title.set_text(mode_name);
    *mode_c.borrow_mut() = new_mode;
}

pub fn set_home_screen(
    keyword: &str,
    mode: &str,
    results_frame: &ListBox,
    launchers: &Vec<Launcher>,
) {
    // Check if launcher should be shown on home
    let (show, _): (Vec<Launcher>, Vec<Launcher>) = launchers
        .clone()
        .into_iter()
        .partition(|launcher| launcher.home);

    // Remove all elements inside to avoid duplicates
    while let Some(row) = results_frame.last_child() {
        results_frame.remove(&row);
    }

    let widgets = construct_tiles(&keyword.to_string(), &show, &mode.to_string());
    if let Some(c) = CONFIG.get() {
        for widget in widgets {
            if c.behavior.animate {
                widget.add_css_class("animate");
            }
            results_frame.append(&widget);
        }
    }
}
pub fn read_from_label(label_obj: &Widget) -> Option<(String, String)> {
    if let Some(label) = label_obj.downcast_ref::<Label>() {
        let text = label.text();
        let parts: Vec<&str> = text.split(" | ").collect();

        if parts.len() == 2 {
            return Some((parts[0].to_string(), parts[1].to_string()));
        }
    }
    return None;
}

pub trait RowOperations {
    fn focus_next(&self, result_viewport: &ScrolledWindow);
    fn focus_prev(&self, result_viewport: &ScrolledWindow);
    fn focus_first(&self);
    fn select_offset_row(&self, offset: i32) -> ListBoxRow;
}

impl RowOperations for ListBox {
    fn focus_next(&self, result_viewport: &ScrolledWindow) {
        let new_row = self.select_offset_row(1);
        let allocation = result_viewport.allocation();
        let list_box_rect = Rectangle::from(allocation);

        let row_allocation = new_row.allocation();
        let row_rect = Rectangle::from(row_allocation);

        let list_height = list_box_rect.height() as f64;
        let row_end = (row_rect.y() + row_rect.height() + 14) as f64;
        let vadjustment = result_viewport.vadjustment();

        let current_value = vadjustment.value();
        let list_end = list_height + current_value;
        if row_end > list_end {
            let delta = row_end - list_end;
            let new_value = current_value + delta;
            vadjustment.set_value(new_value);
        }
    }
    fn focus_prev(&self, result_viewport: &ScrolledWindow) {
        let new_row = self.select_offset_row(-1);

        let row_allocation = new_row.allocation();
        let row_rect = Rectangle::from(row_allocation);

        let row_start = (row_rect.y()) as f64;
        let vadjustment = result_viewport.vadjustment();

        let current_value = vadjustment.value();
        if current_value > row_start {
            vadjustment.set_value(row_start);
        }
    }
    fn focus_first(&self) {
        if let Some(first_row) = self.first_child() {
            if let Some(row) = first_row.downcast_ref::<gtk4::ListBoxRow>() {
                self.select_row(Some(row));
            }
        }
    }
    fn select_offset_row(&self, offset: i32) -> ListBoxRow {
        if let Some(row) = self.selected_row() {
            let new_index = row.index() + offset;
            if let Some(new_row) = self.row_at_index(new_index) {
                self.select_row(Some(&new_row));
                return new_row;
            };
            return row;
        };
        return ListBoxRow::new();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConfKeys {
    pub next: Option<Key>,
    pub next_mod: Option<ModifierType>,
    pub prev: Option<Key>,
    pub prev_mod: Option<ModifierType>,
}
impl ConfKeys {
    pub fn from<T: AsRef<str>>(next: T, prev: T) -> Self {
        let (next_mod, next) = ConfKeys::eval_bind_combination(next);
        let (prev_mod, prev) = ConfKeys::eval_bind_combination(prev);
        ConfKeys {
            next,
            next_mod,
            prev,
            prev_mod,
        }
    }
    pub fn empty() -> Self {
        ConfKeys {
            next: None,
            next_mod: None,
            prev: None,
            prev_mod: None,
        }
    }
    fn eval_bind_combination<T: AsRef<str>>(key: T) -> (Option<ModifierType>, Option<Key>) {
        let key_str = key.as_ref();
        match key_str.split("-").collect::<Vec<&str>>().as_slice() {
            [modifier, key, ..] => (ConfKeys::eval_mod(modifier), ConfKeys::eval_key(key)),
            [key, ..] => (None, ConfKeys::eval_key(key)),
            _ => (None, None),
        }
    }
    fn eval_key(key: &str) -> Option<Key> {
        match key {
            "tab" => Some(Key::Tab),
            "up" => Some(Key::Up),
            "down" => Some(Key::Down),
            "left" => Some(Key::Left),
            "right" => Some(Key::Right),
            "pgup" => Some(Key::Page_Up),
            "pgdown" => Some(Key::Page_Down),
            "end" => Some(Key::End),
            "home" => Some(Key::Home),
            _ => None,
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
}

#[test]
fn test_custom_binds() {
    let prev = "s-tab";
    let next = "tab";
    let sk = ConfKeys::from(next, prev);
    let sb = ConfKeys {
        prev: Some(Key::Tab),
        prev_mod: Some(ModifierType::SHIFT_MASK),

        next: Some(Key::Tab),
        next_mod: None,
    };
    let ck = ConfKeys::from("tab", "c-tab");
    let cb = ConfKeys {
        prev: Some(Key::Tab),
        prev_mod: Some(ModifierType::CONTROL_MASK),

        next: Some(Key::Tab),
        next_mod: None,
    };
    println!("{:?}", ck);
    assert_eq!(sk, sb);
    assert_eq!(ck, cb);
}
