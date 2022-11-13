// Indirect expressions applied to indirectly represented types or data.
use std::cmp::{Eq, PartialOrd};

use super::errors::SchemaError;
use super::primitives::{Type, Value};

// TODO: Should be bitwise, Copy, and have more members.
#[derive(Debug, Clone)]
pub enum Comparator {
    Eq,
    Neq,
    Lt,
    Gt
}

// TODO: Currently evaluates as false when comparisons are actually impossible...
impl Comparator {
    pub fn parse_from_value(value: Value) -> Result<Self, SchemaError> {
        let which: String = value.try_into()?;

        match which.as_str() {
            "eq" => Ok(Self::Eq),
            "neq" => Ok(Self::Neq),
            "lt" => Ok(Self::Lt),
            "gt" => Ok(Self::Gt),
            _ => Err(SchemaError::NotImplemented("invalid comparator".into()))
        }
    }

    fn compare_eq<T>(&self, a: T, b: T) -> bool
    where
        T: Eq
    {
        match self {
            Self::Eq => a == b,
            Self::Neq => a != b,
            _ => false // ...here
        }
    }

    fn compare_ord<T>(&self, a: T, b: T) -> bool
    where
        T: PartialOrd
    {
        match self {
            Self::Lt => a < b,
            Self::Gt => a > b,
            _ => false // ...here
        }
    }

    fn compare_full<T>(&self, a: T, b: T) -> bool
    where
        T: Eq + PartialOrd
    {
        match self {
            Self::Eq => a == b,
            Self::Neq => a != b,
            Self::Lt => a < b,
            Self::Gt => a > b
        }
    }

    pub fn validate(&self, typ: &Type, left: &ValueReference, right: &ValueReference) -> Result<(), SchemaError> {
        let a = left.lookup_type(typ)?;
        let b = right.lookup_type(typ)?;

        if !a.primitive_eq(&b) {
            Err(SchemaError::InvalidComparison(self.clone(), a.into(), b.into()))
        }
        else {
            Ok(())
        }
    }

    // TODO: Branch casing here is brutal, macroize.
    pub fn evaluate(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Null, Value::Null) => true,
            (Value::Bool(a), Value::Bool(b)) => self.compare_eq(a, b),
            (Value::Float64(a), Value::Float64(b)) => self.compare_ord(a, b),
            (Value::Str(a), Value::Str(b)) => self.compare_full(a, b),
            (Value::Int32(a), Value::Int32(b)) => self.compare_full(a, b),
            (Value::Uint32(a), Value::Uint32(b)) => self.compare_full(a, b),
            _ => false
        }
    }
}

#[derive(Debug)]
pub enum Conjunctive {
    And,
    Or
}

impl Conjunctive {
    pub fn parse_from_value(value: Value) -> Result<Self, SchemaError> {
        let which: String = value.try_into()?;

        match which.as_str() {
            "and" => Ok(Self::And),
            "or" => Ok(Self::Or),
            _ => Err(SchemaError::NotImplemented("invalid conjunctive".into()))
        }
    }
}

// An evaluatable indirect expression.
#[derive(Debug)]
pub enum Expression {
    Comparison(Comparator, ValueReference, ValueReference),
    Conjunctive(Conjunctive, Vec<Expression>)
}

impl Expression {
    // TODO: Bad schema.
    pub fn parse_from_value(value: Value) -> Result<Self, SchemaError> {
        if let Ok(cmp_value) = value.lookup("compare") {
            Ok(Self::Comparison(
                Comparator::parse_from_value(cmp_value.index(0)?)?,
                ValueReference::parse_from_value(cmp_value.index(1)?)?,
                ValueReference::parse_from_value(cmp_value.index(2)?)?
            ))
        }
        else if let Ok(conj_value) = value.lookup("many") {
            let elements_value = conj_value.index(1)?;
            let elements = elements_value.elements()?;

            let mut exprs = Vec::with_capacity(elements.len());
            for element in elements {
                exprs.push(Expression::parse_from_value(element.clone())?);
            }

            Ok(Self::Conjunctive(
                Conjunctive::parse_from_value(conj_value.index(0)?)?,
                exprs
            ))
        }
        else {
            Err(SchemaError::NotImplemented("invalid expression".into()))
        }
    }

    pub fn validate(&self, typ: &Type) -> Result<(), SchemaError> {
        match self {
            Self::Comparison(op, a, b) => {
                op.validate(typ, a, b)
            },
            Self::Conjunctive(_, parts) => {
                if parts.len() == 0 {
                    return Err(SchemaError::NotImplemented("Empty conjunctive".into()));
                }

                for expr in parts.as_slice() {
                    expr.validate(typ)?;
                }

                Ok(())
            }
        }
    }

    pub fn evaluate(&self, value: &Value) -> Result<bool, SchemaError> {
        match self {
            Self::Comparison(op, a, b) => {
                Ok(op.evaluate(&a.lookup_value(value)?, &b.lookup_value(value)?))
            },
            Self::Conjunctive(op, parts) => {
                // TODO: PartialEq?
                let is_and = match op {
                    Conjunctive::Or => false,
                    Conjunctive::And => true
                };

                let mut any = false;
                for expr in parts.as_slice() {
                    match expr.evaluate(value) {
                        Err(err) => return Err(err),
                        Ok(subresult) => {
                            if !subresult && is_and {
                                return Ok(false);
                            }

                            any = any || subresult
                        }
                    };
                }

                Ok(any)
            }
        }
    }
}

// An indirect reference to a value or a value.
// TODO: Better reference encapsulation.
#[derive(Debug)]
pub enum ValueReference {
    Value(Value),
    Reference(String)
}

impl From<Value> for ValueReference {
    fn from(value: Value) -> Self {
        ValueReference::Value(value)
    }
}

impl ValueReference {
    pub fn parse_from_value(value: Value) -> Result<Self, SchemaError> {
        if let Ok(literal) = value.lookup("value") {
            Ok(Self::Value(literal))
        }
        else if let Ok(ref_str) = value.lookup("ref") {
            Ok(Self::Reference(ref_str.try_into()?))
        }
        else {
            Err(SchemaError::NotImplemented("invalid value reference".into()))
        }
    }

    pub fn lookup_type(&self, typ: &Type) -> Result<Type, SchemaError> {
        match self {
            Self::Value(value) => value.try_into(),
            Self::Reference(lookup) => typ.lookup(lookup)
        }
    }

    pub fn lookup_value(&self, value: &Value) -> Result<Value, SchemaError> {
        match self {
            Self::Value(value) => Ok(value.clone()),
            Self::Reference(lookup) => value.lookup(lookup)
        }
    }
}
