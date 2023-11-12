use crate::{
    config::{args::Args, builder::ConfigBuilder, env::Env},
    traits::{builder::Builder, loader::Loader},
};
use handlers::{entry_handler::EntryHandler, error_handler::ErrorHandler};
use services::list_files::builder::ListFilesServiceBuilder;
use traits::{handler::Handler, service::Service};

mod config;
mod handlers;
mod services;
mod traits;
mod utils;

fn main() {
    let config = ConfigBuilder::new()
        .merge(Env::new().into())
        .merge(Args::load().into())
        .build();

    ListFilesServiceBuilder::new()
        .root(config.root)
        .exclude(config.exclude)
        .max_depth(config.max_depth)
        .all(config.all)
        .follow_links(config.follow_links)
        .flatten(config.flatten)
        .callback(Box::new(|depth, file_type, path| {
            EntryHandler::new(
                depth,
                file_type,
                path,
                Box::new(|error| {
                    ErrorHandler::new(error).execute();
                }),
            )
            .execute();
        }))
        .error_callback(Box::new(|error| {
            ErrorHandler::new(error).execute();
        }))
        .build()
        .execute();
}
