use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum SerialError {
    Format(String),
    Parse(String)
}

impl Display for SerialError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "serial error: {:?}",self)
    }
}

impl Error for SerialError {}
