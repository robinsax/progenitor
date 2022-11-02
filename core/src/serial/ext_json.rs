use std::collections::HashMap;
use std::string::FromUtf8Error;

// TODO: Temporary dependency. This entire module is a placeholder.
use serde_json as serde;

use crate::schema::Value;

use super::errors::SerialError;
use super::value::SerialValue;
use super::format::{SerialFormat, SerialFormatReader, SerialFormatWriter};

impl From<serde::Error> for SerialError {
    fn from(err: serde::Error) -> Self {
        SerialError::DataFormat(format!("<(todo)serde_err: {:?}>", err))
    }
}

impl From<FromUtf8Error> for SerialError {
    fn from(err: FromUtf8Error) -> Self {
        SerialError::DataFormat(format!("<uf8_err: {:?}>", err))
    }
}

impl From<&serde::Value> for Value {
    fn from(serde_value: &serde::Value) -> Self {
        serde_value.clone().into()
    }
}

impl From<serde::Value> for Value {
    fn from(serde_value: serde::Value) -> Self {
        match serde_value {
            serde::Value::Null => Self::Null,
            serde::Value::Bool(value) => Self::Bool(value),
            serde::Value::String(value) => Self::String(value),
            serde::Value::Number(value) => { // TODO inner unwraps safe outers arnt
                if value.is_f64() {
                    Self::Float64(value.as_f64().unwrap())
                }
                else if value.is_i64() {
                    Self::Int32(i32::try_from(value.as_i64().unwrap()).unwrap())
                }
                else {
                    Self::Uint32(u32::try_from(value.as_u64().unwrap()).unwrap())
                }
            },
            serde::Value::Array(value) => {
                Self::List(value.iter().map(|e| e.into()).collect())
            },
            serde::Value::Object(value) => {
                let mut indirect: HashMap<String, Self> = HashMap::new();

                for (key, element) in value {
                    indirect.insert(key, element.into());
                }

                Self::Map(indirect)
            }
        }
    }
}

impl From<&Value> for serde::Value {
    fn from(indirect_value: &Value) -> Self {
        indirect_value.clone().into()
    }
}

impl From<Value> for serde::Value {
    fn from(indirect_value: Value) -> Self {
        match indirect_value {
            Value::Null => Self::Null,
            Value::Bool(value) => Self::Bool(value),
            Value::String(value) => Self::String(value),
            Value::Int32(value) => Self::Number(value.into()),
            Value::Uint32(value) => Self::Number(value.into()),
            Value::Float64(value) => {
                Self::Number(serde::Number::from_f64(value).unwrap()) // TODO !
            },
            Value::List(value) => {
                Self::Array(value.iter().map(|e| e.into()).collect())
            },
            Value::Map(value) => {
                let mut indirect: serde::Map<String, Self> = serde::Map::new();

                for (key, element) in value {
                    indirect.insert(key, element.into());
                }

                Self::Object(indirect.into())
            }
        }
    }
}

pub struct JsonSerialWriter {
    value: Option<SerialValue>,
}

impl From<JsonSerialWriter> for SerialValue {
    fn from(json: JsonSerialWriter) -> Self {
        if let Some(value) = json.value {
            value
        }
        else {
            SerialValue::empty()
        }
    }
}

impl TryFrom<Value> for JsonSerialWriter {
    type Error = SerialError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        use bytes::Bytes;

        let serde_value: serde::Value = value.clone().into();

        let bytes: Bytes = serde::to_string(&serde_value)?.into();

        Ok(Self { value: Some(bytes.into()) })
    }
}

impl SerialFormatWriter for JsonSerialWriter {
    fn write(&mut self, indirect: Value) -> Result<(), SerialError> {
        use bytes::Bytes;

        let serde_value: serde::Value = indirect.clone().into();

        let bytes: Bytes = serde::to_string(&serde_value)?.into();

        self.value = Some(bytes.into());

        Ok(())
    }

    fn flush(self) -> Result<SerialValue, SerialError> {
        Ok(if let Some(serial) = self.value {
            serial
        }
        else {
            SerialValue::empty()
        })
    }
}

pub struct JsonSerialReader {
    // Preserve Result to fail on read; a further hack to target eventual semantics.
    value: Result<Value, SerialError>
}

impl From<SerialValue> for JsonSerialReader {
    fn from(value: SerialValue) -> Self {
        Self { value: Self::try_parse_eager(value) }
    }
}

impl TryFrom<JsonSerialReader> for Value {
    type Error = SerialError;

    fn try_from(json: JsonSerialReader) -> Result<Self, Self::Error> {
        json.value
    }
}

impl JsonSerialReader {
    fn try_parse_eager(value: SerialValue) -> Result<Value, SerialError> {
        let input_str = match value {
            SerialValue::Buffer(data) => {
                String::from_utf8(data.into())?
            },
            _ => return Err(SerialError::DataFormat("Only buffers supported yet".into()))
        };

        let serde_value = serde::from_str::<serde::Value>(&input_str)?;

        Ok(serde_value.into())
    }
}

impl SerialFormatReader for JsonSerialReader {
    fn lookup(&self, key: &str) -> Result<Self, SerialError> {
        Ok(Self { value: Ok(self.value.clone()?.lookup(key)?) })
    }

    fn elements(&self) -> Result<Vec<Self>, SerialError> {
        match &self.value.clone()? {
            Value::List(value) => {
                let mut format_wraps = Vec::new();

                for element in value {
                    format_wraps.push(Self { value: Ok(element.clone()) });
                }

                Ok(format_wraps)
            },
            _ => Err(SerialError::DataContent("elements() on non-list data".into()))
        }
    }
}

pub struct JsonSerial;

impl SerialFormat for JsonSerial {
    type Reader = JsonSerialReader;
    type Writer = JsonSerialWriter;

    fn new_reader(value: SerialValue) -> Self::Reader {
        value.into()
    }

    fn new_writer() -> Self::Writer {
        Self::Writer { value: Some(SerialValue::new_write_buffer()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::conversion::{StreamSerial, DirectSerial, elements_auto, lookup_auto};
    use crate::schema::Type;

    #[derive(Clone)]
    pub struct Foo {
        a: String,
        b: i32,
        c: bool,
        d: Vec<u32>
    }

    impl StreamSerial for Foo {
        fn schema() -> Type {
            let mut fields = HashMap::new();

            fields.insert("a".into(), Type::String);
            fields.insert("b".into(), Type::Int32);
            fields.insert("c".into(), Type::Bool);
            fields.insert("d".into(), Type::List(Box::new(Type::Uint32)));

            Type::Map(fields)
        }

        fn stream_deserialize(serial: &mut impl SerialFormatReader) -> Result<Self, SerialError> {
            let elements = serial.lookup("d")?;
            let d = elements_auto!(elements);

            Ok(Self {
                a: lookup_auto!(serial, "a"),
                b: lookup_auto!(serial, "b"),
                c: lookup_auto!(serial, "c"),
                d
            })
        }
    
        fn stream_serialize(self, serial: &mut impl SerialFormatWriter) -> Result<(), SerialError> {
            let mut fields = HashMap::new();

            let src = self.clone();

            fields.insert("a".into(), src.a.try_into()?);
            fields.insert("b".into(), src.b.try_into()?);
            fields.insert("c".into(), src.c.try_into()?);

            let mut d = Vec::new();
            for element in src.d {
                d.push(element.try_into()?);
            }

            fields.insert("d".into(), Value::List(d));

            serial.write(Value::Map(fields))
        }
    }
    
    // TODO
    #[test]
    fn serial_roundtrip() {
        let mut f = Foo {
            a: "a".into(),
            b: 12,
            c: true,
            d: Vec::new()
        };

        f.d.push(23);

        let serial = f.clone().serialize::<JsonSerial>().expect("json write failed");
        let raw_data = serial.try_clone_buffer().expect("Buffer extraction").try_into_bytes().expect("Bytes unwrap");

        assert_eq!(String::from_utf8(raw_data.into()).expect("serialized value valid"), "{\"a\":\"a\",\"b\":12,\"c\":true,\"d\":[23]}");

        let g = Foo::deserialize::<JsonSerial>(serial).expect("Json read failed");

        assert_eq!(f.a, g.a);
        assert_eq!(f.b, g.b);
        assert_eq!(f.c, g.c);
        assert_eq!(f.d, g.d);
    }
}
