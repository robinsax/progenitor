use crate::archetype::SchemaError;

#[derive(Debug)]
pub enum StorageOpError {
    SchemaError(SchemaError),
    TODO
}

impl From<SchemaError> for StorageOpError {
    fn from(err: SchemaError) -> Self {
        Self::SchemaError(err)
    }
}
