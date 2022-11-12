use std::error::Error;
use std::fmt::Display;

use progenitor::{SerialError, SchemaError};

#[derive(Debug)]
pub enum ExecError {
    Io(String)
}

impl From<SerialError> for ExecError {
    fn from(err: SerialError) -> Self {
        ExecError::Io(format!("{}", err))
    }
}

impl From<SchemaError> for ExecError {
    fn from(err: SchemaError) -> Self {
        ExecError::Io(format!("{}", err))
    }
}

impl Display for ExecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(message) => write!(f, "io error\n\n{}", message)
        }
    }
}

impl Error for ExecError {}
