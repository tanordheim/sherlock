use gtk4::ListBoxRow;

use super::util::EventTileBuilder;
use super::Tile;
use crate::launcher::event_launcher::EventLauncher;
use crate::launcher::Launcher;

impl Tile {
    pub fn event_tile(
        launcher: &Launcher,
        index: i32,
        event_launcher: &EventLauncher,
    ) -> (i32, Vec<ListBoxRow>) {
        let builder = EventTileBuilder::new("/dev/skxxtz/sherlock/ui/event_tile.ui", index, true);
        let mut attrs: Vec<(&str, &str)> = vec![];


        if let Some(event) = &event_launcher.event {
            builder.title.set_text(&event.title);
            builder.time.set_text(&event.time);
            attrs.push(("meeting_url", &event.meeting_url));
        }

        if let Some(next) = launcher.next_content.as_deref() {
            attrs.push(("next_content", next));
        }

        builder.add_default_attrs(Some(&launcher.method), None, None, None, Some(attrs));

        return (index + 1, vec![builder.object]);
    }
}
