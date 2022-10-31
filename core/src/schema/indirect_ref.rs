use super::{indirect_value::IndirectValue, indirect_type::IndirectType, errors::SchemaError};

#[derive(Debug)]
pub enum IndirectRef {
    Value(IndirectValue),
    AttrRef(String)
}

impl From<IndirectValue> for IndirectRef {
    fn from(value: IndirectValue) -> Self {
        IndirectRef::Value(value)
    }
}

impl IndirectRef {
    pub fn lookup_type(&self, typ: &IndirectType) -> Result<IndirectType, SchemaError> {
        match self {
            Self::Value(value) => value.try_into(),
            Self::AttrRef(lookup) => typ.lookup(lookup)
        }
    }

    pub fn lookup_value(&self, value: &IndirectValue) -> Result<IndirectValue, SchemaError> {
        match self {
            Self::Value(value) => Ok(value.clone()),
            Self::AttrRef(lookup) => value.lookup(lookup)
        }
    }
}
