use clap::Parser;
use tokio;
use tokio::time::{sleep, Duration};
use log::{error, debug};

mod arguments;
mod configuration;
mod checker;
mod core;
mod models;
mod logs;

fn load_logger(debug: bool, path: &str) -> bool {
    match logs::init(debug, path) {
        Ok(()) => {
            true
        },
        Err(e) => {
            error!("{}", e);
            false
        }
    }
}

async fn wait(time: u64) -> bool {
    debug!("waiting {}s", time);

    sleep(Duration::from_secs(time)).await;

    time > 0
}

#[tokio::main]
async fn main() {
    let arguments: arguments::Arguments = arguments::Arguments::parse();
    let loaded: bool = load_logger(arguments.debug, &arguments.logs);
    let mut alive: bool = true;

    if loaded == true {
        let mut core: core::Core = core::Core::new(
            arguments.servers,
            arguments.port,
            arguments.timeout
        );
        
        core.load();

        while alive {
            core.run().await;
            alive = wait(arguments.wait).await;
        }
    }
}
