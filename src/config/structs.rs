use std::path::PathBuf;

#[derive(Default, Debug)]
pub struct PartialConfig {
    pub root: Option<PathBuf>,
    pub exclude: Vec<String>,
    pub max_depth: Option<u8>,
    pub size: Option<bool>,
    pub all: Option<bool>,
}

#[derive(Debug)]
pub struct Config {
    pub root: PathBuf,
    pub exclude: Vec<String>,
    pub max_depth: Option<u8>,
    pub size: bool,
    pub all: bool,
}
