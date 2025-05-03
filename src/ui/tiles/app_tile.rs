use gtk4::prelude::*;
use std::collections::HashSet;

use crate::actions::{execute_from_attrs, get_attrs_map};
use crate::g_subclasses::sherlock_row::SherlockRow;
use crate::launcher::Launcher;
use crate::loader::util::AppData;

use super::util::{SherlockSearch, TileBuilder};
use super::Tile;

impl Tile {
    pub fn app_tile(
        launcher: &Launcher,
        keyword: &str,
        commands: &HashSet<AppData>,
    ) -> Vec<SherlockRow> {
        let mut results: Vec<SherlockRow> = Vec::with_capacity(commands.len());

        for value in commands.into_iter() {
            if value
                .search_string
                .to_lowercase()
                .fuzzy_match(&keyword.to_lowercase())
            {
                let builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/tile.ui");

                let tile_name = value.name.replace("{keyword}", keyword);
                builder.display_tag_start(&value.tag_start, keyword);
                builder.display_tag_end(&value.tag_end, keyword);

                builder.category.and_then(|tmp| tmp.upgrade()).map(|cat| {
                    if let Some(name) = &launcher.name {
                        cat.set_text(name);
                    } else {
                        cat.set_visible(false);
                    }
                });

                // Icon stuff
                builder.icon.and_then(|tmp| tmp.upgrade()).map(|icon| {
                    if value.icon.starts_with("/") {
                        icon.set_from_file(Some(&value.icon));
                    } else {
                        icon.set_icon_name(Some(&value.icon));
                    }
                    value.icon_class.as_ref().map(|c| icon.add_css_class(c));
                });

                builder
                    .title
                    .and_then(|tmp| tmp.upgrade())
                    .map(|title| title.set_markup(&tile_name));

                let attrs =
                    get_attrs_map(vec![("method", &launcher.method), ("exec", &value.exec)]);

                builder.object.set_search(&value.search_string);
                builder.object.with_launcher(launcher);
                builder.object.set_priority(value.priority);
                builder
                    .object
                    .connect_local("row-should-activate", false, move |row| {
                        let row = row.first().map(|f| f.get::<SherlockRow>().ok())??;
                        execute_from_attrs(&row, &attrs);
                        None
                    });
                if launcher.shortcut {
                    builder.object.set_shortcut_holder(builder.shortcut_holder);
                }

                results.push(builder.object);
            }
        }
        return results;
    }
}
