use crate::traits::service::Service;
use std::path::PathBuf;

pub struct PrintFileService {
    path: PathBuf,
}

impl PrintFileService {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Service for PrintFileService {
    fn execute(&self) {
        println!("{}", self.path.display());
        let content = std::fs::read_to_string(&self.path)
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
