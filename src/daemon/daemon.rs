use std::os::unix::net::UnixStream;

use crate::loader::Loader;
use crate::utils::errors::{SherlockError, SherlockErrorType};
use crate::{sher_log, sherlock_error, SOCKET_PATH};
use std::io::{Read, Write};
use std::os::unix::net::UnixListener;

pub struct SherlockDaemon {
    socket: String,
}
impl SherlockDaemon {
    pub async fn new(pipeline: async_channel::Sender<String>) -> Self {
        let _ = std::fs::remove_file(SOCKET_PATH);
        let listener = UnixListener::bind(SOCKET_PATH).expect("Failed to bind socket");
        sher_log!(format!("Daemon listening on {}", SOCKET_PATH));
        println!("Daemon listening on {}", SOCKET_PATH);

        for stream in listener.incoming() {
            if let Ok(mut stream) = stream {
                match stream.read_sized() {
                    Ok(bytes_read) => {
                        if bytes_read.len() > 0 {
                            let received_data = String::from_utf8_lossy(&bytes_read);
                            let received_data = received_data.trim();
                            match received_data {
                                "Show" => {
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

        let pipe = Loader::load_pipe_args();
        if pipe.is_empty() {
            stream.write_sized(b"Show")?;
        } else {
            stream.write_sized(pipe.as_slice())?;
        }

        Ok(())
    }
}

impl Drop for SherlockDaemon {
    fn drop(&mut self) {
        let _ = self.remove();
    }
}

pub trait SizedMessage {
    fn write_sized(&mut self, buf: &[u8]) -> Result<(), SherlockError>;
    fn read_sized(&mut self) -> Result<Vec<u8>, SherlockError>;
}
impl SizedMessage for UnixStream {
    fn write_sized(&mut self, buf: &[u8]) -> Result<(), SherlockError> {
        let buf_len = buf.len() as u32;
        self.write_all(&buf_len.to_be_bytes()).map_err(|e| {
            sherlock_error!(
                SherlockErrorType::SocketWriteError(SOCKET_PATH.to_string()),
                e.to_string()
            )
        })?;
        self.write_all(buf).map_err(|e| {
            sherlock_error!(
                SherlockErrorType::SocketWriteError(SOCKET_PATH.to_string()),
                e.to_string()
            )
        })?;

        Ok(())
    }
    fn read_sized(&mut self) -> Result<Vec<u8>, SherlockError> {
        let mut buf_len = [0u8; 4];
        self.read_exact(&mut buf_len).map_err(|e| {
            sherlock_error!(
                SherlockErrorType::SocketWriteError(SOCKET_PATH.to_string()),
                e.to_string()
            )
        })?;
        let msg_len = u32::from_be_bytes(buf_len) as usize;

        let mut buf = vec![0u8; msg_len];
        self.read_exact(&mut buf).map_err(|e| {
            sherlock_error!(
                SherlockErrorType::SocketWriteError(SOCKET_PATH.to_string()),
                e.to_string()
            )
        })?;

        Ok(buf)
    }
}
