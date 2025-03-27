use gtk4::{Box, Label, ListBoxRow};

use super::util::TileBuilder;
use super::Tile;

impl Tile {
    pub fn bulk_text_tile_loader(
        name: &str,
        method: &str,
        icon: &str,
        keyword: &str,
    ) -> Option<(ListBoxRow, Label, Label, Box)> {
        let builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/bulk_text_tile.ui", 0, false);

        builder.category.set_text(name);
        builder.icon.set_icon_name(Some(icon));
        builder.icon.set_pixel_size(15);
        builder.content_title.set_text(keyword);
        builder.content_body.set_text("Loading...");
        builder.add_default_attrs(Some(method), None, Some(keyword), None, None);

        return Some((
            builder.object,
            builder.content_title,
            builder.content_body,
            builder.attrs,
        ));
    }
    pub fn bulk_text_tile(
        name: &str,
        method: &str,
        icon: &str,
        index: i32,
        keyword: &str,
    ) -> (i32, Vec<ListBoxRow>) {
        let builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/bulk_text_tile.ui", index, false);

        builder.category.set_text(name);
        builder.icon.set_icon_name(Some(icon));
        builder.icon.set_pixel_size(15);
        builder.title.set_text(keyword);
        builder.add_default_attrs(Some(method), None, Some(keyword), None, None);

        return (index + 1, vec![builder.object]);
    }
}
