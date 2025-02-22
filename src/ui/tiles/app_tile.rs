use gtk4::{prelude::*, ListBoxRow};
use std::collections::HashMap;

use crate::loader::util::{AppData, Config};

use super::util::{ensure_icon_name, get_builder, insert_attrs};
use super::Tile;

impl Tile {
    pub fn app_tile(
        index: i32,
        commands: HashMap<String, AppData>,
        name: &String,
        method: &String,
        keyword: &String,
        app_config: &Config,
    ) -> (i32, Vec<ListBoxRow>) {
        let mut results: Vec<ListBoxRow> = Default::default();
        let mut index_ref = index;

        for (key, value) in commands.into_iter() {
            if value
                .search_string
                .to_lowercase()
                .contains(&keyword.to_lowercase())
            {
                let builder = get_builder("/dev/skxxtz/sherlock/ui/tile.ui", index, true);

                let icon = if app_config.appearance.recolor_icons {
                    ensure_icon_name(value.icon)
                } else {
                    value.icon
                };

                let tile_name = if key.contains("{keyword}") {
                    builder.tag_start.set_text(keyword);
                    builder.tag_start.set_visible(true);
                    &key.replace("{keyword}", "")
                } else {
                    &key
                };

                if name.is_empty() {
                    builder.category.set_visible(false);
                }
                builder.category.set_text(name);
                builder.icon.set_icon_name(Some(&icon));
                builder.title.set_markup(tile_name);

                let attrs: Vec<(&str, &str)> = vec![
                    ("method", method),
                    ("app_name", &key),
                    ("exec", &value.exec),
                    ("keyword", keyword),
                ];

                insert_attrs(&builder.attrs, attrs);
                index_ref += 1;
                results.push(builder.object);
            }
        }
        return (index_ref, results);
    }
}
