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
