use gtk4::{prelude::*, ListBoxRow};
use std::collections::HashMap;

use crate::loader::util::{AppData, Config};
use crate::launcher::Launcher;

use super::util::{ensure_icon_name, get_builder, insert_attrs};
use super::Tile;

impl Tile {
    pub fn app_tile(
        launcher: &Launcher,
        index: i32,
        keyword: &String,
        commands: HashMap<String, AppData>,
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
                let builder = get_builder("/dev/skxxtz/sherlock/ui/tile.ui", index_ref, true);

                let icon = if app_config.appearance.recolor_icons {
                    ensure_icon_name(value.icon)
                } else {
                    value.icon
                };
                let tile_name = key.replace("{keyword}", keyword);
                if let Some(start_tag) = &launcher.start_tag {
                    let text = start_tag.replace("{keyword}", keyword);
                    builder.tag_start.set_text(&text);
                    builder.tag_start.set_visible(true);
                }
                if let Some(end_tag) = &launcher.end_tag {
                    let text = end_tag.replace("{keyword}", keyword);
                    builder.tag_end.set_text(&text);
                    builder.tag_end.set_visible(true);
                }

                if launcher.name.is_empty() {
                    builder.category.set_visible(false);
                }
                builder.category.set_text(&launcher.name);
                builder.icon.set_icon_name(Some(&icon));
                builder.title.set_markup(&tile_name);

                let attrs: Vec<(&str, &str)> = vec![
                    ("method", &launcher.method),
                    ("app_name", &key),
                    ("exec", &value.exec),
                    ("keyword", keyword),
                    ("result", keyword),
                ];

                insert_attrs(&builder.attrs, attrs);
                index_ref += 1;
                results.push(builder.object);
            }
        }
        return (index_ref, results);
    }
}
