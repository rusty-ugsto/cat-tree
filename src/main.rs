use crate::{
    config::{args::Args, builder::ConfigBuilder, env::Env},
    traits::{builder::Builder, loader::Loader},
};
use handlers::{entry_handler::EntryHandler, error_handler::ErrorHandler};
use services::list_files::builder::{ListFilesServiceBuilder, ListFilesServiceCallback};
use traits::{handler::Handler, service::Service};

mod config;
mod handlers;
mod services;
mod traits;
mod utils;

fn error_callback() -> Box<dyn Fn(String)> {
    Box::new(|error| {
        ErrorHandler::new(error).execute();
    })
}

fn list_file_service_callback(file_display_template: String) -> ListFilesServiceCallback {
    let callback = move |depth, file_type, path| {
        EntryHandler::new(
            depth,
            file_type,
            path,
            file_display_template.clone(),
            error_callback(),
        )
        .execute();
    };

    Box::new(callback)
}

fn main() {
    let config = ConfigBuilder::new()
        .merge(Env::new().into())
        .merge(Args::load().into())
        .build();

    ListFilesServiceBuilder::new()
        .root(config.root.clone())
        .exclude(config.exclude.clone())
        .max_depth(config.max_depth)
        .all(config.all)
        .follow_links(config.follow_links)
        .flatten(config.flatten)
        .callback(list_file_service_callback(config.file_display_template))
        .error_callback(error_callback())
        .build()
        .execute();
}
