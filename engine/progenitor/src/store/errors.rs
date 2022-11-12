use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::schema::SchemaError;

#[derive(Debug)]
pub enum StoreError {
    Schema(SchemaError),
    Query(String),
    Backend(String)
}

impl From<SchemaError> for StoreError {
    fn from(err: SchemaError) -> Self {
        Self::Schema(err)
    }
}

impl Display for StoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "store error: {:?}", self)
    }
}

impl Error for StoreError {}
