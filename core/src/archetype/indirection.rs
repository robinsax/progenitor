use std::mem;

use super::primitives::{LiteralValue, DataType, SchemaError};

#[derive(Debug)]
pub enum IndirectValue {
    Literal{literal: LiteralValue},
    Attribute{lookup: String}
}

impl From<LiteralValue> for IndirectValue {
    fn from(literal: LiteralValue) -> Self {
        IndirectValue::Literal{ literal }
    }
}

impl IndirectValue {
    pub fn data_type_within(&self, data_type: &DataType) -> Result<DataType, SchemaError> {
        match self {
            IndirectValue::Literal{ literal } => Ok(literal.data_type()),
            IndirectValue::Attribute{ lookup } => data_type.lookup(lookup)
        }
    }

    pub fn realize(&self, target: &LiteralValue) -> Result<LiteralValue, SchemaError> {
        match self {
            IndirectValue::Literal{ literal } => Ok(literal.clone()),
            IndirectValue::Attribute{ lookup } => target.lookup(lookup)
        }
    }
}

#[derive(Debug)]
pub enum IndirectComparator { // TODO bitwise + copy + more
    Eq,
    Neq,
    Lt,
    Gt
}

macro_rules! apply_comparator_std_ops {
    ($o: ident, $a: ident, $b: ident) => {
        match $o {
            IndirectComparator::Eq => $a == $b,
            IndirectComparator::Neq => $a != $b,
            IndirectComparator::Lt => $a < $b,
            IndirectComparator::Gt => $a > $b
        }
    };
}

impl IndirectComparator {
    // TODO result
    pub fn realize(&self, left: &LiteralValue, right: &LiteralValue) -> bool {
        match left {
            LiteralValue::Uint32{ value: a } => {
                match right {
                    LiteralValue::Uint32{ value: b } => apply_comparator_std_ops!(self, a, b),
                    _ => false
                }
            },
            LiteralValue::Int32{ value: a } => {
                match right {
                    LiteralValue::Int32{ value: b } => apply_comparator_std_ops!(self, a, b),
                    _ => false
                }
            },
            LiteralValue::Float64{ value: a } => {
                match right {
                    LiteralValue::Float64{ value: b } => apply_comparator_std_ops!(self, a, b),
                    _ => false
                }
            },
            LiteralValue::String{ value: a } => {
                match right {
                    LiteralValue::String{ value: b } => apply_comparator_std_ops!(self, a, b),
                    _ => false
                }
            },
            // TODO deep?
            LiteralValue::List{ .. } => false,
            LiteralValue::Object{ .. } => false
        }
    }

    pub fn validate_within(
        &self, data_type: &DataType, left: &IndirectValue, right: &IndirectValue
    ) -> Result<(), SchemaError> {
        let left_type = left.data_type_within(data_type)?;
        let right_type = right.data_type_within(data_type)?;
        
        if !left_type.primitive_match(&right_type) {
            return Err(SchemaError::TODO);
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum IndirectConjunctive {
    And,
    Or
}

impl PartialEq<IndirectConjunctive> for IndirectConjunctive {
    fn eq(&self, other: &IndirectConjunctive) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}

#[derive(Debug)]
pub enum IndirectExpression {
    Comparison{left: IndirectValue, right: IndirectValue, op: IndirectComparator},
    Conjunctive{op: IndirectConjunctive, inner: Vec<IndirectExpression>}
}

impl IndirectExpression {
    pub fn validate_within(&self, data_type: &DataType) -> Result<(), SchemaError> {
        match self {
            IndirectExpression::Comparison{ op, left, right } => {
                op.validate_within(data_type, left, right)
            },
            IndirectExpression::Conjunctive { op, inner } => {
                let mut any = false;
                for expr in inner.as_slice() {
                    match expr.validate_within(data_type) {
                        Err(err) => {
                            if *op == IndirectConjunctive::And {
                                return Err(err);
                            }
                        },
                        Ok(_) => any = true
                    };
                }
                if *op == IndirectConjunctive::Or && !any {
                    return Err(SchemaError::TODO)
                }

                Ok(())
            }
        }
    }
}

pub enum IndirectMutation {
    Set{lookup: String, to: LiteralValue},
    Many{parts: Vec<IndirectMutation>}
}

impl IndirectMutation {
    pub fn apply(&self, target: &LiteralValue) -> Result<LiteralValue, SchemaError> {
        fn apply_via_lookup(target: &LiteralValue, lookup: &str, to: LiteralValue) -> Result<LiteralValue, SchemaError> {
            let (this_lookup, next_lookup) = match lookup.split_once(".") {
                Some((a, b)) => (a, Some(b)),
                None => (lookup, None)
            };
    
            match target {
                LiteralValue::List{ value, inner_type } => {
                    match this_lookup.parse::<usize>() {
                        Ok(i) => {
                            if i >= value.len() {
                                return Err(SchemaError::TODO);
                            }

                            match next_lookup {
                                Some(next) => { 
                                    let mut updated = value.clone();
                                    updated[i] = apply_via_lookup(target, next, to)?;

                                    Ok(LiteralValue::List{
                                        value: updated,
                                        inner_type: inner_type.clone()
                                    })
                                },
                                None => {
                                    // TODO obj in array
                                    Err(SchemaError::TODO)
                                }
                            }
                        },
                        Err(_) => Err(SchemaError::TODO)
                    }
                },
                LiteralValue::Object{ value, type_schema } => {
                    match value.get(this_lookup) {
                        Some(inner) => {
                            let mut updated = value.clone();

                            updated.insert(
                                this_lookup.to_string(),
                                match next_lookup {
                                    Some(next) => {
                                        apply_via_lookup(inner, next, to)?
                                    },
                                    None => to,
                                }
                            );

                            Ok(LiteralValue::Object{
                                value: updated,
                                type_schema: type_schema.clone()
                            })
                        },
                        None => Err(SchemaError::TODO)
                    }
                },
                _ => Ok(to)
            }
        }
    
        match self {
            IndirectMutation::Set{ lookup, to } => {
                apply_via_lookup(target, lookup, to.clone())
            },
            IndirectMutation::Many{ parts } => {
                let mut updated = target.clone();
                for mutation in parts {
                    updated = mutation.apply(&updated)?;
                }

                Ok(updated)
            }
        }
    }
}
