use gtk4::{Label, ListBoxRow};

use super::util::{get_builder, insert_attrs};
use super::Tile;


impl Tile {
    pub fn bulk_text_tile_loader(
        name: &String,
        method: &String,
        icon: &String,
        keyword: &String,
    ) -> Option<(ListBoxRow, Label, Label)> {
        if !keyword.is_empty() {
            let builder = get_builder("/dev/skxxtz/sherlock/ui/bulk_text_tile.ui", 0);

            builder.category.set_text(name);
            builder.icon.set_icon_name(Some(icon));
            builder.content_title.set_text(keyword);
            builder.content_body.set_text("Loading...");

            let attrs: Vec<(&str, &str)> = vec![
                ("method", method),
                ("keyword", keyword),
            ];
            insert_attrs(&builder.attrs, attrs);

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

            let builder = get_builder("/dev/skxxtz/sherlock/ui/bulk_text_tile.ui", index);
            builder.category.set_text(name);
            builder.icon.set_icon_name(Some(icon));
            builder.title.set_text(keyword);

            let attrs: Vec<(&str, &str)> = vec![
                ("method", method),
                ("keyword", keyword),
            ];
            insert_attrs(&builder.attrs, attrs);

            return (index + 1, vec![builder.object]);
        }
        (index, vec![])
    }
}
