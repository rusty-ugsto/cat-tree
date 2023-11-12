use crate::{
    config::{args::Args, builder::ConfigBuilder, env::Env},
    traits::{builder::Builder, loader::Loader},
};
use handlers::error_handler::ErrorHandler;
use services::{list_files::ListFilesService, print_file::PrintFileService};
use traits::{handler::Handler, service::Service};

mod config;
mod handlers;
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
        Box::new(|path| {
            PrintFileService::new(path, Box::new(|error| ErrorHandler::new(error).execute()))
                .execute()
        }),
        Box::new(|error| ErrorHandler::new(error).execute()),
    )
    .execute();
}
