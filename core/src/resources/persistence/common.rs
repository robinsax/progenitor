use crate::archetype::SchemaError;

#[derive(Debug)]
pub enum PersistenceError {
    SchemaError(SchemaError),
    TODO
}

impl From<SchemaError> for PersistenceError {
    fn from(err: SchemaError) -> Self {
        Self::SchemaError(err)
    }
}
