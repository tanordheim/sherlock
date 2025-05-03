use gio::glib::object::ObjectExt;
use gtk4::prelude::WidgetExt;
use std::pin::Pin;
use std::vec;

use crate::actions::{execute_from_attrs, get_attrs_map};
use crate::g_subclasses::sherlock_row::SherlockRow;
use crate::launcher::bulk_text_launcher::BulkTextLauncher;
use crate::launcher::{Launcher, ResultItem};

use super::util::TileBuilder;
use super::Tile;

impl Tile {
    pub fn bulk_text_tile_loader(
        launcher: &Launcher,
        keyword: &str,
        bulk_text: &BulkTextLauncher,
    ) -> Vec<ResultItem> {
        let builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/bulk_text_tile.ui");

        // Set category name
        builder.category.upgrade().map(|cat| {
            if let Some(name) = &launcher.name {
                cat.set_text(name);
            } else {
                cat.set_visible(false);
            }
        });

        // Set icons
        builder.icon.upgrade().map(|icon| {
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
        builder.object.add_css_class("bulk-text");
        builder.object.with_launcher(&launcher);
        builder.object.set_keyword_aware(true);

        let row_weak = builder.object.downgrade();
        let launcher_clone = launcher.clone();
        let async_update_closure: Box<dyn Fn(&str) -> Pin<Box<dyn futures::Future<Output = ()>>>> =
            Box::new(move |keyword: &str| {
                let launcher = launcher_clone.clone();
                let row = row_weak.clone();
                let content_title = builder.content_title.clone();
                let content_body = builder.content_body.clone();
                let mut attrs = attrs.clone();
                let keyword = keyword.to_string();

                Box::pin(async move {
                    if let Some((title, body, next_content)) = &launcher.get_result(&keyword).await
                    {
                        content_title.upgrade().map(|t| t.set_text(&title));
                        content_body.upgrade().map(|b| b.set_text(&body));
                        if let Some(next_content) = next_content {
                            attrs.insert(String::from("next_content"), next_content.to_string());
                            row.upgrade().map(|row| {
                                let signal_id =
                                    row.connect_local("row-should-activate", false, move |row| {
                                        let row =
                                            row.first().map(|f| f.get::<SherlockRow>().ok())??;
                                        execute_from_attrs(&row, &attrs);
                                        None
                                    });
                                row.set_signal_id(signal_id);
                            });
                        }
                    }
                })
            });
        builder.object.set_async_update(async_update_closure);

        let shortcut_holder = match launcher.shortcut {
            true => builder.shortcut_holder,
            _ => None,
        };
        let result_item = ResultItem {
            row_item: builder.object,
            shortcut_holder,
        };
        return vec![result_item];
    }
}
