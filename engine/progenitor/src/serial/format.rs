use crate::schema::Value;

use super::errors::SerialError;
use super::value::SerialValue;

pub trait SerialFormat {
    fn parse(serial: SerialValue) -> Result<Value, SerialError>;
    fn write(value: &Value) -> Result<SerialValue, SerialError>;
}
