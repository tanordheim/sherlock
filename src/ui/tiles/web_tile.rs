use gtk4::prelude::WidgetExt;
use gtk4::ListBoxRow;

use super::util::{get_builder, insert_attrs};
use super::Tile;
use crate::launcher::web_launcher::Web;

impl Tile {
    pub fn web_tile(
        name: &String,
        method: &String,
        web: &Web,
        index: i32,
        keyword: &String,
    ) -> (i32, Vec<ListBoxRow>) {
        if !keyword.is_empty() {
            let builder = get_builder("/dev/skxxtz/sherlock/ui/tile.ui", index, true);

            builder.category.set_text(name);
            builder.icon.set_icon_name(Some(&web.icon));
            if web.display_name.contains("{keyword}") {
                builder
                    .title
                    .set_text(&web.display_name.replace("{keyword}", ""));
                builder.tag_start.set_text(keyword);
                builder.tag_start.set_visible(true);
            } else {
                builder.title.set_text(keyword);
            }

            let attrs: Vec<(&str, &str)> = vec![
                ("method", method),
                ("engine", &web.engine),
                ("keyword", keyword),
                ("result", keyword),
            ];
            insert_attrs(&builder.attrs, attrs);

            return (index + 1, vec![builder.object]);
        }
        (index, vec![])
    }
}
