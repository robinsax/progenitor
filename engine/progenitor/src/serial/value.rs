use bytes::Bytes;

use super::errors::SerialError;

// TODO: First class stream, maybe even async.
// Intermediate container for serial data.
#[derive(Clone)] // TODO: No!
pub enum SerialValue {
    Buffer(Bytes)
}

impl SerialValue {
    pub fn from_bytes(bytes: Bytes) -> Self {
        Self::Buffer(bytes)
    }

    pub fn from_string(string: String) -> Self {
        Self::from_bytes(Bytes::from(string))
    }

    pub fn empty() -> Self {
        Self::Buffer(Bytes::new())
    }

    // TODO: This is try for future proofing (stream drop during flush for ex.).
    pub fn try_into_bytes(self) -> Result<Bytes, SerialError> {
        match self {
            Self::Buffer(data) => Ok(data)
        }
    }
}
