use crate::{
    traits::{parser::Parser, service::Service},
    utils::file_type_to_string::FileTypeToString,
};
use handlebars::Handlebars;
use std::{collections::BTreeMap, fs::FileType, path::PathBuf};

pub struct EntryHandler {
    depth: usize,
    file_type: FileType,
    path: PathBuf,
    file_display_template: String,
    _error_callback: Box<dyn Fn(String)>,
}

impl EntryHandler {
    pub fn new(
        depth: usize,
        file_type: FileType,
        path: PathBuf,
        file_display_template: String,
        error_callback: Box<dyn Fn(String)>,
    ) -> Self {
        Self {
            depth,
            file_type,
            path,
            file_display_template,
            _error_callback: error_callback,
        }
    }
}

impl Service for EntryHandler {
    fn execute(&self) {
        let mut handlebars = Handlebars::new();
        let _ = handlebars
            .register_template_string("file_display_template", &self.file_display_template);

        let indent = "  ".repeat(self.depth);
        let file_type = FileTypeToString::new(&self.file_type).parse().to_string();
        let path = self.path.to_str().unwrap().to_string();

        let data = BTreeMap::from([("indent", indent), ("file_type", file_type), ("path", path)]);

        println!(
            "{}",
            handlebars.render("file_display_template", &data).unwrap()
        );
    }
}
