use super::structs::PartialConfig;
use std::{collections::HashSet, env, path::PathBuf};

pub struct Env {
    root: Option<PathBuf>,
    exclude: HashSet<PathBuf>,
    max_depth: Option<usize>,
    size: Option<bool>,
    all: Option<bool>,
    follow_links: Option<bool>,
    flatten: Option<bool>,
    file_display_template: Option<String>,
}

impl Env {
    pub fn new() -> Env {
        let exclude = env::var("CAT_TREE_EXCLUDE")
            .unwrap_or_default()
            .split(',')
            .filter(|s| !s.is_empty())
            .map(PathBuf::from)
            .collect();

        Env {
            root: env::var("CAT_TREE_ROOT").ok().map(PathBuf::from),
            exclude,
            max_depth: env::var("CAT_TREE_MAX_DEPTH")
                .ok()
                .map(|s| s.parse().unwrap()),
            size: env::var("CAT_TREE_SIZE")
                .ok()
                .map(|s| s.to_lowercase() == "true"),
            all: env::var("CAT_TREE_ALL")
                .ok()
                .map(|s| s.to_lowercase() == "true"),
            follow_links: env::var("CAT_TREE_FOLLOW_LINKS")
                .ok()
                .map(|s| s.to_lowercase() == "true"),
            flatten: env::var("CAT_TREE_FLATTEN")
                .ok()
                .map(|s| s.to_lowercase() == "true"),
            file_display_template: env::var("CAT_TREE_file_display_TEMPLATE").ok(),
        }
    }
}

impl From<Env> for PartialConfig {
    fn from(env: Env) -> Self {
        Self {
            root: env.root,
            exclude: env.exclude,
            max_depth: env.max_depth,
            size: env.size,
            all: env.all,
            follow_links: env.follow_links,
            flatten: env.flatten,
            file_display_template: env.file_display_template,
        }
    }
}
