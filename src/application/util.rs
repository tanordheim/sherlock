use gtk4::{prelude::*, ApplicationWindow};
use gtk4::{EventController, Stack, Widget};

use crate::g_subclasses::sherlock_input::SherlockInput;
use crate::loader::pipe_loader::deserialize_pipe;
use crate::loader::util::SherlockError;
use crate::loader::Loader;
use crate::ui::util::{remove_stack_children, show_stack_page};
use crate::{ui, CONFIG, FLAGS};

pub struct AppState {
    pub window: Option<ApplicationWindow>,
    pub stack: Option<Stack>,
    pub search_bar: Option<SherlockInput>,
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

pub fn reload_content() -> Option<()> {
    let mut startup_errors: Vec<SherlockError> = Vec::new();
    let mut non_breaking: Vec<SherlockError> = Vec::new();
    let app_config = CONFIG.get()?;
    let sherlock_flags = FLAGS.get()?;
    remove_stack_children();

    let (launchers, n) = Loader::load_launchers_sync()
        .map_err(|e| startup_errors.push(e))
        .unwrap_or_default();

    non_breaking.extend(n);
    let pipe = Loader::load_pipe_args();
    if pipe.is_empty() {
        ui::search::search(&launchers);
    } else {
        if sherlock_flags.display_raw {
            let pipe = String::from_utf8_lossy(&pipe);
            ui::user::display_raw(pipe, sherlock_flags.center_raw);
        } else {
            let parsed = deserialize_pipe(pipe);
            if let Some(c) = CONFIG.get() {
                let method: &str = c.pipe.method.as_deref().unwrap_or("print");
                ui::user::display_pipe(parsed, method)
            }
        }
    };

    // Logic for the Error-View
    if !app_config.debug.try_suppress_errors {
        let show_errors = !startup_errors.is_empty();
        let show_warnings = !app_config.debug.try_suppress_warnings && !non_breaking.is_empty();
        if show_errors || show_warnings {
            ui::error_view::errors(&startup_errors, &non_breaking);
            show_stack_page("error-page", None);
        }
    };
    None
}
