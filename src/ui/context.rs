use gio::{
    glib::object::{Cast, CastNone, ObjectExt},
    prelude::ListModelExt,
    ListStore,
};
use gtk4::{
    prelude::{ListItemExt, WidgetExt},
    ListView, Revealer, SignalListItemFactory, SingleSelection,
};

use crate::{g_subclasses::action_entry::ContextAction, CONFIG};

pub fn make_context() -> (ListView, ListStore, Revealer) {
    let factory = make_factory();
    let model = ListStore::new::<ContextAction>();
    let selection = SingleSelection::new(Some(model.clone()));
    let context = ListView::new(Some(selection), Some(factory));

    let revealer = Revealer::builder()
        .transition_type(gtk4::RevealerTransitionType::Crossfade)
        .transition_duration(100)
        .valign(gtk4::Align::End)
        .halign(gtk4::Align::End)
        .child(&context)
        .build();

    if !CONFIG.get().map_or(false, |c| c.behavior.animate) {
        revealer.set_transition_duration(0);
    }

    model.connect_items_changed({
        let revealer = revealer.downgrade();
        move |model, _, _, _| {
            let n_items = model.n_items();
            if let Some(revealer) = revealer.upgrade() {
                if n_items != 0 {
                    revealer.set_reveal_child(true);
                } else {
                    let tmp = revealer.transition_duration();
                    revealer.set_transition_duration(0);
                    revealer.set_reveal_child(false);
                    revealer.set_transition_duration(tmp);
                }
            }
        }
    });

    context.set_widget_name("context-menu");
    context.set_focusable(false);
    context.set_width_request(300);
    (context, model, revealer)
}
fn make_factory() -> SignalListItemFactory {
    let factory = SignalListItemFactory::new();
    factory.connect_bind(|_, item| {
        let item = item
            .downcast_ref::<gtk4::ListItem>()
            .expect("Item mut be a ListItem");
        let row = item
            .item()
            .clone()
            .and_downcast::<ContextAction>()
            .expect("Row should be ContextAction");
        item.set_child(Some(&row));
    });
    factory
}
