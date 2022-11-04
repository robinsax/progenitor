use std::sync::PoisonError;

use crate::schema::SchemaError;
use crate::serial::SerialError;

#[derive(Debug)]
pub enum StoreError {
    SchemaError(SchemaError),
    SerialError(SerialError),
    QueryInvalid(String),
    Poisoned,
    Backend
}

impl<T> From<PoisonError<T>> for StoreError {
    fn from(_: PoisonError<T>) -> StoreError {
        StoreError::Poisoned
    }
}

impl From<SchemaError> for StoreError {
    fn from(err: SchemaError) -> Self {
        Self::SchemaError(err)
    }
}

impl From<SerialError> for StoreError {
    fn from(err: SerialError) -> Self {
        Self::SerialError(err)
    }
}
