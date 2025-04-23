use clap::Parser;
use tokio;
use tokio::time::{sleep, Duration};

mod arguments;
mod configuration;
mod checker;
mod core;
mod models;
mod logs;

async fn wait(time: u64) -> bool {
    sleep(Duration::from_secs(time)).await;

    time > 0
}

#[tokio::main]
async fn main() {
    let mut alive: bool = true;
    let arguments: arguments::Arguments = arguments::Arguments::parse();

    if let Ok(_) = logs::init(&arguments.logs) {
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
