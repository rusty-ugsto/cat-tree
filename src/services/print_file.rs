use crate::traits::service::Service;
use std::path::PathBuf;

pub struct PrintFileService {
    path: PathBuf,
    _error_callback: Box<dyn Fn(String)>,
}

impl PrintFileService {
    pub fn _new(path: PathBuf, error_callback: Box<dyn Fn(String)>) -> Self {
        Self {
            path,
            _error_callback: error_callback,
        }
    }
}

impl Service for PrintFileService {
    fn execute(&self) {
        println!("{}", self.path.display());
    }
}
