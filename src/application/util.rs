use gtk4::{prelude::*, ApplicationWindow};
use gtk4::{Entry, EventController, Stack, Widget};

pub struct AppState {
    pub window: Option<ApplicationWindow>,
    pub stack: Option<Stack>,
    pub search_bar: Option<Entry>,
}
impl AppState {
    pub fn add_stack_page<T, U>(&self, child: T, name: U)
    where
        T: IsA<Widget>,
        U: AsRef<str>,
    {
        if let Some(stack) = &self.stack {
            stack.add_named(&child, Some(name.as_ref()));
        }
    }

    pub fn add_event_listener<T: IsA<EventController>>(&self, controller: T) {
        if let Some(window) = &self.window {
            window.add_controller(controller);
        }
    }
    pub fn remove_event_listener<T: IsA<EventController>>(&self, controller: &T) {
        if let Some(window) = &self.window {
            window.remove_controller(controller);
        }
    }
}
