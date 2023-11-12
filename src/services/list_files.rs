use crate::traits::service::Service;
use std::path::{Path, PathBuf};

pub struct ListFilesService {
    root: PathBuf,
    exclude: Vec<PathBuf>,
    max_depth: Option<u8>,
    all: bool,
    callback: Box<dyn Fn(PathBuf)>,
    error_callback: Box<dyn Fn(String)>,
}

impl ListFilesService {
    pub fn new(
        root: PathBuf,
        exclude: Vec<PathBuf>,
        max_depth: Option<u8>,
        all: bool,
        callback: Box<dyn Fn(PathBuf)>,
        error_callback: Box<dyn Fn(String)>,
    ) -> Self {
        Self {
            root,
            exclude,
            max_depth,
            all,
            callback,
            error_callback,
        }
    }

    fn is_excluded(&self, path: &Path) -> bool {
        self.exclude.iter().any(|exclude| path == *exclude)
    }

    fn list_files_recursive(&self, root: &PathBuf, _depth: u8) {
        if self.max_depth.is_some() && _depth >= self.max_depth.unwrap() {
            return;
        }

        let entries = std::fs::read_dir(root);

        if let Err(error) = entries {
            (self.error_callback)(error.to_string());
            return;
        }

        for entry in entries.unwrap() {
            if let Err(error) = entry {
                (self.error_callback)(error.to_string());
                continue;
            }

            let entry = entry.unwrap();
            let path = entry.path();

            if path.starts_with(".") && !self.all {
                continue;
            }

            if path.is_dir() {
                self.list_files_recursive(&entry.path(), _depth + 1);
                continue;
            }

            if self.is_excluded(path.as_path()) {
                continue;
            }

            (self.callback)(path);
        }
    }
}

impl Service for ListFilesService {
    fn execute(&self) {
        self.list_files_recursive(&self.root, 0);
    }
}
