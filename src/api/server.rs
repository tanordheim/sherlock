use gio::glib::MainContext;
use gtk4::prelude::WidgetExt;
use std::{cell::RefCell, os::unix::net::UnixStream, rc::Rc, thread};

use crate::{
    daemon::daemon::{SherlockDaemon, SizedMessage},
    sherlock_error,
    utils::errors::{SherlockError, SherlockErrorType},
    SOCKET_PATH,
};

use super::{api::SherlockAPI, call::ApiCall};

pub struct SherlockServer;
impl SherlockServer {
    pub fn listen(api: Rc<RefCell<SherlockAPI>>) {
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
                    if let Some(window) = api.borrow().window.as_ref().and_then(|win| win.upgrade())
                    {
                        if let Ok(cmd) = serde_json::from_str::<ApiCall>(&msg) {
                            match cmd {
                                ApiCall::Show => {
                                    let _ = gtk4::prelude::WidgetExt::activate_action(
                                        &window, "win.open", None,
                                    );
                                }
                                ApiCall::Obfuscate(visibility) => {
                                    api.borrow().obfuscate(visibility)
                                }
                                ApiCall::SherlockError(error) => {
                                    api.borrow().insert_msg(error);
                                }
                                ApiCall::Clear => {
                                    window.set_visible(true);
                                    api.borrow().clear_results();
                                }
                                ApiCall::InputOnly => {
                                    window.set_visible(true);
                                    api.borrow().show_raw();
                                }
                                ApiCall::DisplayPipe(mut content) => {
                                    window.set_visible(true);
                                    api.borrow().display_pipe(&mut content);
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
    pub fn send<T: AsRef<[u8]>>(message: T) -> Result<(), SherlockError> {
        let mut stream = UnixStream::connect(SOCKET_PATH).map_err(|e| {
            sherlock_error!(
                SherlockErrorType::SocketConnectError(SOCKET_PATH.to_string()),
                e.to_string()
            )
        })?;
        stream.write_sized(message.as_ref())?;
        Ok(())
    }
}
