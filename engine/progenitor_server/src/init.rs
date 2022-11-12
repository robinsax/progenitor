use std::env;

use progenitor::{InitConfig, InitError};

pub struct ServerInitConfig;

impl InitConfig for ServerInitConfig {
    fn read(&self, key: &str) -> Result<String, InitError> {
        match env::var(key.to_owned().to_uppercase()) {
            Ok(val) => Ok(val),
            Err(_) => Err(InitError::Config(key.to_owned()))
        }
    }
}

impl ServerInitConfig {
    pub fn new() -> Self {
        Self
    }
}
