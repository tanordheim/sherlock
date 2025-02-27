use gtk4::{Label, ListBoxRow};

use super::util::TileBuilder;
use super::Tile;

impl Tile {
    pub fn bulk_text_tile_loader(
        name: &String,
        method: &String,
        icon: &String,
        keyword: &String,
    ) -> Option<(ListBoxRow, Label, Label)> {
        if !keyword.is_empty() {
            let builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/bulk_text_tile.ui", 0, false);

            builder.category.set_text(name);
            builder.icon.set_icon_name(Some(icon));
            builder.content_title.set_text(keyword);
            builder.content_body.set_text("Loading...");
            builder.add_default_attrs(Some(method), None, Some(keyword), None, None);

            return Some((builder.object, builder.content_title, builder.content_body));
        }
        return None;
    }
    pub fn bulk_text_tile(
        name: &String,
        method: &String,
        icon: &String,
        index: i32,
        keyword: &String,
    ) -> (i32, Vec<ListBoxRow>) {
        if !keyword.is_empty() {
            let builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/bulk_text_tile.ui", index, false);
            builder.category.set_text(name);
            builder.icon.set_icon_name(Some(icon));
            builder.title.set_text(keyword);
            builder.add_default_attrs(Some(method), None, Some(keyword), None, None);


            return (index + 1, vec![builder.object]);
        }
        (index, vec![])
    }
}
