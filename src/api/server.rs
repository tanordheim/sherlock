use gio::glib::MainContext;
use std::{cell::RefCell, os::unix::net::UnixStream, rc::Rc, thread};

use crate::{
    daemon::daemon::{SherlockDaemon, SizedMessage}, sher_log, sherlock_error, utils::errors::{SherlockError, SherlockErrorType}, SOCKET_PATH
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
                    if let Ok(cmd) = serde_json::from_str::<ApiCall>(&msg) {
                        println!("{:?}", cmd);
                        api.borrow_mut().apply_action(cmd);
                    } else {
                        sher_log!(format!("Failed to deserialize api call(s): {}", msg));
                    }
                }
            }
        });
    }
    pub fn _send<T: AsRef<[u8]>>(message: T) -> Result<(), SherlockError> {
        let mut stream = UnixStream::connect(SOCKET_PATH).map_err(|e| {
            sherlock_error!(
                SherlockErrorType::SocketConnectError(SOCKET_PATH.to_string()),
                e.to_string()
            )
        })?;
        stream.write_sized(message.as_ref())?;
        Ok(())
    }
    pub fn _send_action(api_call: ApiCall) -> Result<(), SherlockError> {
        let mut stream = UnixStream::connect(SOCKET_PATH).map_err(|e| {
            sherlock_error!(
                SherlockErrorType::SocketConnectError(SOCKET_PATH.to_string()),
                e.to_string()
            )
        })?;
        let msg = simd_json::to_string(&api_call)
            .map_err(|e| sherlock_error!(SherlockErrorType::SerializationError(), e.to_string()))?;
        stream.write_sized(msg.as_bytes())?;
        Ok(())
    }
}
