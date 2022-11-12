use std::error::Error;
use std::fmt::{Display, Formatter};

use super::primitives::Type;
use super::expr::Comparator;

// TODO: Clean this up.
#[derive(Debug, Clone)]
pub enum SchemaError {
    UnknownableType,
    InvalidComparison(Comparator, Type, Type),
    InvalidLookup(Option<Type>, String),
    InvalidIndex(Option<Type>, Option<usize>),
    InvalidCast(Type),
    MissingKey(String),
    InvalidType(Type, Type),
    NotImplemented(String)
}

impl Display for SchemaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "schema error: {:?}", self)
    }
}

impl Error for SchemaError {}
