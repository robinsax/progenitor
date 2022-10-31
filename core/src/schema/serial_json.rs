use std::collections::HashMap;

use serde_json::{
    from_str, to_string,
    Value as SerdeValue, Error as SerdeError, Number as SerdeNumber, Map as SerdeMap
};
use bytes::Bytes;

use super::errors::SchemaError;
use super::indirect_value::{IndirectValue};
use super::serial::{SerialFormat, SerialValue};

impl From<SerdeError> for SchemaError {
    fn from(err: SerdeError) -> Self {
        SchemaError::TODO(format!("serde err {}", err))
    }
}

impl From<&SerdeValue> for IndirectValue {
    fn from(serde_value: &SerdeValue) -> Self {
        serde_value.clone().into()
    }
}

impl From<SerdeValue> for IndirectValue {
    fn from(serde_value: SerdeValue) -> Self {
        match serde_value {
            SerdeValue::Null => IndirectValue::Null,
            SerdeValue::Bool(value) => IndirectValue::Bool(value),
            SerdeValue::String(value) => IndirectValue::String(value),
            SerdeValue::Number(value) => { // TODO inner unwraps safe outers arnt
                if value.is_f64() {
                    IndirectValue::Float64(value.as_f64().unwrap())
                }
                else if value.is_i64() {
                    IndirectValue::Int32(i32::try_from(value.as_i64().unwrap()).unwrap())
                }
                else {
                    IndirectValue::Uint32(u32::try_from(value.as_u64().unwrap()).unwrap())
                }
            },
            SerdeValue::Array(value) => {
                IndirectValue::List(value.iter().map(|e| e.into()).collect())
            },
            SerdeValue::Object(value) => {
                let mut indirect: HashMap<String, IndirectValue> = HashMap::new();

                for (key, element) in value {
                    indirect.insert(key, element.into());
                }

                IndirectValue::Map(indirect)
            }
        }
    }
}

impl From<&IndirectValue> for SerdeValue {
    fn from(indirect_value: &IndirectValue) -> Self {
        indirect_value.clone().into()
    }
}

impl From<IndirectValue> for SerdeValue {
    fn from(indirect_value: IndirectValue) -> Self {
        match indirect_value {
            IndirectValue::Null => SerdeValue::Null,
            IndirectValue::Bool(value) => SerdeValue::Bool(value),
            IndirectValue::String(value) => SerdeValue::String(value),
            IndirectValue::Int32(value) => SerdeValue::Number(value.into()),
            IndirectValue::Uint32(value) => SerdeValue::Number(value.into()),
            IndirectValue::Float64(value) => {
                SerdeValue::Number(SerdeNumber::from_f64(value).unwrap()) // TODO !
            },
            IndirectValue::List(value) => {
                SerdeValue::Array(value.iter().map(|e| e.into()).collect())
            },
            IndirectValue::Map(value) => {
                let mut indirect: SerdeMap<String, SerdeValue> = SerdeMap::new();

                for (key, element) in value {
                    indirect.insert(key, element.into());
                }

                SerdeValue::Object(indirect.into())
            }
        }
    }
}

// TODO hack + slow impl still
pub struct JsonSerial {
    value: Option<SerialValue>,
    parse_result: Option<Result<IndirectValue, SchemaError>>
}

impl From<SerialValue> for JsonSerial {
    fn from(value: SerialValue) -> Self {
        // preserve result to fail on read, works around the fact we for now pre-parse
        let parse_result = String::from_utf8(value.data.clone().into())
            .or_else(|_| Err(SchemaError::TODO("parse bytes to utf8".into())))
            .and_then(
                |str|
                        from_str::<SerdeValue>(&str)
                            .or_else(|_| Err(SchemaError::TODO("serde parse as serde value".into())))
            )
            .and_then(|value| Ok(IndirectValue::from(value)));

        JsonSerial { value: Some(value), parse_result: Some(parse_result) }
    }
}

impl From<JsonSerial> for SerialValue {
    fn from(json: JsonSerial) -> Self {
        if let Some(value) = json.value {
            value
        }
        else {
            // TODO are these unwraps okay w/ internal invariants? probably...
            let serde_value: SerdeValue = json.parse_result.unwrap().unwrap().into();
            let bytes: Bytes = to_string(&serde_value).unwrap().into();

            SerialValue::from(bytes)
        }
    }
}

impl TryFrom<IndirectValue> for JsonSerial {
    type Error = SchemaError;

    fn try_from(value: IndirectValue) -> Result<Self, Self::Error> {
        let serde_value: SerdeValue = value.clone().into();

        let bytes: Bytes = to_string(&serde_value)?.into();

        Ok(Self { value: Some(bytes.into()), parse_result: Some(Ok(value)) })
    }
}

impl TryFrom<JsonSerial> for IndirectValue {
    type Error = SchemaError;

    fn try_from(json: JsonSerial) -> Result<Self, Self::Error> {
        match json.parse_result {
            Some(Ok(result)) => Ok(result),
            _ => Err(SchemaError::TODO("json ser to indirect".into()))
        }
    }
}

impl SerialFormat for JsonSerial {
    fn new_writer() -> Self {
        Self { value: Some(SerialValue::new_write_buffer()), parse_result: None }
    }

    fn new_reader(bytes: Bytes) -> Self {
        let value: SerialValue = bytes.into();

        value.into()
    }

    fn lookup(&self, key: &str) -> Result<Self, SchemaError> {
        match &self.parse_result {
            Some(Ok(indirect)) => Ok(indirect.lookup(key)?.try_into()?),
            _ => Err(SchemaError::TODO("json fmt state error or parse failed".into())), // TODO really a state error, maybe break these out
        }
    }

    fn elements(&self) -> Result<Vec<Self>, SchemaError> {
        match &self.parse_result {
            Some(Ok(IndirectValue::List(value))) => {
                let mut format_wraps = Vec::new();

                for element in value {
                    format_wraps.push(element.clone().try_into()?);
                }

                Ok(format_wraps)
            },
            _ => Err(SchemaError::TODO("json elements lookup".into()))
        }
    }

    fn write(&mut self, indirect: IndirectValue) -> Result<(), SchemaError> {
        let serde_value: SerdeValue = indirect.clone().into();

        let bytes: Bytes = to_string(&serde_value)?.into();

        self.value = Some(bytes.into());
        self.parse_result = Some(Ok(indirect));

        Ok(())
    }

    fn flush(self) -> Result<SerialValue, SchemaError> {
        match self.value {
            None => Err(SchemaError::TODO("json fmt flush state error".into())),
            Some(value) => Ok(value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::serial_repr::SerialRepr;
    use super::*;
    
    #[derive(Clone)]
    pub struct Foo {
        a: String,
        b: i32,
        c: bool,
        d: Vec<u32>
    }

    // XXX note to self; effectively achieved ser here with into indirect, but won't be the case long term
    impl TryFrom<Foo> for IndirectValue {
        type Error = SchemaError;
        
        fn try_from(inst: Foo) -> Result<Self, Self::Error> {
            let mut fields = HashMap::new();

            let src = inst.clone();

            fields.insert("a".into(), src.a.try_into()?);
            fields.insert("b".into(), src.b.try_into()?);
            fields.insert("c".into(), src.c.try_into()?);

            let mut d = Vec::new();
            for element in src.d {
                d.push(element.try_into()?);
            }

            fields.insert("d".into(), IndirectValue::List(d));

            Ok(Self::Map(fields))
        }
    }
    
    impl SerialRepr for Foo {
        fn deserialize(serial: impl SerialFormat) -> Result<Self, SchemaError> {
            use super::super::serial_repr::macros::*;

            let elements = serial.lookup("d")?;
            let d = elements_auto!(elements);

            Ok(Self {
                a: lookup_auto!(serial, "a"),
                b: lookup_auto!(serial, "b"),
                c: lookup_auto!(serial, "c"),
                d
            })
        }
    
        fn serialize(&self, mut serial: impl SerialFormat) -> Result<SerialValue, SchemaError> {
            let dup = self.clone(); // TODO bruh
            serial.write(dup.try_into()?)?;

            serial.flush()
        }
    }
    
    // TODO
    #[test]
    fn test_serial_roundtrip() {
        let mut f = Foo {
            a: "a".into(),
            b: 12,
            c: true,
            d: Vec::new()
        };

        f.d.push(23);

        let s = f.serialize(JsonSerial::new_writer()).expect("json write failed");
        let b: Bytes = s.data.clone();

        assert_eq!(String::from_utf8(b.into()).expect("serialized value valid"), "{\"a\":\"a\",\"b\":12,\"c\":true,\"d\":[23]}");

        let g = Foo::deserialize(JsonSerial::from(s)).expect("json read failed");

        assert_eq!(f.a, g.a);
        assert_eq!(f.b, g.b);
        assert_eq!(f.c, g.c);
        assert_eq!(f.d, g.d);
    }
}
