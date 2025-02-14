use gtk4::{self, prelude::*, Builder, Label, ListBoxRow, TextView, Orientation};

use crate::loader::util::SherlockError;
use super::Tile;

impl Tile {
    pub fn error_tile(index: i32, errors: &Vec<SherlockError>, icon: &str, tile_type: &str) -> (i32, Vec<ListBoxRow>) {
        let widgets: Vec<ListBoxRow> = errors
            .iter()
            .map(|e| {
                let builder = Builder::from_resource("/dev/skxxtz/sherlock/ui/error_tile.ui");
                
                let holder: ListBoxRow = builder.object("holder").unwrap();
                let title: Label = builder.object("title").unwrap();
                let content_title: Label = builder.object("content-title").unwrap();
                let content_body: Label = builder.object("content-body").unwrap();

                if let Some(class) = match tile_type {
                    "ERROR" => Some("error"),
                    "WARNING" => Some("warning"),
                    _ => None,
                } {
                    holder.set_css_classes(&["error-tile", class]);
                }


                
                title.set_text(format!("{:5}{}:  {}", icon, tile_type, &e.name).as_str());
                content_title.set_text(&e.message);
                content_body.set_text(&e.traceback.trim());
                holder
            })
            .collect();

        (index + widgets.len() as i32, widgets)
    }
}
