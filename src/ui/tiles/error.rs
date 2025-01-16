use gtk4::{self, prelude::*, Builder, Label, ListBoxRow, TextView};

use crate::loader::util::SherlockError;
use super::Tile;

impl Tile {
    pub fn error_tile(index: i32, errors: &Vec<SherlockError>) -> (i32, Vec<ListBoxRow>) {
        let widgets: Vec<ListBoxRow> = errors
            .iter()
            .map(|e| {
                let builder = Builder::from_resource("/dev/skxxtz/sherlock/ui/error_tile.ui");
                
                let holder: ListBoxRow = builder.object("holder").unwrap();
                let title: Label = builder.object("error-title").unwrap();
                let content_title: Label = builder.object("content-title").unwrap();
                let content_body: TextView = builder.object("content-body").unwrap();
                
                title.set_text(&e.name);
                content_title.set_text(&e.message);
                content_body.buffer().set_text(&e.traceback.trim());

                holder
            })
            .collect();

        (index + widgets.len() as i32, widgets)
    }
}
