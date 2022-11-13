use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum StateError {
    Empty(String),
    InvalidType(String)
}

impl Display for StateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "state error: {:?}", self)
    }
}

impl Error for StateError {}
