use tokio::task;
use tracing::{info, error};

use crate::configuration::Configuration;
use crate::checker::ping;

pub struct Core {
    configuration: Configuration,
    socket_port: u16,
    timeout: u64
}

impl Core {
    pub fn new(configuration: String, socket_port: u16, timeout: u64) -> Self {
        Self {
            configuration: Configuration::new(configuration),
            socket_port,
            timeout
        }
    }

    pub fn load(&mut self) {
        self.configuration.load();
    }

    async fn check(name: String, url: String, port: u16, timeout: u64) -> (bool, String, String) {
        match ping::ping(&url, port, timeout).await {
            Ok(time) => {
                (true, name, format!("{}ms", time))
            }
            Err(e) => {
                (false, name, e)
            }
        }
    }

    async fn render(&self, results: &Vec<(bool, String, String)>) -> () {
        for (success, name, result) in results {
            if *success == true {
                info!(server = name, ping = result);
            } else {
                error!(server = name, ping = result);
            } 
        }
    }

    pub async fn run(&mut self) {
        let mut handles: Vec<task::JoinHandle<(bool, String, String)>> = vec![];

        for server in self.configuration.server_configuration.servers.iter() {
            let name: String = server.name.clone();
            let url: String = server.url.clone();
            let port: u16 = self.socket_port;
            let timeout: u64 = self.timeout;

            let handle: task::JoinHandle<(bool, String, String)> = task::spawn(async move {
                Core::check(name, url, port, timeout).await
            });

            handles.push(handle);
        }

        let mut results: Vec<(bool, String, String)> = vec![];

        for handle in handles {
            if let Ok(result) = handle.await {
                results.push(result);
            }
        }
    
        self.render(&results).await;
    }
}
