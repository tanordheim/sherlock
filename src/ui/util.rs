use gtk4::{prelude::*, ListBox, ListBoxRow, Widget, Label};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::launcher::{construct_tiles, Launcher};
use crate::actions::execute_from_attrs;

pub fn execute_by_index(results:&ListBox, index:i32){
    if let Some(row) = results.row_at_index(index-1){
        let attrs = get_row_attrs(row);
        execute_from_attrs(attrs);
    }
}
pub fn get_row_attrs(selected_row:ListBoxRow)->HashMap<String, String>{
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

pub fn set_mode(mode_title:&Label, mode_c:&Rc<RefCell<String>>, ctext:&str, mode_name:&String){
    let new_mode = ctext.to_string();
    mode_title.set_text(mode_name);
    *mode_c.borrow_mut() = new_mode;
}


pub fn set_results(keyword:&str,mode:&str, results_frame:&ListBox, launchers:&Vec<Launcher>){
    // Remove all elements inside to avoid duplicates
    while let Some(row) = results_frame.last_child() {
        results_frame.remove(&row);
    }
    let widgets = construct_tiles(&keyword.to_string(), &launchers, &mode.to_string());
    for widget in widgets {
        results_frame.append(&widget);
    }
}
pub fn read_from_label(label_obj:&Widget)->Option<(String, String)>{
    if let Some(label) = label_obj.downcast_ref::<Label>(){
        let text = label.text();
        let parts: Vec<&str> = text.split(" | ").collect();

        if parts.len() == 2 {
            return Some((parts[0].to_string(), parts[1].to_string()))
        }
    }
    return None
}



pub fn select_first_row(results: &ListBox){
    if let Some(first_row) = results.first_child(){
        if let Some(row) = first_row.downcast_ref::<gtk4::ListBoxRow>() {
            results.select_row(Some(row));
        } 
    }
}

pub fn select_row(offset: i32, listbox:&Rc<ListBox>)->ListBoxRow{
    if let Some(row) = listbox.selected_row(){
        let new_index = row.index() + offset;
        if let Some(new_row) = listbox.row_at_index(new_index){
            listbox.select_row(Some(&new_row));
            return new_row
        };
    };
    return ListBoxRow::new();
}

