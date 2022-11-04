use bytes::Bytes;

use crate::schema::Value;

use super::errors::SerialError;

// Intermediate container for serial data.
pub enum SerialValue {
    Buffer(Bytes),
    // TODO: First class stream, maybe even async.
    Stream,
    // TODO: Do we actually want this?
    Pseudo(Value)
}

impl TryFrom<SerialValue> for Bytes {
    type Error = SerialError;

    fn try_from(value: SerialValue) -> Result<Self, Self::Error> {
        match value {
            SerialValue::Buffer(bytes) => Ok(bytes),
            _ => Err(Self::Error::NotImplemented("Conversion to bytes from this format".into()))
        }
    }
}

impl From<Bytes> for SerialValue {
    fn from(data: Bytes) -> Self {
        SerialValue::Buffer(data)
    }
}

impl SerialValue {
    pub fn new_write_buffer() -> Self {
        Self::Buffer(Bytes::new())
    }

    pub fn empty() -> Self {
        Self::Buffer(Bytes::new())
    }

    // TODO: Both of these are only try because of representable invalid state which isn't Rusty.
    pub fn try_into_bytes(self) -> Result<Bytes, SerialError> {
        match self {
            Self::Buffer(data) => Ok(data),
            _ => Err(SerialError::NotImplemented("Can't extract bytes (wrong variant)".into()))
        }
    }

    pub fn try_clone_buffer(&self) -> Result<SerialValue, SerialError> {
        match self {
            Self::Buffer(data) => Ok(data.clone().into()),
            _ => Err(SerialError::NotImplemented("Can't clone buffer (wrong variant)".into()))
        }
    }
}
