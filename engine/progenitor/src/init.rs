use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum InitError {
    Archetype(String),
    Config(String),
    State(String)
}

impl Display for InitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<init error: {:?}>", self)
    }
}

impl Error for InitError {}

pub trait InitConfig {
    fn read(&self, key: &str) -> Result<String, InitError>;
}

pub trait ConfigInit
where
    Self: Sized
{
    fn from_config(config: &Box<dyn InitConfig>) -> Result<Self, InitError>;
}
