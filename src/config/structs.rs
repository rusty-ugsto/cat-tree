use std::{collections::HashSet, path::PathBuf};

#[derive(Default)]
pub struct PartialConfig {
    pub root: Option<PathBuf>,
    pub exclude: HashSet<PathBuf>,
    pub max_depth: Option<usize>,
    pub size: Option<bool>,
    pub all: Option<bool>,
    pub follow_links: Option<bool>,
    pub flatten: Option<bool>,
    pub file_display_template: Option<String>,
    pub content_display_template: Option<String>,
}

#[derive(Clone)]
pub struct Config {
    pub root: PathBuf,
    pub exclude: HashSet<PathBuf>,
    pub max_depth: Option<usize>,
    pub size: bool,
    pub all: bool,
    pub follow_links: bool,
    pub flatten: bool,
    pub file_display_template: String,
    pub content_display_template: String,
}
