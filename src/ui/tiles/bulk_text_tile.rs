use gio::glib::object::ObjectExt;
use gtk4::prelude::WidgetExt;
use gtk4::Label;
use std::collections::HashMap;
use std::vec;

use crate::actions::{execute_from_attrs, get_attrs_map};
use crate::g_subclasses::sherlock_row::SherlockRow;
use crate::launcher::bulk_text_launcher::BulkText;
use crate::launcher::{Launcher, ResultItem};

use super::util::{AsyncOptions, TileBuilder};
use super::Tile;

impl Tile {
    pub fn bulk_text_tile_loader(
        launcher: &Launcher,
        keyword: &str,
        bulk_text: &BulkText,
    ) -> Option<(
        ResultItem,
        Option<Label>,
        Option<Label>,
        Option<AsyncOptions>,
        HashMap<String, String>,
    )> {
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
        let res = ResultItem {
            priority: launcher.priority as f32,
            row_item: builder.object,
            shortcut_holder,
        };

        return Some((
            res,
            Some(builder.content_title),
            Some(builder.content_body),
            None,
            attrs,
        ));
    }
    pub fn bulk_text_tile(
        launcher: &Launcher,
        keyword: &str,
        bulk_text: &BulkText,
    ) -> Vec<ResultItem> {
        let builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/bulk_text_tile.ui");
        builder.object.add_css_class("bulk-text");

        builder.category.set_text(&launcher.name);
        builder.icon.set_icon_name(Some(&bulk_text.icon));
        builder.icon.set_pixel_size(15);
        builder.title.set_text(keyword);
        let shortcut_holder = match launcher.shortcut {
            true => builder.shortcut_holder,
            _ => None,
        };

        let attrs = get_attrs_map(vec![("method", &launcher.method), ("keyword", keyword)]);

        builder
            .object
            .connect("row-should-activate", false, move |row| {
                let row = row.first().map(|f| f.get::<SherlockRow>().ok())??;
                execute_from_attrs(&row, &attrs);
                None
            });
        let res = ResultItem {
            priority: launcher.priority as f32,
            row_item: builder.object,
            shortcut_holder,
        };

        return vec![res];
    }
}
