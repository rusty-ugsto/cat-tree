use services::{list_files::ListFilesService, print_file::PrintFileService};
use traits::service::Service;

use crate::{
    config::{args::Args, builder::ConfigBuilder, env::Env},
    traits::{builder::Builder, loader::Loader},
};

mod config;
mod services;
mod traits;

fn main() {
    let config = ConfigBuilder::new()
        .merge(Env::new().into())
        .merge(Args::load().into())
        .build();

    ListFilesService::new(
        config.root,
        config.exclude,
        config.max_depth,
        config.all,
        Box::new(|path| PrintFileService::new(path).execute()),
    )
    .execute();
}
