use crate::traits::service::Service;
use std::{
    collections::HashSet,
    fs::FileType,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub struct ListFilesService {
    root: PathBuf,
    exclude: HashSet<PathBuf>,
    max_depth: Option<usize>,
    all: bool,
    follow_links: bool,
    callback: Box<dyn Fn(usize, FileType, PathBuf)>,
    error_callback: Box<dyn Fn(String)>,
}

impl ListFilesService {
    pub fn new(
        root: PathBuf,
        exclude: HashSet<PathBuf>,
        max_depth: Option<usize>,
        all: bool,
        follow_links: bool,
        callback: Box<dyn Fn(usize, FileType, PathBuf)>,
        error_callback: Box<dyn Fn(String)>,
    ) -> Self {
        Self {
            root,
            exclude,
            max_depth,
            all,
            follow_links,
            callback,
            error_callback,
        }
    }

    fn is_excluded(&self, path: &Path) -> bool {
        self.exclude.contains(path)
    }

    fn is_hidden(&self, path: &Path) -> bool {
        path.file_name()
            .map(|name| name.to_string_lossy().starts_with('.'))
            .unwrap_or(false)
    }

    fn is_max_depth_reached(&self, depth: usize) -> bool {
        self.max_depth
            .map(|max_depth| depth >= max_depth)
            .unwrap_or(false)
    }

    fn should_skip(&self, depth: usize, path: &Path) -> bool {
        self.is_excluded(path)
            || (self.is_hidden(path) && !self.all)
            || self.is_max_depth_reached(depth)
    }
}

impl Service for ListFilesService {
    fn execute(&self) {
        println!("{}", self.follow_links);
        let walker = WalkDir::new(&self.root)
            .follow_links(self.follow_links)
            .into_iter()
            .filter_entry(|entry| !self.should_skip(entry.depth(), entry.path()));
        for entry in walker {
            let entry = match entry {
                Ok(entry) => entry,
                Err(error) => {
                    (self.error_callback)(error.to_string());
                    continue;
                }
            };

            let depth = entry.depth();
            (self.callback)(depth, entry.file_type(), entry.path().to_path_buf());
        }
    }
}
