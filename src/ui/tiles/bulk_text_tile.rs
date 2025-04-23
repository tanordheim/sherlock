use gio::glib::object::ObjectExt;
use gtk4::prelude::WidgetExt;
use std::vec;

use crate::actions::{execute_from_attrs, get_attrs_map};
use crate::g_subclasses::sherlock_row::SherlockRow;
use crate::launcher::bulk_text_launcher::BulkText;
use crate::launcher::{Launcher, ResultItem};

use super::util::{AsyncLauncherTile, TextTileElements, TileBuilder};
use super::Tile;

impl Tile {
    pub fn bulk_text_tile_loader(
        launcher: Launcher,
        keyword: &str,
        bulk_text: &BulkText,
    ) -> Option<(AsyncLauncherTile, ResultItem)> {
        let builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/bulk_text_tile.ui");

        builder.category.upgrade().map(|cat| {
            if let Some(name) = &launcher.name {
                cat.set_text(name);
            } else {
                cat.set_visible(false);
            }
        });

        builder
            .icon
            .upgrade()
            .map(|icon| {
                if bulk_text.icon.starts_with("/") {
                    icon.set_from_file(Some(&bulk_text.icon));
                } else {
                    icon.set_icon_name(Some(&bulk_text.icon));
                }
                icon.set_pixel_size(15);
            });

        builder
            .content_title
            .upgrade()
            .map(|title| title.set_text(keyword));
        builder
            .content_body
            .upgrade()
            .map(|body| body.set_text("Loading..."));

        let attrs = get_attrs_map(vec![("method", &launcher.method), ("keyword", keyword)]);
        let attrs_clone = attrs.clone();
        builder.object.set_spawn_focus(launcher.spawn_focus);
        builder.object.set_shortcut(launcher.shortcut);
        builder.object.add_css_class("bulk-text");
        builder
            .object
            .connect("row-should-activate", false, move |row| {
                let row = row.first().map(|f| f.get::<SherlockRow>().ok())??;
                execute_from_attrs(&row, &attrs_clone);
                None
            });

        let shortcut_holder = match launcher.shortcut {
            true => builder.shortcut_holder,
            _ => None,
        };
        let result_item = ResultItem {
            priority: launcher.priority as f32,
            row_item: builder.object,
            shortcut_holder,
        };
        let text_tile = Some(TextTileElements {
            title: builder.content_title,
            body: builder.content_body,
        });
        return Some((
            AsyncLauncherTile {
                launcher,
                row: result_item.row_item.downgrade(),
                text_tile,
                image_replacement: None,
                weather_tile: None,
                attrs,
            },
            result_item,
        ));
    }
}
