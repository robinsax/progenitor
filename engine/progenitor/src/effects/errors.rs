use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::SchemaError;
use crate::errors::InitError;
use crate::serial::SerialError;
use crate::store::StoreError;
use crate::state::StateError;

#[derive(Debug)]
pub enum EffectError {
    Missing(String),
    State(StateError),
    Serial(SerialError),
    Store(StoreError),
    Schema(SchemaError),
    Stack(String, Box<EffectError>),
    Internal(String)
}

impl From<StateError> for EffectError {
    fn from(err: StateError) -> Self {
        Self::State(err)
    }
}

impl From<SerialError> for EffectError {
    fn from(err: SerialError) -> Self {
        Self::Serial(err)
    }
}

impl From<StoreError> for EffectError {
    fn from(err: StoreError) -> Self {
        Self::Store(err)
    }
}

impl From<SchemaError> for EffectError {
    fn from(err: SchemaError) -> Self {
        Self::Schema(err)
    }
}

// TODO: This, better.
impl From<InitError> for EffectError {
    fn from(init: InitError) -> Self {
        Self::Internal(format!("{:?}", init))
    }
}

impl Display for EffectError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Missing(name) => write!(f, "effect was expected to exist but doesn't: {}", name),
            Self::State(err) => write!(f, "state invalidated: {}", err),
            Self::Serial(err) => write!(f, "serialization error: {}", err),
            Self::Store(err) => write!(f, "persistence layer error: {}", err),
            Self::Schema(err) => write!(f, "invalid schema: {}", err),
            Self::Stack(name, inner) => write!(f, "{}/ {}", name, inner),
            Self::Internal(message) => write!(f, "internal: {}", message)
        }
    }
}

impl Error for EffectError {}
