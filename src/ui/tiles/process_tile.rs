use gtk4::prelude::*;

use crate::launcher::process_launcher::ProcessLauncher;
use crate::launcher::{Launcher, ResultItem};

use super::util::TileBuilder;
use super::Tile;

impl Tile {
    pub fn process_tile(
        launcher: &Launcher,
        keyword: &str,
        proc: &ProcessLauncher
    ) -> Vec<ResultItem> {
        let mut results: Vec<ResultItem> = Default::default();

        for (_key, value) in proc.processes.iter() {
            if value
                .to_lowercase()
                .contains(&keyword.to_lowercase())
            {
                let builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/tile.ui");
                builder.object.set_spawn_focus(launcher.spawn_focus);
                builder.object.set_shortcut(launcher.shortcut);


                if launcher.name.is_empty() {
                    builder.category.set_visible(false);
                }
                builder.category.set_text(&launcher.name);
                builder.title.set_markup(&value);
                builder.add_default_attrs(
                    Some(&launcher.method),
                    Some(keyword),
                    Some(keyword),
                    None,
                    None,
                );

                let shortcut_holder = match launcher.shortcut {
                    true => builder.shortcut_holder,
                    _ => None,
                };
                results.push(ResultItem {
                    priority: launcher.priority as f32,
                    row_item: builder.object,
                    shortcut_holder,
                });
            }
        }
        return results;
    }
}
