use nix::fcntl::{fcntl, FcntlArg, OFlag};
use std::io::{self, Read};
use std::os::unix::io::AsRawFd;

use super::Loader;

impl Loader {
    pub fn load_pipe_args() -> String {
        let mut stdin = io::stdin();
        let mut buffer = String::new();

        let fd = stdin.lock().as_raw_fd();

        match fcntl(fd, FcntlArg::F_GETFL) {
            Ok(flags) => {
                let flags = OFlag::from_bits_truncate(flags);
                if let Ok(_) = fcntl(fd, FcntlArg::F_SETFL(OFlag::O_NONBLOCK | flags)) {
                    let _ = stdin.read_to_string(&mut buffer);

                    return buffer;
                }
            }
            Err(_) => {}
        }
        String::new()
    }
}
