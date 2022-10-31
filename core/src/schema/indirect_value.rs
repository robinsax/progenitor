use std::collections::HashMap;

use super::errors::SchemaError;
use super::indirect_type::IndirectType;

#[derive(Clone, Debug)]
pub enum IndirectValue {
    Null,
    Bool(bool),
    Int32(i32),
    Uint32(u32),
    Float64(f64),
    String(String),
    // TODO more primitive numbers
    List(Vec<IndirectValue>),
    Map(HashMap<String, IndirectValue>)
}

impl IndirectValue {
    pub fn lookup(&self, lookup: &str) -> Result<IndirectValue, SchemaError> {
        match self {
            Self::Map(value) => {
                match value.get(lookup.into()) {
                    Some(inner) => Ok(inner.clone()),
                    None => Err(SchemaError::TODO(format!("lookup indirect {} map key missing", lookup)))
                }
            },
            _ => Err(SchemaError::TODO(format!("lookup indirect {}", lookup)))
        }
    }
}

impl TryFrom<&IndirectValue> for IndirectType {
    type Error = SchemaError;

    fn try_from(value: &IndirectValue) -> Result<Self, Self::Error> {
        value.clone().try_into()
    }
}

impl TryFrom<IndirectValue> for IndirectType {
    type Error = SchemaError;

    fn try_from(container: IndirectValue) -> Result<Self, Self::Error> {
        Ok(match container {
            IndirectValue::Null => return Err(SchemaError::UnknownableType),
            IndirectValue::Bool(_) => Self::Bool,
            IndirectValue::Int32(_) => Self::Int32,
            IndirectValue::Uint32(_) => Self::Uint32,
            IndirectValue::Float64(_) => Self::Float64,
            IndirectValue::String(_) => Self::String,
            IndirectValue::List(members) => {
                if members.len() == 0 {
                    return Err(SchemaError::UnknownableType);
                }

                Self::List(Box::new((&members[0]).try_into()?))
            },
            IndirectValue::Map(members) => {
                let mut schema = HashMap::new();

                for (key, member) in members.iter() {
                    schema.insert(key.into(), member.try_into()?);
                }

                Self::Map(schema)
            }
        })
    }
}

macro_rules! prim_from_indirect_impl {
    ($v: path, $t: ty) => {
        impl TryFrom<IndirectValue> for $t {
            type Error = SchemaError;

            fn try_from(indirect: IndirectValue) -> Result<Self, Self::Error> {
                match indirect {
                    $v(value) => Ok(value),
                    case => Err(SchemaError::TODO(format!("try from indirect -> {} ({:?})", stringify!($t), case)))
                }
            }
        }

        impl TryFrom<IndirectValue> for Option<$t> {
            type Error = SchemaError;

            fn try_from(indirect: IndirectValue) -> Result<Self, Self::Error> {
                Ok(match indirect {
                    IndirectValue::Null => None,
                    _ => Some(indirect.try_into()?)
                })
            }
        }
    };
}

macro_rules! prim_to_indirect_impl {
    ($v: path, $t: ty) => {
        // TODO ever fails?
        impl TryFrom<$t> for IndirectValue {
            type Error = SchemaError;

            fn try_from(data: $t) -> Result<Self, Self::Error> {
                Ok($v(data))
            }
        }

        impl TryFrom<Option<$t>> for IndirectValue {
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

prim_from_indirect_impl!(IndirectValue::Bool, bool);
prim_from_indirect_impl!(IndirectValue::Int32, i32);
prim_from_indirect_impl!(IndirectValue::Float64, f64);
prim_from_indirect_impl!(IndirectValue::String, String);

impl TryFrom<IndirectValue> for u32 {
    type Error = SchemaError;

    fn try_from(indirect: IndirectValue) -> Result<Self, Self::Error> {
        match indirect {
            IndirectValue::Uint32(value) => Ok(value),
            IndirectValue::Int32(value) => {
                match u32::try_from(value) {
                    Ok(unsigned) => Ok(unsigned),
                    Err(_) => Err(SchemaError::TODO("i32 -> u32 failed".into()))
                }
            },
            case => Err(SchemaError::TODO(format!("try from indirect -> {} ({:?})", stringify!($t), case)))
        }
    }
}

impl TryFrom<IndirectValue> for Option<u32> {
    type Error = SchemaError;

    fn try_from(indirect: IndirectValue) -> Result<Self, Self::Error> {
        Ok(match indirect {
            IndirectValue::Null => None,
            _ => Some(indirect.try_into()?)
        })
    }
}

prim_to_indirect_impl!(IndirectValue::Bool, bool);
prim_to_indirect_impl!(IndirectValue::Int32, i32);
prim_to_indirect_impl!(IndirectValue::Uint32, u32);
prim_to_indirect_impl!(IndirectValue::Float64, f64);
prim_to_indirect_impl!(IndirectValue::String, String);
