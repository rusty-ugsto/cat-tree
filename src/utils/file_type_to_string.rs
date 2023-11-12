use crate::traits::parser::Parser;
use std::fs::FileType;

pub struct FileTypeToString<'a> {
    input: &'a FileType,
}

impl<'a> FileTypeToString<'a> {
    pub fn new(input: &'a FileType) -> Self {
        Self { input }
    }
}

impl<'a> Parser<&'static str> for FileTypeToString<'a> {
    fn parse(&self) -> &'static str {
        if self.input.is_symlink() {
            "symlink"
        } else if self.input.is_dir() {
            "directory"
        } else if self.input.is_file() {
            "file"
        } else {
            "other"
        }
    }
}
