use gtk4::ListBoxRow;

use super::util::{insert_attrs, get_builder};
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
            let builder = get_builder("/dev/skxxtz/sherlock/ui/tile.ui", index);

            builder.category.set_text(name);
            builder.icon.set_icon_name(Some(&web.icon));
            builder.title.set_text(keyword);

            let attrs: Vec<String> = vec![
                format!("{} | {}", "method", method),
                format!("{} | {}", "engine", web.engine),
                format!("{} | {}", "keyword", keyword),
            ];
            insert_attrs(&builder.attrs, attrs);

            return (index + 1, vec![builder.object]);
        }
        (index, vec![])
    }
}
