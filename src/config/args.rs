use super::structs::PartialConfig;
use crate::traits::loader::Loader;
use clap::Parser;
use std::{collections::HashSet, path::PathBuf};

#[derive(Parser)]
pub struct Args {
    /// Root directory to start the tree display
    #[clap()]
    pub root: Option<PathBuf>,

    /// Exclude certain file types or directories from the display
    #[clap(short, long)]
    pub exclude: Vec<PathBuf>,

    /// Define the maximum depth of directory traversal
    #[clap(short, long)]
    pub max_depth: Option<usize>,

    /// Optionally display the size of each file next to its name
    #[clap(short, long)]
    pub size: Option<bool>,

    /// Include hidden files and directories in the display
    #[clap(short, long, action=clap::ArgAction::SetTrue)]
    pub all: Option<bool>,

    /// Follow symbolic links
    #[clap(short='l', long, action=clap::ArgAction::SetTrue)]
    pub follow_links: Option<bool>,

    /// Flatten the directory structure (same indentation for any level)
    #[clap(short, long, action=clap::ArgAction::SetTrue)]
    pub flatten: Option<bool>,

    /// Template for the file name
    #[clap(
        short = 'F',
        long,
        default_value = "{{ indent }}<{{ path }}> [{{ file_type }}]"
    )]
    pub file_display_template: String,
}

impl Loader<Args> for Args {
    fn load() -> Args {
        Args::parse()
    }
}

impl From<Args> for PartialConfig {
    fn from(args: Args) -> Self {
        Self {
            root: args.root,
            exclude: HashSet::from_iter(args.exclude),
            max_depth: args.max_depth,
            size: args.size,
            all: args.all,
            follow_links: args.follow_links,
            flatten: args.flatten,
            file_display_template: Some(args.file_display_template),
        }
    }
}
