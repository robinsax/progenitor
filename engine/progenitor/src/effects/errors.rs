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
    Duplicate(String),
    State(StateError),
    Serial(SerialError),
    Store(StoreError),
    Schema(SchemaError),
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
        write!(f, "effect error: {:?}", self)
    }
}

impl Error for EffectError {}
