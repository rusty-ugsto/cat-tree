use super::structs::{Config, PartialConfig};
use crate::traits::builder::Builder;
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
            self.config.exclude = other
                .exclude
                .iter()
                .filter(|s| !s.is_empty())
                .cloned()
                .collect();
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
            root: self.config.root.unwrap_or_default(),
            exclude: self.config.exclude,
            max_depth: self.config.max_depth,
            size: self.config.size.unwrap_or(false),
            all: self.config.all.unwrap_or(false),
        }
    }
}
