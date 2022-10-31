use crate::schema::SchemaError;

pub struct ConnectionOptions {
    pub uri: String,
    pub access_key: String,
    pub access_secret: String
}

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
