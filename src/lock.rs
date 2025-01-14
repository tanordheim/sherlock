use std::fs::{remove_file, File};


pub fn ensure_single_instance(lock_file: &str) -> Result<LockFile, String> {
    LockFile::new(lock_file)
}

pub struct LockFile {
    pub path: String,
}

impl LockFile {
    pub fn new(path: &str) -> Result<Self, String> {
        match File::create(path) {
            Ok(_) => Ok(LockFile { path: path.to_string() }),
            Err(e) => {
                let message = format!("Error occurred: {} \n {}", e, path);
                Err(message)
            }
        }
    }

    pub fn remove(&self) {
        if let Err(e) = remove_file(&self.path) {
            eprintln!("Failed to remove lock file: {}", e);
        }
    }
}

impl Drop for LockFile {
    fn drop(&mut self) {
        self.remove(); 
    }
}

