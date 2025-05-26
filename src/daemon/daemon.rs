use std::os::unix::net::UnixStream;

use crate::utils::errors::{SherlockError, SherlockErrorType};
use crate::{sherlock_error, SOCKET_PATH};
use std::io::{Read, Write};
use std::os::unix::net::UnixListener;

pub struct SherlockDaemon {
    socket: String,
}
impl SherlockDaemon {
    pub async fn new(pipeline: async_channel::Sender<String>) -> Self {
        let _ = std::fs::remove_file(SOCKET_PATH);
        let listener = UnixListener::bind(SOCKET_PATH).expect("Failed to bind socket");
        println!("Daemon listening on {}", SOCKET_PATH);

        for stream in listener.incoming() {
            if let Ok(mut stream) = stream {
                let mut buffer = [0; 1024];
                match stream.read(&mut buffer) {
                    Ok(bytes_read) => {
                        if bytes_read > 0 {
                            let received_data = String::from_utf8_lossy(&buffer[..bytes_read]);
                            let received_data = received_data.trim();
                            match received_data {
                                "show" => {
                                    let _ = pipeline.send(String::from("open-window")).await;
                                }
                                _ => {
                                    let _ = pipeline.send(received_data.to_string()).await;
                                }
                            }
                            let _ = stream.write_all(b"OK\n");
                        }
                    }
                    Err(e) => {
                        let _ = stream.write_all(format!("Error encountered: {:?}", e).as_bytes());
                        eprintln!("Error: {:?}", e)
                    }
                }
            }
        }
        Self {
            socket: SOCKET_PATH.to_string(),
        }
    }
    fn remove(&self) -> Result<(), SherlockError> {
        std::fs::remove_file(&self.socket).map_err(|e| {
            sherlock_error!(
                SherlockErrorType::SocketRemoveError(self.socket.clone()),
                e.to_string()
            )
        })?;
        Ok(())
    }
    pub fn open() -> Result<(), SherlockError> {
        let mut stream = UnixStream::connect(SOCKET_PATH).map_err(|e| {
            sherlock_error!(
                SherlockErrorType::SocketConnectError(SOCKET_PATH.to_string()),
                e.to_string()
            )
        })?;
        stream.write_all(b"show").map_err(|e| {
            sherlock_error!(
                SherlockErrorType::SocketWriteError(SOCKET_PATH.to_string()),
                e.to_string()
            )
        })?;

        Ok(())
    }
}

impl Drop for SherlockDaemon {
    fn drop(&mut self) {
        let _ = self.remove();
    }
}
