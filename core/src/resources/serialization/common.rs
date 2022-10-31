use bytes::Bytes;

use crate::archetype::LiteralValue;

#[derive(Debug)]
pub enum SerialError {
    TODO
}

pub trait Serial:
    TryInto<Bytes, Error = SerialError> +
    TryFrom<Bytes, Error = SerialError> +
    TryInto<LiteralValue, Error = SerialError> +
    TryFrom<LiteralValue, Error = SerialError> {
    // TODO lookup, itemwise cast, etc
}
