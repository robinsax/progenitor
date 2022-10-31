// TODO as extension, also complete hack currently

use std::string::FromUtf8Error;

use bytes::Bytes;

use crate::archetype::LiteralValue;

use super::common::{Serial, SerialError};

pub struct SerialJson {
    data: String,
}

impl From<FromUtf8Error> for SerialError {
    fn from(_: FromUtf8Error) -> Self {
        SerialError::TODO
    }
}

impl TryFrom<Bytes> for SerialJson {
    type Error = SerialError;

    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        Ok(Self {
            data: String::from_utf8(bytes.into())?
        })
    }
}

impl TryFrom<SerialJson> for Bytes {
    type Error = SerialError;

    fn try_from(json: SerialJson) -> Result<Self, Self::Error> {
        Ok(json.data.into())
    }
}

// TODO guh this is so dumb
use serde_json::{to_string, Map, from_str, Value};

fn to_serde(val: LiteralValue) -> Result<Value, SerialError> {
    Ok(match val {
        LiteralValue::Null { .. } => Value::Null,
        LiteralValue::Float64 { value } => Value::from(value),
        LiteralValue::Int32 { value } => Value::from(value),
        LiteralValue::Uint32 { value } => Value::from(value),
        LiteralValue::String { value } => Value::from(value),
        LiteralValue::List { value, .. } => {
            let mut values: Vec<Value> = Vec::new();
            for inner in value {
                values.push(to_serde(inner)?);
            }

            Value::from(values)
        },
        LiteralValue::Object { value,.. } => {
            let mut values: Map<String, Value> = Map::new();

            for (key, inner) in value.iter() {
                values.insert(key.into(), to_serde(inner.clone())?);
            }

            Value::from(values)
        },
    })
}

fn from_serde(value: Value) -> Result<LiteralValue, SerialError> {
    Err(SerialError::TODO)
}

impl TryFrom<LiteralValue> for SerialJson {
    type Error = SerialError;

    fn try_from(value: LiteralValue) -> Result<Self, Self::Error> {
        let serializable = to_serde(value)?;
        
        match to_string(&serializable) {
            Ok(string) => Ok(SerialJson{data: string}),
            Err(_) => Err(SerialError::TODO)
        }
    }
}

impl TryFrom<SerialJson> for LiteralValue {
    type Error = SerialError;

    fn try_from(value: SerialJson) -> Result<Self, Self::Error> {
        let serde_val = match from_str(&value.data) {
            Ok(str) => str,
            Err(_) => return Err(SerialError::TODO)
        };
    
        from_serde(serde_val)
    }
}
// marker
impl Serial for SerialJson {}
