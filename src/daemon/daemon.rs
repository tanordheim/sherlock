use crate::loader::util::{SherlockError, SherlockErrorType};
use crate::ui::window::show_window;
use gtk4::glib::{self, ControlFlow};
use std::io::{Read, Write};
use std::os::unix::net::UnixListener;

pub struct SherlockDeamon {
    socket: String,
}
impl SherlockDeamon {
    pub fn new(socket_path: &str) -> Self {
        let _ = std::fs::remove_file(socket_path);
        let listener = UnixListener::bind(socket_path).expect("Failed to bind socket");
        println!("Daemon listening on {}", socket_path);

        for stream in listener.incoming() {
            if let Ok(mut stream) = stream {
                let mut buffer = [0; 1024];
                match stream.read(&mut buffer) {
                    Ok(bytes_read) => {
                        if bytes_read > 0 {
                            let received_data = String::from_utf8_lossy(&buffer[..bytes_read]);
                            match received_data.trim() {
                                "show" => {
                                    glib::idle_add(move || {
                                        show_window(true);
                                        ControlFlow::Break
                                    });
                                }
                                _ => println!("Received: {}", received_data),
                            }

                            let _ = stream.write_all(b"OK\n");
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {:?}", e)
                    }
                }
            }
        }
        Self {
            socket: socket_path.to_string(),
        }
    }
    fn remove(&self) -> Result<(), SherlockError> {
        std::fs::remove_file(&self.socket).map_err(|e| SherlockError {
            error: SherlockErrorType::SocketRemoveError(self.socket.clone()),
            traceback: e.to_string(),
        })?;
        Ok(())
    }
}

impl Drop for SherlockDeamon {
    fn drop(&mut self) {
        let _ = self.remove();
    }
}
