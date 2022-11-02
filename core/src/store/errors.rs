use crate::schema::SchemaError;
use crate::serial::SerialError;

#[derive(Debug)]
pub enum PersistenceError {
    SchemaError(SchemaError),
    SerialError(SerialError),
    QueryInvalid(String),
    Poisoned
}

impl From<SchemaError> for PersistenceError {
    fn from(err: SchemaError) -> Self {
        Self::SchemaError(err)
    }
}

impl From<SerialError> for PersistenceError {
    fn from(err: SerialError) -> Self {
        Self::SerialError(err)
    }
}
