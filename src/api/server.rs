use std::{cell::RefCell, rc::Rc, thread};
use gio::glib::MainContext;
use gtk4::prelude::WidgetExt;

use crate::daemon::daemon::SherlockDaemon;

use super::{api::SherlockAPI, call::ApiCall};

pub struct SherlockServer;
impl SherlockServer {
    pub fn listen(api: Rc<RefCell<SherlockAPI>>){
        // Create async pipeline
        let (sender, receiver) = async_channel::bounded(1);
        thread::spawn(move || {
            async_std::task::block_on(async {
                let _daemon = SherlockDaemon::new(sender).await;
            });
        });

        // Handle receiving using pipline
        MainContext::default().spawn_local({
            async move {
                while let Ok(msg) = receiver.recv().await {
                    if let Some(window) = api.borrow().window.as_ref().and_then(|win| win.upgrade()) {
                        if let Ok(cmd) = serde_json::from_str::<ApiCall>(&msg){
                            match cmd {
                                ApiCall::Show => {
                                    let _ = gtk4::prelude::WidgetExt::activate_action(
                                        &window, "win.open", None,
                                    );
                                },
                                ApiCall::Obfuscate(visibility) => {
                                    api.borrow().obfuscate(visibility)
                                },
                                ApiCall::SherlockError(error) => {
                                    api.borrow().insert_msg(error);
                                },
                                ApiCall::Clear => {
                                    window.set_visible(true);
                                    api.borrow().clear_results();
                                },
                                ApiCall::InputOnly => {
                                    window.set_visible(true);
                                    api.borrow().show_raw();
                                },
                                ApiCall::DisplayPipe(content) => {
                                    window.set_visible(true);
                                    api.borrow().display_pipe(&content);
                                }
                            }
                        } else {
                            println!("{}", msg);
                        }
                    }
                }
            }
        }); 
    }

}
