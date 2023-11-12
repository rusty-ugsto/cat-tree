use super::service::ListFilesService;
use crate::traits::builder::Builder;
use std::{collections::HashSet, fs::FileType, path::PathBuf};

pub type ListFilesServiceCallback = Box<dyn Fn(usize, FileType, PathBuf)>;
pub type ListFilesServiceErrorCallback = Box<dyn Fn(String)>;

#[derive(Default)]
pub struct ListFilesServiceBuilder {
    root: Option<PathBuf>,
    exclude: Option<HashSet<PathBuf>>,
    max_depth: Option<usize>,
    all: Option<bool>,
    follow_links: Option<bool>,
    flatten: Option<bool>,
    callback: Option<ListFilesServiceCallback>,
    error_callback: Option<ListFilesServiceErrorCallback>,
}

impl ListFilesServiceBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn root(mut self, root: PathBuf) -> Self {
        self.root = Some(root);
        self
    }

    pub fn exclude(mut self, exclude: HashSet<PathBuf>) -> Self {
        self.exclude = Some(exclude);
        self
    }

    pub fn max_depth(mut self, max_depth: Option<usize>) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub fn all(mut self, all: bool) -> Self {
        self.all = Some(all);
        self
    }

    pub fn follow_links(mut self, follow_links: bool) -> Self {
        self.follow_links = Some(follow_links);
        self
    }

    pub fn flatten(mut self, flatten: bool) -> Self {
        self.flatten = Some(flatten);
        self
    }

    pub fn callback(mut self, callback: Box<dyn Fn(usize, FileType, PathBuf)>) -> Self {
        self.callback = Some(callback);
        self
    }

    pub fn error_callback(mut self, error_callback: Box<dyn Fn(String)>) -> Self {
        self.error_callback = Some(error_callback);
        self
    }
}

impl Builder<ListFilesService> for ListFilesServiceBuilder {
    fn build(self) -> ListFilesService {
        ListFilesService {
            root: self.root.unwrap(),
            exclude: self.exclude.unwrap(),
            max_depth: self.max_depth,
            all: self.all.unwrap(),
            follow_links: self.follow_links.unwrap(),
            flatten: self.flatten.unwrap(),
            callback: self.callback.unwrap(),
            error_callback: self.error_callback.unwrap(),
        }
    }
}
