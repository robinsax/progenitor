use crate::schema::SchemaError;

#[derive(Debug, Clone)]
pub enum SerialError {
    Schema(SchemaError),
    // TODO: Real diagnostic information, not strings.
    DataFormat(String),
    DataContent(String),
    NotImplemented(String)
}

impl From<SchemaError> for SerialError {
    fn from(err: SchemaError) -> Self {
        Self::Schema(err)
    }
}
