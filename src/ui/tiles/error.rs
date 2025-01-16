use gtk4::{self, prelude::*, Box, Builder, Image, Label, ListBoxRow, TextView};

use crate::loader::util::SherlockError;
use super::Tile;

#[derive(Clone)]
struct ErrorTileReq {
    holder: ListBoxRow,
    title: Label,
    content_title: Label,
    content_body: TextView,
}


impl Tile{
    pub fn error_tile(index:i32, errors: &Vec<SherlockError>)->(i32, Vec<ListBoxRow>){
        let builder = Builder::from_resource("/dev/skxxtz/sherlock/ui/error_tile.ui");
        let holder: ListBoxRow = builder.object("holder").unwrap();
        let title: Label = builder.object("error-title").unwrap();
        let content_title: Label = builder.object("content-title").unwrap();
        let content_body: TextView = builder.object("content-body").unwrap();
        let ui_elements = ErrorTileReq {holder, title, content_title, content_body};
        
        let widgets: Vec<ListBoxRow> = errors
            .iter()
            .map(|e| {
                let custom_ui_elements = ui_elements.clone();
                custom_ui_elements.title.set_text(&e.name);
                custom_ui_elements.content_title.set_text(&e.message);
                custom_ui_elements.content_body.buffer().set_text(&e.traceback);
                custom_ui_elements.holder
            })
            .collect();
        (index + widgets.len() as i32, widgets)
    }
}
