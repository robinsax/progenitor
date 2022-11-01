use bytes::Bytes;

use super::errors::SchemaError;
use super::indirect_value::IndirectValue;

pub struct SerialValue {
    pub data: Bytes,
}

impl From<SerialValue> for Bytes {
    fn from(value: SerialValue) -> Self {
        value.data
    }
}

impl From<Bytes> for SerialValue {
    fn from(data: Bytes) -> Self {
        SerialValue { data }
    }
}

impl SerialValue {
    pub fn new_write_buffer() -> Self {
        Self { data: Bytes::new() }
    }

    pub fn empty() -> Self {
        Self { data: Bytes::new() }
    }
}

// TODO break in half? pros and cons...
pub trait SerialFormat:
    From<SerialValue> +
    Into<SerialValue> +
    TryInto<IndirectValue, Error = SchemaError> +
    TryFrom<IndirectValue, Error = SchemaError> {
    fn new_writer() -> Self;
    fn new_reader(bytes: Bytes) -> Self;
    // TODO more of this and less of lazy from indirect
    fn lookup(&self, key: &str) -> Result<Self, SchemaError>;
    fn elements(&self) -> Result<Vec<Self>, SchemaError>;
    fn write(&mut self, indirect: IndirectValue) -> Result<(), SchemaError>;
    fn flush(&self) -> Result<SerialValue, SchemaError>; // TODO make consume self
}
