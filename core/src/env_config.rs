use std::env;

use crate::errors::InitError;

#[derive(Clone)]
pub struct EnvConfig;

impl EnvConfig {
    pub fn new() -> Self {
        Self
    }

    pub fn get_var(&self, str: &str) -> Result<String, InitError> {
        // TODO: Why can't the Result auto-cast?
        env::var(str).or_else(|e| Err(e.into()))
    }
}

pub trait FromEnvConfig
where
    Self: Sized
{
    fn try_from_config(env_config: EnvConfig) -> Result<Self, InitError>;
}
