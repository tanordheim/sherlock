use gtk4::{prelude::*, Label, Box, ListBoxRow, TextView};
use crate::launcher::Launcher;

pub struct AsyncLauncherTile {
    pub launcher: Launcher,
    pub widget: ListBoxRow,
    pub title: Label,
    pub body: TextView,
}

pub fn insert_attrs(attr_holder:&Box, attrs:Vec<String>){
    for item in attrs{
        let label = Label::new(None);
        label.set_text(&item);
        attr_holder.append(&label);
    }
}


pub fn ensure_icon_name(name: String)->String{
    if name.ends_with("-symbolic"){
        name
    } else {
        format!("{}-symbolic", name)
    }
}

pub fn ensure_icon_name_new(name: &String)->Option<String>{
    if !name.ends_with("-symbolic"){
        return Some(format!("{}-symbolic", name))
    }
    None
}
