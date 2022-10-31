use super::errors::SchemaError;
use super::indirect_ref::IndirectRef;
use super::indirect_type::IndirectType;
use super::indirect_value::IndirectValue;

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
    // TODO proc_macro or something
    pub fn evaluate(&self, left: &IndirectValue, right: &IndirectValue) -> bool {
        match left {
            IndirectValue::Null{ .. } => {
                match right {
                    IndirectValue::Null{ .. } => true,
                    _ => false
                }
            },
            IndirectValue::Bool(a) => {
                match right {
                    IndirectValue::Bool(b) => a == b,
                    _ => false
                }
            }
            IndirectValue::Uint32(a) => {
                match right {
                    IndirectValue::Uint32(b) => apply_comparator_std_ops!(self, a, b),
                    _ => false
                }
            },
            IndirectValue::Int32(a) => {
                match right {
                    IndirectValue::Int32(b) => apply_comparator_std_ops!(self, a, b),
                    _ => false
                }
            },
            IndirectValue::Float64(a) => {
                match right {
                    IndirectValue::Float64(b) => apply_comparator_std_ops!(self, a, b),
                    _ => false
                }
            },
            IndirectValue::String(a) => {
                match right {
                    IndirectValue::String(b) => apply_comparator_std_ops!(self, a, b),
                    _ => false
                }
            },
            // TODO deep?
            IndirectValue::List(_) => false,
            IndirectValue::Map(_) => false
        }
    }

    pub fn validate(&self, typ: &IndirectType, left: &IndirectRef, right: &IndirectRef) -> Result<(), SchemaError> {
        let left_type = left.lookup_type(typ)?;
        let right_type = right.lookup_type(typ)?;

        if !left_type.primitive_eq(&right_type) {
            Err(SchemaError::TODO("not prim eq".into()))
        }
        else { Ok(()) }
    }
}

#[derive(Debug)]
pub enum IndirectConjunctive {
    And,
    Or
}

#[derive(Debug)]
pub enum IndirectExpression {
    Comparison(IndirectComparator, IndirectRef, IndirectRef),
    Conjunctive(IndirectConjunctive, Vec<IndirectExpression>)
}

impl IndirectExpression {
    pub fn validate(&self, typ: &IndirectType) -> Result<(), SchemaError> {
        match self {
            Self::Comparison(op, a, b) => {
                op.validate(typ, a, b)
            },
            Self::Conjunctive(_, parts) => {
                if parts.len() == 0 {
                    return Err(SchemaError::TODO("empty conjunctive".into()))
                }

                for expr in parts.as_slice() {
                    match expr.validate(typ) {
                        Err(err) => return Err(err),
                        _ => {}
                    };
                }

                Ok(())
            }
        }
    }

    // TODO ctrl + c / ctrl + v above
    pub fn evaluate(&self, value: &IndirectValue) -> Result<bool, SchemaError> {
        match self {
            Self::Comparison(op, a, b) => {
                Ok(op.evaluate(&a.lookup_value(value)?, &b.lookup_value(value)?))
            },
            Self::Conjunctive(op, parts) => {
                let is_and = match op {
                    IndirectConjunctive::Or => false,
                    IndirectConjunctive::And => true
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
