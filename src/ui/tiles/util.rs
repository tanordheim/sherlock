use crate::launcher::Launcher;
use gtk4::{prelude::*, Box, Label, ListBoxRow, Image, Builder};

pub struct AsyncLauncherTile {
    pub launcher: Launcher,
    pub widget: ListBoxRow,
    pub title: Label,
    pub body: Label,
}

pub fn insert_attrs(attr_holder: &Box, attrs: Vec<(&str, &str)>) {
    for item in attrs {
        let (key, value, ..) = item;
        let label = Label::new(Some(format!("{} | {}", key, value).as_str()));
        attr_holder.append(&label);
    }
}

pub fn ensure_icon_name(name: String) -> String {
    if name.ends_with("-symbolic") {
        name
    } else {
        format!("{}-symbolic", name)
    }
}


pub struct TileBuilder{
    pub object: ListBoxRow,
    pub icon: Image,
    pub title: Label,
    pub category: Label,
    pub attrs: Box,
    pub tag_start: Label,
    pub tag_end: Label,
}
pub fn get_builder(resource: &str, index: i32)-> TileBuilder{
    let builder = Builder::from_resource(resource);
    let object: ListBoxRow = builder.object("holder").unwrap();
    let icon: Image = builder.object("icon-name").unwrap();
    let title: Label = builder.object("app-name").unwrap();
    let category: Label = builder.object("launcher-type").unwrap();
    let attrs: Box = builder.object("attrs-holder").unwrap();
    let tag_start: Label = builder.object("app-name-tag-start").unwrap();
    let tag_end: Label = builder.object("app-name-tag-end").unwrap();

    if index < 5 {
        let shortcut_holder: Box = builder.object("shortcut-holder").unwrap();
        let shortcut: Label = builder.object("shortcut").unwrap();
        shortcut_holder.set_visible(true);
        shortcut.set_text(format!("ctrl + {}", index + 1).as_str());
    }

    TileBuilder{
        object,
        icon,
        title,
        category,
        attrs,
        tag_start,
        tag_end,
    }



}

