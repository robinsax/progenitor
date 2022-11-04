use super::primitives::Type;
use super::expr::Comparator;

#[derive(Debug, Clone)]
pub enum SchemaError {
    UnknownableType,
    InvalidComparison(Comparator, Type, Type),
    InvalidLookup(Option<Type>, String),
    InvalidIndex(Option<Type>, Option<usize>),
    NotImplemented(String)
}
