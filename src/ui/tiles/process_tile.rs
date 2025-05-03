use gtk4::prelude::*;

use crate::actions::{execute_from_attrs, get_attrs_map};
use crate::g_subclasses::sherlock_row::SherlockRow;
use crate::launcher::process_launcher::ProcessLauncher;
use crate::launcher::Launcher;

use super::util::{SherlockSearch, TileBuilder};
use super::Tile;

impl Tile {
    pub fn process_tile(
        launcher: &Launcher,
        keyword: &str,
        proc: &ProcessLauncher,
    ) -> Vec<SherlockRow> {
        let mut results: Vec<SherlockRow> = Default::default();

        for (key, value) in proc.processes.iter() {
            if value.to_lowercase().fuzzy_match(&keyword.to_lowercase()) {
                let builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/tile.ui");

                builder.category.upgrade().map(|category| {
                    if let Some(name) = &launcher.name {
                        category.set_text(name);
                    } else {
                        category.set_visible(false);
                    }
                });
                builder
                    .title
                    .upgrade()
                    .map(|title| title.set_markup(&value));
                builder.icon.upgrade().map(|icon| {
                    if proc.icon.starts_with("/") {
                        icon.set_from_file(Some(&proc.icon));
                    } else {
                        icon.set_icon_name(Some(&proc.icon));
                    }
                });

                let ppid = key.0;
                let cpid = key.1;
                let parent = ppid.to_string();
                let child = cpid.to_string();

                // Construct attrs and enable action capabilities
                let attrs = get_attrs_map(vec![
                    ("method", "kill-process"),
                    ("result", value),
                    ("keyword", keyword),
                    ("parent-pid", &parent),
                    ("child-pid", &child),
                ]);

                builder.object.with_launcher(&launcher);
                builder.object.set_search(value);
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
                results.push(builder.object)
            }
        }
        return results;
    }
}
