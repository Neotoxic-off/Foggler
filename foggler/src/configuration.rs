use toml;
use std::fs;
use log::{debug, error};

use crate::models::servers::Servers;

pub struct Configuration {
    pub path: String,
    pub server_configuration: Servers
}

impl Configuration {
    pub fn new(path: String) -> Self {
        Self {
            path,
            server_configuration: Servers { servers: vec![] }
        }
    }

    pub fn load(&mut self) -> () {
        if let Ok(result) = self.read_configuration() {
            match toml::from_str(&result) {
                Ok(deserialized) => {
                    self.server_configuration = deserialized;
                    debug!("server configuration loaded");
                },
                Err(e) => {
                    error!("server configuration loading failed: {}", e);
                }
            }
        }
    }

    fn read_configuration(&mut self) -> Result<String, std::io::Error> {
        match fs::read_to_string(&self.path) {
            Ok(result) => {
                debug!("configuration file '{}' read", self.path);
                
                Ok(result)
            },
            Err(e) => {
                error!("configuration file '{}' read failed: {}", self.path, e);

                Err(e)
            }
        }

    }
}
