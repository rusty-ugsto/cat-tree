use super::structs::{Config, PartialConfig};
use crate::traits::builder::Builder;
use std::path::PathBuf;
pub struct ConfigBuilder {
    pub config: PartialConfig,
}

impl ConfigBuilder {
    pub fn new() -> ConfigBuilder {
        ConfigBuilder {
            config: PartialConfig::default(),
        }
    }

    pub fn merge(mut self, other: PartialConfig) -> Self {
        self.config.root = other.root.or(self.config.root);
        if !other.exclude.is_empty() {
            self.config.exclude = other.exclude;
        }
        self.config.max_depth = other.max_depth.or(self.config.max_depth);
        self.config.size = other.size.or(self.config.size);
        self.config.all = other.all.or(self.config.all);

        self
    }
}

impl Builder<Config> for ConfigBuilder {
    fn build(self) -> Config {
        Config {
            root: self.config.root.unwrap_or_else(|| PathBuf::from(".")),
            exclude: self.config.exclude,
            max_depth: self.config.max_depth,
            size: self.config.size.unwrap_or(false),
            all: self.config.all.unwrap_or(false),
            follow_links: self.config.follow_links.unwrap_or(false),
            flatten: self.config.flatten.unwrap_or(false),
            file_display_template: self.config.file_display_template.unwrap_or_default(),
        }
    }
}
