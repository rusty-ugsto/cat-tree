use crate::{
    traits::{handler::Handler, parser::Parser},
    utils::file_type_to_string::FileTypeToString,
};
use handlebars::Handlebars;
use std::{collections::BTreeMap, fs::FileType, path::PathBuf};

pub struct EntryHandler {
    depth: usize,
    file_type: FileType,
    path: PathBuf,
    file_display_template: String,
}

impl EntryHandler {
    pub fn new(
        depth: usize,
        file_type: FileType,
        path: PathBuf,
        file_display_template: String,
    ) -> Self {
        Self {
            depth,
            file_type,
            path,
            file_display_template,
        }
    }
}

impl Handler for EntryHandler {
    fn execute(&self) -> Result<(), String> {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string("file_display_template", &self.file_display_template)
            .map_err(|e| e.to_string())?;

        let indent = "  ".repeat(self.depth);
        let file_type = FileTypeToString::new(&self.file_type).parse().to_string();
        let path = self
            .path
            .to_str()
            .ok_or("Failed to convert path to string")?
            .to_string();

        let data = BTreeMap::from([("indent", indent), ("file_type", file_type), ("path", path)]);

        println!(
            "{}",
            handlebars.render("file_display_template", &data).unwrap()
        );

        if self.file_type.is_file() {
            let content = std::fs::read_to_string(&self.path)
                .map_err(|e| e.to_string())?
                .to_string();
            let content = content
                .lines()
                .map(|l| {
                    if l.is_empty() {
                        l.to_string()
                    } else {
                        format!("{}{}", "  ".repeat(self.depth + 1), l)
                    }
                })
                .collect::<Vec<String>>()
                .join("\n");

            println!("{}", content);
        }

        Ok(())
    }
}
