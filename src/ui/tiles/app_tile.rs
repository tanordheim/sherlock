use gtk4::prelude::*;
use std::collections::HashMap;

use crate::launcher::{Launcher, ResultItem};
use crate::loader::util::{AppData, SherlockConfig};

use super::util::{ensure_icon_name, TileBuilder};
use super::Tile;

impl Tile {
    pub fn app_tile(
        launcher: &Launcher,
        keyword: &str,
        commands: HashMap<String, AppData>,
        app_config: &SherlockConfig,
    ) -> Vec<ResultItem> {
        let mut results: Vec<ResultItem> = Default::default();

        for (key, value) in commands.into_iter() {
            if value
                .search_string
                .to_lowercase()
                .contains(&keyword.to_lowercase())
            {
                let builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/tile.ui");
                builder.object.set_spawn_focus(launcher.spawn_focus);

                let icon = if app_config.appearance.recolor_icons {
                    ensure_icon_name(value.icon)
                } else {
                    value.icon
                };
                let tile_name = key.replace("{keyword}", keyword);
                builder.display_tag_start(&value.tag_start, keyword);
                builder.display_tag_end(&value.tag_end, keyword);

                if launcher.name.is_empty() {
                    builder.category.set_visible(false);
                }
                builder.category.set_text(&launcher.name);
                builder.icon.set_icon_name(Some(&icon));
                builder.title.set_markup(&tile_name);
                builder.add_default_attrs(
                    Some(&launcher.method),
                    Some(keyword),
                    Some(keyword),
                    Some(&value.exec),
                    None,
                );

                let shortcut_holder = match launcher.shortcut {
                    true => builder.shortcut_holder,
                    _ => None,
                };
                results.push(ResultItem {
                    priority: value.priority,
                    row_item: builder.object,
                    shortcut_holder,
                });
            }
        }
        return results;
    }
}
