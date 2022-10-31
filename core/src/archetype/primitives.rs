use std::{collections::HashMap, mem};

use super::component::component;

#[derive(Debug)]
pub enum SchemaError {
    TODO
}

pub type ObjectSchema = HashMap<String, DataType>;

component! {
    pub enum DataType {  // TODO more + constraints
        Uint32,
        Int32,
        Float64,
        String,
        List{inner: Box<DataType>},
        Object{schema: ObjectSchema}
    }
}

impl DataType {
    pub fn primitive_match(&self, other: &DataType) -> bool {
        match self {
            DataType::List{ .. } => false,
            DataType::Object{ .. } => false,
            rest => mem::discriminant(rest) == mem::discriminant(other)
        }
    }
}

impl DataType {
    pub fn lookup(&self, lookup: &str) -> Result<DataType, SchemaError> {
        let (this_lookup, next_lookup) = match lookup.split_once(".") {
            Some((a, b)) => (a, Some(b)),
            None => (lookup, None)
        };

        match self {
            DataType::List { inner } => {
                match this_lookup.parse::<u8>() {
                    Ok(_) => Ok(inner.as_ref().clone()),
                    Err(_) => Err(SchemaError::TODO)
                }
            },
            DataType::Object{ schema } => {
                match schema.get(this_lookup) {
                    Some(inner) => {
                        match next_lookup {
                            Some(next) => inner.lookup(next),
                            None => Ok(inner.clone()),
                        }
                    },
                    None => Err(SchemaError::TODO)
                }
            },
            _ => Err(SchemaError::TODO)
        }
    }
}

component! {
    pub enum LiteralValue { // TODO more
        Null{real_type: Option<DataType>},
        Uint32{value: u32},
        Int32{value: i32},
        Float64{value: f64},
        String{value: String},
        List{value: Vec<LiteralValue>, inner_type: DataType},
        Object{value: HashMap<String, LiteralValue>, type_schema: ObjectSchema}
    }
}

impl LiteralValue {
    pub fn data_type(&self) -> DataType {
        match self {
            LiteralValue::Null { real_type } => {
                match real_type {
                    Some(inner) => inner.clone(),
                    None => panic!("TODO base abstraction")
                }
            },
            LiteralValue::Uint32{ .. } => DataType::Uint32,
            LiteralValue::Int32 { .. } => DataType::Int32,
            LiteralValue::Float64{ ..} => DataType::Float64,
            LiteralValue::String{ ..} => DataType::String,
            LiteralValue::List{ inner_type, .. } => DataType::List{ inner: Box::new(inner_type.clone()) },
            LiteralValue::Object{ type_schema, .. } => DataType::Object { schema: type_schema.clone() }
        }
    }

    //  TODO high-key copy paste from datatype
    pub fn lookup(&self, lookup: &str) -> Result<LiteralValue, SchemaError> {
        let (this_lookup, next_lookup) = match lookup.split_once(".") {
            Some((a, b)) => (a, Some(b)),
            None => (lookup, None)
        };

        match self {
            LiteralValue::List{ value, .. } => {
                match this_lookup.parse::<usize>() {
                    Ok(i) => match i >= value.len() {
                        true => Err(SchemaError::TODO),
                        false => Ok(value[i].clone())
                    },
                    Err(_) => Err(SchemaError::TODO)
                }
            },
            LiteralValue::Object{ value, .. } => {
                match value.get(this_lookup) {
                    Some(inner) => {
                        match next_lookup {
                            Some(next) => inner.lookup(next),
                            None => Ok(inner.clone()),
                        }
                    },
                    None => Err(SchemaError::TODO)
                }
            },
            _ => Err(SchemaError::TODO)
        }
    }
}

impl From<i32> for LiteralValue {
    fn from(value: i32) -> Self {
        LiteralValue::Int32{ value }
    }
}

impl From<u32> for LiteralValue {
    fn from(value: u32) -> Self {
        LiteralValue::Uint32{ value }
    }
}

impl From<f64> for LiteralValue {
    fn from(value: f64) -> Self {
        LiteralValue::Float64 { value }
    }
}

impl From<String> for LiteralValue {
    fn from(value: String) -> Self {
        LiteralValue::String { value }
    }
}
