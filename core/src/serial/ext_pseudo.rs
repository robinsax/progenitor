use crate::schema::Value;

use super::errors::SerialError;
use super::value::SerialValue;
use super::format::{SerialFormat, SerialFormatReader, SerialFormatWriter};

pub struct PseudoSerialWriter {
    value: Value
}

impl From<PseudoSerialWriter> for SerialValue {
    fn from(serial: PseudoSerialWriter) -> Self {
        SerialValue::Pseudo(serial.value)
    }
}

impl TryFrom<Value> for PseudoSerialWriter {
    type Error = SerialError;

    fn try_from(value: Value) -> Result<Self, SerialError> {
        Ok(PseudoSerialWriter { value })
    }
}

impl SerialFormatWriter for PseudoSerialWriter {
    fn flush(self) -> Result<SerialValue, SerialError> {
        Ok(SerialValue::Pseudo(self.value))
    }

    fn write(&mut self, value: Value) -> Result<(), SerialError> {
        self.value = value;

        Ok(())
    }
}

pub struct PseudoSerialReader {
    value: Option<Value>
}

impl From<SerialValue> for PseudoSerialReader {
    fn from(serial: SerialValue) -> Self {
        let value = match serial {
            SerialValue::Pseudo(value) => Some(value),
            _ => None
        };

        Self { value }
    }
}

impl TryFrom<PseudoSerialReader> for Value {
    type Error = SerialError;

    fn try_from(serial: PseudoSerialReader) -> Result<Self, Self::Error> {
        match serial.value {
            // TODO: Yeah, this is far from a beautiful abstraction.
            None => Err(Self::Error::DataFormat("Invalid serial format for this reader".into())),
            Some(value) => Ok(value)
        }
    }
}

impl SerialFormatReader for PseudoSerialReader {
    fn elements(&self) -> Result<Vec<Self>, SerialError> {
        match &self.value {
            None => Err(SerialError::DataFormat("Invalid serial format for this reader".into())),
            Some(value) => {
                match value {
                    Value::List(contents) => {
                        Ok(contents.iter().map(
                            |item| Self::from(SerialValue::Pseudo(item.clone()))
                        ).collect())
                    },
                    _ => Err(SerialError::DataContent("Elements lookup on non-list".into()))
                }
            }
        }
    }

    fn lookup(&self, key: &str) -> Result<Self, SerialError> {
        match &self.value {
            None => Err(SerialError::DataFormat("Invalid serial format for this reader".into())),
            Some(value) => {
                match value {
                    Value::Map(contents) => {
                        match contents.get(key.into()) {
                            None => Err(SerialError::DataContent(format!("Key not present: {}", key))),
                            Some(inner) => Ok(Self::from(SerialValue::Pseudo(inner.clone())))
                        }
                    },
                    _ => Err(SerialError::DataContent("Key lookup on non-map".into()))
                }
            }
        }
    }
}

pub struct PseudoSerial;

impl SerialFormat for PseudoSerial {
    type Reader = PseudoSerialReader;
    type Writer = PseudoSerialWriter;

    fn new_reader(value: SerialValue) -> Self::Reader {
        let value_option = match value {
            SerialValue::Pseudo(inner) => Some(inner),
            _ => None
        };

        Self::Reader { value: value_option }
    }

    fn new_writer() -> Self::Writer {
        Self::Writer { value: Value::Null }
    }
}
