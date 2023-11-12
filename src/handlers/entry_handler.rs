use crate::{
    traits::{parser::Parser, service::Service},
    utils::file_type_to_string::FileTypeToString,
};
use std::{fs::FileType, path::PathBuf};

pub struct EntryHandler {
    depth: usize,
    file_type: FileType,
    path: PathBuf,
    _error_callback: Box<dyn Fn(String)>,
}

impl EntryHandler {
    pub fn new(
        depth: usize,
        file_type: FileType,
        path: PathBuf,
        error_callback: Box<dyn Fn(String)>,
    ) -> Self {
        Self {
            depth,
            file_type,
            path,
            _error_callback: error_callback,
        }
    }
}

impl Service for EntryHandler {
    fn execute(&self) {
        println!(
            "{}<{}> [{}]",
            "  ".repeat(self.depth),
            self.path.display(),
            FileTypeToString::new(&self.file_type).parse(),
        );
    }
}
