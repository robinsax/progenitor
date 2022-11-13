use crate::schema::Value;

use super::errors::SerialError;
use super::value::SerialValue;

pub trait SerialFormat {
    fn parse(&self, serial: SerialValue) -> Result<Value, SerialError>;
    fn write(&self, value: &Value) -> Result<SerialValue, SerialError>;
}
