use super::structs::PartialConfig;
use crate::traits::loader::Loader;
use clap::Parser;
use std::path::PathBuf;

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
    pub max_depth: Option<u8>,

    /// Optionally display the size of each file next to its name
    #[clap(short, long)]
    pub size: Option<bool>,

    /// Include hidden files and directories in the display
    #[clap(short, long)]
    pub all: Option<bool>,
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
            exclude: args.exclude,
            max_depth: args.max_depth,
            size: args.size,
            all: args.all,
        }
    }
}
