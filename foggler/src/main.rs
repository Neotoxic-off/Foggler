use clap::Parser;
use tokio;

mod arguments;
mod configuration;
mod checker;
mod core;
mod models;

use crate::arguments::Arguments;

fn init_logger(debug: bool) {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", if debug { "debug" } else { "info" });
    }

    let _ = env_logger::Builder::from_default_env().try_init();
}

#[tokio::main]
async fn main() {
    let arguments: Arguments = Arguments::parse();
    init_logger(arguments.debug);

    let mut core: core::Core = core::Core::new(
        arguments.servers,
        arguments.port,
        arguments.timeout
    );
    
    core.load();
    core.run().await;
}
