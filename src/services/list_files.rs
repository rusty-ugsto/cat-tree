use crate::traits::service::Service;
use std::path::PathBuf;

pub struct ListFilesService {
    root: PathBuf,
    exclude: Vec<String>,
    max_depth: Option<u8>,
    all: bool,
    callback: Box<dyn Fn(PathBuf)>,
}

impl ListFilesService {
    pub fn new(
        root: PathBuf,
        exclude: Vec<String>,
        max_depth: Option<u8>,
        all: bool,
        callback: Box<dyn Fn(PathBuf)>,
    ) -> Self {
        Self {
            root,
            exclude,
            max_depth,
            all,
            callback,
        }
    }

    fn is_excluded(&self, path: &str) -> bool {
        self.exclude.iter().any(|exclude| path == exclude)
    }

    fn list_files_recursive(&self, root: &PathBuf, _depth: u8) {
        if self.max_depth.is_some() && _depth >= self.max_depth.unwrap() {
            return;
        }

        for entry in std::fs::read_dir(root).unwrap() {
            let entry = entry.unwrap();

            if entry.file_name().to_str().unwrap().starts_with('.') && !self.all {
                continue;
            }

            if entry.file_type().unwrap().is_dir() {
                self.list_files_recursive(&entry.path(), _depth + 1);
                continue;
            }

            let path = entry.path();

            if self.is_excluded(path.to_str().unwrap()) {
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
