use std::collections::HashMap;
use std::fs;
use std::net::ToSocketAddrs;
use std::time::Instant;
use log::{info, error};
use serde::Deserialize;
use tokio;

#[derive(Debug, Deserialize)]
struct Config {
    servers: HashMap<String, String>,
}

fn load_servers() -> Config {
    let toml_str: String = fs::read_to_string("servers.toml")
        .expect("Failed to read servers.toml");
    toml::from_str(&toml_str)
        .expect("Failed to parse servers.toml")
}

pub fn init_logger() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    let _ = env_logger::Builder::from_default_env().try_init();
}

#[tokio::main]
async fn main() {
    init_logger();

    let config: Config = load_servers();
    let mut handles: Vec<tokio::task::JoinHandle<(String, String, bool, u64)>> = vec![];
    for (name, url) in config.servers.iter() {
        let name: String = name.clone();
        let url: String = url.clone();
        
        let handle = tokio::spawn(async move {
            match dns_check(&url).await {
                Ok(duration_ms) => {
                    let formatted_name = format!("{:<20}", name);
                    info!("{} UP ({}ms)", formatted_name, duration_ms);
                    (name, url, true, duration_ms)
                },
                Err(e) => {
                    let formatted_name = format!("{:<20}", name);
                    error!("{} DOWN: {}", formatted_name, e);
                    (name, url, false, 0)
                }
            }
        });
        
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }
}

async fn dns_check(host: &str) -> Result<u64, String> {
    let start = Instant::now();
    let socket_addr = format!("{}:443", host);

    let socket_addr_clone = socket_addr.clone();
    let result = tokio::task::spawn_blocking(move || {
        socket_addr_clone.to_socket_addrs()
    }).await.map_err(|e| format!("Task join error: {}", e))?;
    
    match result {
        Ok(mut addrs) => {
            if addrs.next().is_some() {
                let duration = start.elapsed();
                let duration_ms = duration.as_millis() as u64;
                Ok(duration_ms)
            } else {
                Err("No addresses found".to_string())
            }
        },
        Err(e) => Err(format!("Resolution error: {}", e)),
    }
}
