use std::vec;

use super::util::EventTileBuilder;
use super::Tile;
use crate::launcher::event_launcher::EventLauncher;
use crate::launcher::{Launcher, ResultItem};

impl Tile {
    pub fn event_tile(
        launcher: &Launcher,
        index: i32,
        keyword: &str,
        event_launcher: &EventLauncher,
    ) -> (i32, Vec<ResultItem>) {
        let event = match &event_launcher.event {
            Some(event) => event,
            None => return (index, vec![]),
        };

        //Handle searching
        if !event.title.contains(keyword) {
            return (index, vec![]);
        }

        let builder = EventTileBuilder::new("/dev/skxxtz/sherlock/ui/event_tile.ui", index, false);
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
        let res = ResultItem {
            priority: launcher.priority as f32,
            row_item: builder.object,
        };
        return (index + 1, vec![res]);
    }
}
