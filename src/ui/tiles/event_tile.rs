use std::vec;

use gtk4::prelude::WidgetExt;

use super::util::EventTileBuilder;
use super::Tile;
use crate::launcher::event_launcher::EventLauncher;
use crate::launcher::{Launcher, ResultItem};

impl Tile {
    pub fn event_tile(
        launcher: &Launcher,
        keyword: &str,
        event_launcher: &EventLauncher,
    ) -> Vec<ResultItem> {
        let event = match &event_launcher.event {
            Some(event) => event,
            None => return vec![],
        };

        //Handle searching
        if !event.title.contains(keyword) {
            return vec![];
        }

        let builder = EventTileBuilder::new("/dev/skxxtz/sherlock/ui/event_tile.ui");
        builder.object.add_css_class("event-tile");
        builder.object.set_spawn_focus(launcher.spawn_focus);

        let mut attrs: Vec<(&str, &str)> = vec![];
        builder.title.set_text(&event.title);
        builder
            .icon
            .set_icon_name(Some(event_launcher.icon.as_ref()));
        builder.start_time.set_text(&event.start_time);
        builder
            .end_time
            .set_text(format!(".. {}", event.end_time).as_str());
        attrs.push(("meeting_url", &event.meeting_url));

        if let Some(next) = launcher.next_content.as_deref() {
            attrs.push(("next_content", next));
        }

        builder.add_default_attrs(Some(&launcher.method), None, None, None, Some(attrs));

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
