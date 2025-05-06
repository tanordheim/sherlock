use gio::{
    glib::object::{Cast, CastNone, ObjectExt},
    prelude::ListModelExt,
    ListStore,
};
use gtk4::{
    prelude::{ListItemExt, WidgetExt},
    ListView, SignalListItemFactory, SingleSelection,
};

use crate::g_subclasses::action_entry::ContextAction;

pub fn make_context() -> (ListView, ListStore) {
    let factory = make_factory();
    let model = ListStore::new::<ContextAction>();
    let selection = SingleSelection::new(Some(model.clone()));
    let context = ListView::new(Some(selection), Some(factory));

    model.connect_items_changed({
        let context_clone = context.downgrade();
        move |model, _, _, _| {
            if let Some(context) = context_clone.upgrade() {
                let n_items = model.n_items();
                context.set_visible(n_items != 0);
            }
        }
    });

    context.set_widget_name("context-menu");
    context.set_focusable(false);
    context.set_visible(false);
    context.set_width_request(300);
    context.set_halign(gtk4::Align::End);
    context.set_valign(gtk4::Align::End);
    (context, model)
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
