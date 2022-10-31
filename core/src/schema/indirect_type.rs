use std::{mem, collections::HashMap};

use super::errors::SchemaError;

#[derive(Clone)]
pub enum IndirectType {
    Bool,
    Int32,
    Uint32,
    Float64,
    String,
    List(Box<IndirectType>), // LEARN explicit lifetime + &, vs box, vs [t; 0] for recursive values?
    Map(HashMap<String, IndirectType>)
}

impl IndirectType {
    pub fn lookup(&self, key: &str) -> Result<IndirectType, SchemaError> {
        match self {
            Self::Map(inner) => {
                match inner.get(key.into()) {
                    Some(value_type) => Ok(value_type.clone()),
                    None => Err(SchemaError::TODO(format!("missing key in itype lookup {}", key)))
                }
            },
            _ => Err(SchemaError::TODO(format!("invalid itype lookup {}", key)))
        }
    }

    pub fn primitive_eq(&self, other: &IndirectType) -> bool {
        match self {
            Self::List{ .. } => false,
            Self::Map{ .. } => false,
            rest => mem::discriminant(rest) == mem::discriminant(other)
        }
    }
}
