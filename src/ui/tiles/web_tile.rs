use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use gio::glib::object::ObjectExt;
use gio::glib::WeakRef;
use gtk4::prelude::WidgetExt;
use gtk4::Label;

use super::util::{update_tag, TileBuilder};
use super::Tile;
use crate::actions::{execute_from_attrs, get_attrs_map};
use crate::g_subclasses::sherlock_row::SherlockRow;
use crate::launcher::web_launcher::WebLauncher;
use crate::launcher::{Launcher, ResultItem};

impl Tile {
    pub fn web_tile(launcher: &Launcher, keyword: &str, web: &WebLauncher) -> Vec<ResultItem> {
        let builder = TileBuilder::new("/dev/skxxtz/sherlock/ui/tile.ui");
        builder.category.upgrade().map(|category| {
            if let Some(name) = &launcher.name {
                category.set_text(name);
            } else {
                category.set_visible(false);
            }
        });

        builder.icon.upgrade().map(|icon| {
            if web.icon.starts_with("/") {
                icon.set_from_file(Some(&web.icon));
            } else {
                icon.set_icon_name(Some(&web.icon));
            }
        });

        let tile_name = if web.display_name.contains("{keyword}") {
            web.display_name.replace("{keyword}", keyword)
        } else {
            web.display_name.clone()
        };

        // Construct attrs and enable action capabilities
        let mut attrs = get_attrs_map(vec![
            ("method", &launcher.method),
            ("result", keyword),
            ("keyword", keyword),
            ("engine", &web.engine),
        ]);
        if let Some(next) = launcher.next_content.as_deref() {
            attrs.insert(String::from("next_content"), next.to_string());
        }
        let attrs_rc = Rc::new(RefCell::new(attrs));
        builder.object.with_launcher(&launcher);
        builder.object.set_keyword_aware(true);

        let update_closure = {
            let tag_start = builder.tag_start.clone();
            let tag_end = builder.tag_end.clone();
            let tag_start_content = launcher.tag_start.clone();
            let tag_end_content = launcher.tag_end.clone();
            let title = builder.title.clone();
            let row_weak = builder.object.downgrade();
            move |keyword: &str| -> bool {
                let attrs_clone = Rc::clone(&attrs_rc);

                // Update title
                if let Some(title) = title.upgrade() {
                    title.set_text(&tile_name.replace("{keyword}", keyword));
                }

                // update first tag
                update_tag(&tag_start, &tag_start_content, keyword);

                // update second tag
                update_tag(&tag_end, &tag_end_content, keyword);

                // update attributes to activate correct action
                let keyword_clone = keyword.to_string();
                row_weak.upgrade().map(|row| {
                    let signal_id = row.connect_local("row-should-activate", false, move |row| {
                        let row = row.first().map(|f| f.get::<SherlockRow>().ok())??;
                        attrs_clone
                            .borrow_mut()
                            .insert("keyword".to_string(), keyword_clone.clone());
                        execute_from_attrs(&row, &attrs_clone.borrow());
                        None
                    });
                    row.set_signal_id(signal_id);
                });
                false
            }
        };
        builder.object.set_update(update_closure);

        let shortcut_holder = match launcher.shortcut {
            true => builder.shortcut_holder,
            _ => None,
        };
        let res = ResultItem {
            priority: launcher.priority as f32,
            row_item: builder.object,
            shortcut_holder,
        };

        return vec![res];
    }
}
