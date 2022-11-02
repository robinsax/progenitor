// Serialization format abstraction.
// TODO: Async conversions?
use crate::schema::Value;

use super::errors::SerialError;
use super::value::SerialValue;

// TODO: Not too sure about the TryFroms.

pub trait SerialFormatWriter
where
    Self: Into<SerialValue> + TryFrom<Value, Error = SerialError>
{
    //  TODO: Improve this a lot.
    fn write(&mut self, value: Value) -> Result<(), SerialError>;
    fn flush(self) -> Result<SerialValue, SerialError>;
}

pub trait SerialFormatReader
where
    Self: From<SerialValue> + TryInto<Value, Error = SerialError>
{
    fn lookup(&self, key: &str) -> Result<Self, SerialError>;
    fn elements(&self) -> Result<Vec<Self>, SerialError>;
}

pub trait SerialFormat
where
    Self::Reader: SerialFormatReader,
    Self::Writer: SerialFormatWriter
{
    type Writer;
    type Reader;

    fn new_writer() -> Self::Writer;
    fn new_reader(value: SerialValue) -> Self::Reader;
}
