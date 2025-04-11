use gtk4::prelude::WidgetExt;
use std::vec;

use crate::actions::get_attrs_map;
use crate::launcher::bulk_text_launcher::BulkText;
use crate::launcher::{Launcher, ResultItem};

use super::util::{AsyncLauncherTile, TextTileElements, TileBuilder};
use super::Tile;

impl Tile {
    pub fn bulk_text_tile_loader(
        launcher: Launcher,
        keyword: &str,
        bulk_text: &BulkText,
    ) -> Option<AsyncLauncherTile> {
        let builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/bulk_text_tile.ui");
        builder.object.add_css_class("bulk-text");
        builder.object.set_spawn_focus(launcher.spawn_focus);
        builder.object.set_shortcut(launcher.shortcut);

        builder.category.set_text(&launcher.name);
        builder.icon.set_icon_name(Some(&bulk_text.icon));
        builder.icon.set_pixel_size(15);
        builder.content_title.set_text(keyword);
        builder.content_body.set_text("Loading...");

        let attrs = get_attrs_map(vec![("method", &launcher.method), ("keyword", keyword)]);

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
        return Some(AsyncLauncherTile {
            launcher,
            result_item,
            text_tile,
            image_replacement: None,
            weather_tile: None,
            attrs,
        });
    }
}
