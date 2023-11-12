use std::{collections::HashSet, path::PathBuf};

#[derive(Default, Debug)]
pub struct PartialConfig {
    pub root: Option<PathBuf>,
    pub exclude: HashSet<PathBuf>,
    pub max_depth: Option<usize>,
    pub size: Option<bool>,
    pub all: Option<bool>,
}

#[derive(Debug)]
pub struct Config {
    pub root: PathBuf,
    pub exclude: HashSet<PathBuf>,
    pub max_depth: Option<usize>,
    pub size: bool,
    pub all: bool,
}
