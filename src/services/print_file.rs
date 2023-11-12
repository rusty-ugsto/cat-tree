use crate::traits::service::Service;
use std::path::PathBuf;

pub struct PrintFileService {
    path: PathBuf,
    error_callback: Box<dyn Fn(String)>,
}

impl PrintFileService {
    pub fn new(path: PathBuf, error_callback: Box<dyn Fn(String)>) -> Self {
        Self {
            path,
            error_callback,
        }
    }
}

impl Service for PrintFileService {
    fn execute(&self) {
        println!("{}", self.path.display());

        let content = std::fs::read_to_string(&self.path);

        if let Err(error) = content {
            (self.error_callback)(error.to_string());
            return;
        }

        let content = content
            .unwrap()
            .lines()
            .map(|line| {
                if line.is_empty() {
                    line.to_string()
                } else {
                    format!("  {}", line)
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        println!("{}", content);
    }
}
