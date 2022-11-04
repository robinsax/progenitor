// Indirect representation of mutations to indirectly represented data.
use super::errors::SchemaError;
use super::primitives::{Type, Value};

// TODO: Needs more variants.
pub enum Mutation {
    Set(String, Value),
    Bulk(Vec<Mutation>)
}

// TODO: Avoid clone!
fn execute_mutation_via_lookup(target: &Value, lookup: &str, to: Value) -> Result<Value, SchemaError> {
    // TODO: Rusty way to do this is probably with an iterator.
    let (this_lookup, next_lookup) = match lookup.split_once(".") {
        Some((a, b)) => (a, Some(b)),
        None => (lookup, None)
    };

    match target {
        Value::List(elements) => {
            match this_lookup.parse::<usize>() {
                Ok(i) => {
                    if i >= elements.len() {
                        return Err(SchemaError::InvalidIndex(Type::try_from(target).ok(), Some(i)));
                    }

                    match next_lookup {
                        Some(next) => { 
                            let mut updated = elements.clone();
                            updated[i] = execute_mutation_via_lookup(target, next, to)?;

                            Ok(Value::List(updated))
                        },
                        None => {
                            Err(SchemaError::NotImplemented("Object key mutation of array element".into()))
                        }
                    }
                },
                Err(_) => Err(SchemaError::InvalidIndex(Type::try_from(target).ok(), None))
            }
        },
        Value::Map(members) => {
            match members.get(this_lookup) {
                Some(inner) => {
                    let mut updated = members.clone();

                    updated.insert(
                        this_lookup.to_string(),
                        match next_lookup {
                            Some(next) => {
                                execute_mutation_via_lookup(inner, next, to)?
                            },
                            None => to,
                        }
                    );

                    Ok(Value::Map(updated))
                },
                None => {
                    Err(SchemaError::InvalidLookup(Type::try_from(target).ok(), this_lookup.into()))
                }
            }
        },
        _ => Ok(to)
    }
}

impl Mutation {
    pub fn execute(&self, target: &Value) -> Result<Value, SchemaError> {
        match self {
            Mutation::Set(lookup, value) => {
                execute_mutation_via_lookup(target, lookup, value.clone())
            },
            Mutation::Bulk(parts) => {
                let mut updated = target.clone();

                for mutation in parts {
                    updated = mutation.execute(&updated)?;
                }

                Ok(updated)
            }
        }
    }
}

// TODO: High-prio for tests.
