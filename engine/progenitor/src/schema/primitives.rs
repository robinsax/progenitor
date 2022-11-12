// Indirect representations of types and data.
use std::{mem, collections::HashMap};

use super::errors::SchemaError;

// TODO: Needs more variants. Is null a type here?
#[derive(Debug, Clone)]
pub enum Type {
    Bool,
    Int32,
    Uint32,
    Float64,
    String,
    // TODO: Investigate whether this is the canonical way to prevent infinite size.
    List(Box<Type>),
    Map(HashMap<String, Type>)
}

impl Type {
    pub fn lookup(&self, key: &str) -> Result<Type, SchemaError> {
        if let Self::Map(inner) = self {
            if let Some(value) = inner.get(key) {
                return Ok(value.clone());
            }
        }
        
        Err(SchemaError::InvalidLookup(Some(self.clone()), key.into()))
    }

    pub fn primitive_eq(&self, other: &Type) -> bool {
        match self {
            Self::List(_) => false,
            Self::Map(_) => false,
            rest => mem::discriminant(rest) == mem::discriminant(other)
        }
    }

    // TODO: Strictness options.
    pub fn validate(&self, value: &Value) -> Result<(), SchemaError> {
        let value_t = value.try_into()?;
        
        match self {
            Self::List(inner_t) => {
                match value {
                    Value::List(value_members) => {
                        for member in value_members.iter() {
                            inner_t.validate(member)?;
                        }

                        Ok(())
                    },
                    _ => Err(SchemaError::InvalidType(self.clone(), value_t))
                }
            },
            Self::Map(inner_ts) => {
                match value {
                    Value::Map(value_members) => {
                        for (key, inner_t) in inner_ts.iter() {
                            let member = match value_members.get(key) {
                                Some(member) => member,
                                None => return Err(SchemaError::MissingKey(key.clone()))
                            };

                            inner_t.validate(member)?;
                        }

                        Ok(())
                    },
                    _ => Err(SchemaError::InvalidType(self.clone(), value_t))
                }
            },
            _ => {
                if self.primitive_eq(&value_t) {
                    Ok(())
                }
                else {
                    Err(SchemaError::InvalidType(self.clone(), value_t))
                }
            }
        }
    }
}

// Indirectly represented data, with (some) type encapsulation.
// TODO: More variants.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int32(i32),
    Uint32(u32),
    Float64(f64),
    String(String),
    List(Vec<Value>),
    Map(HashMap<String, Value>)
}

impl Value {
    pub fn lookup(&self, key: &str) -> Result<Value, SchemaError> {
        if let Self::Map(inner) = self {
            if let Some(value) = inner.get(key) {
                return Ok(value.clone());
            }
        }

        Err(SchemaError::InvalidLookup(Type::try_from(self).ok(), key.into()))
    }
}

impl TryFrom<&Value> for Type {
    type Error = SchemaError;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        value.clone().try_into()
    }
}

impl TryFrom<Value> for Type {
    type Error = SchemaError;

    fn try_from(container: Value) -> Result<Self, Self::Error> {
        Ok(match container {
            Value::Null => return Err(SchemaError::UnknownableType),
            Value::Bool(_) => Self::Bool,
            Value::Int32(_) => Self::Int32,
            Value::Uint32(_) => Self::Uint32,
            Value::Float64(_) => Self::Float64,
            Value::String(_) => Self::String,
            Value::List(members) => {
                if members.len() == 0 {
                    return Err(Self::Error::UnknownableType);
                }
                
                Self::List(Box::new((&members[0]).try_into()?))
            },
            Value::Map(members) => {
                let mut schema = HashMap::new();

                for (key, member) in members.into_iter() {
                    schema.insert(key.into(), member.try_into()?);
                }

                Self::Map(schema)
            }
        })
    }
}

// TODO: Clean up below here.
macro_rules! primitive_from_indirect_implementation {
    ($v: path, $t: ty) => {
        impl TryFrom<Value> for $t {
            type Error = SchemaError;

            fn try_from(indirect: Value) -> Result<Self, Self::Error> {
                match indirect {
                    $v(value) => Ok(value),
                    case => Err(SchemaError::NotImplemented(format!("Indirect cast to {} failed: {:?}", stringify!($t), case)))
                }
            }
        }

        impl TryFrom<Value> for Option<$t> {
            type Error = SchemaError;

            fn try_from(indirect: Value) -> Result<Self, Self::Error> {
                Ok(match indirect {
                    Value::Null => None,
                    _ => Some(indirect.try_into()?)
                })
            }
        }
    };
}

macro_rules! primitive_to_indirect_implementation {
    ($v: path, $t: ty) => {
        // TODO ever fails?
        impl TryFrom<$t> for Value {
            type Error = SchemaError;

            fn try_from(data: $t) -> Result<Self, Self::Error> {
                Ok($v(data))
            }
        }

        impl TryFrom<Option<$t>> for Value {
            type Error = SchemaError;

            fn try_from(option: Option<$t>) -> Result<Self, Self::Error> {
                match option {
                    Some(data) => Self::try_from(data),
                    None => Ok(Self::Null)
                } 
            }
        }
    };
}

primitive_from_indirect_implementation!(Value::Bool, bool);
primitive_from_indirect_implementation!(Value::Int32, i32);
primitive_from_indirect_implementation!(Value::Float64, f64);
primitive_from_indirect_implementation!(Value::String, String);

impl TryFrom<Value> for u32 {
    type Error = SchemaError;

    fn try_from(indirect: Value) -> Result<Self, Self::Error> {
        match indirect {
            Value::Uint32(value) => Ok(value),
            Value::Int32(value) => {
                match u32::try_from(value) {
                    Ok(unsigned) => Ok(unsigned),
                    Err(_) => Err(SchemaError::NotImplemented("Cast of i32 -> u32 failed".into()))
                }
            },
            case => Err(SchemaError::NotImplemented(format!("Indirect cast to u32 failed: {:?}", case)))
        }
    }
}

impl TryFrom<Value> for Option<u32> {
    type Error = SchemaError;

    fn try_from(indirect: Value) -> Result<Self, Self::Error> {
        Ok(match indirect {
            Value::Null => None,
            _ => Some(indirect.try_into()?)
        })
    }
}

primitive_to_indirect_implementation!(Value::Bool, bool);
primitive_to_indirect_implementation!(Value::Int32, i32);
primitive_to_indirect_implementation!(Value::Uint32, u32);
primitive_to_indirect_implementation!(Value::Float64, f64);
primitive_to_indirect_implementation!(Value::String, String);
