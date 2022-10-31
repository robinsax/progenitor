use super::indirect_value::IndirectValue;
use super::errors::SchemaError;

pub enum IndirectMutation { // TODO more
    Set(String, IndirectValue),
    Bulk(Vec<IndirectMutation>)
}

impl IndirectMutation {
    pub fn apply(&self, target: &IndirectValue) -> Result<IndirectValue, SchemaError> {
        fn apply_via_lookup(target: &IndirectValue, lookup: &str, to: IndirectValue) -> Result<IndirectValue, SchemaError> {
            let (this_lookup, next_lookup) = match lookup.split_once(".") {
                Some((a, b)) => (a, Some(b)),
                None => (lookup, None)
            };
    
            match target {
                IndirectValue::List(elements) => {
                    match this_lookup.parse::<usize>() {
                        Ok(i) => {
                            if i >= elements.len() {
                                return Err(SchemaError::TODO("index lookup oob".into()));
                            }

                            match next_lookup {
                                Some(next) => { 
                                    let mut updated = elements.clone();
                                    updated[i] = apply_via_lookup(target, next, to)?;

                                    Ok(IndirectValue::List(updated))
                                },
                                None => {
                                    Err(SchemaError::TODO("object key mut apply below ary idx".into()))
                                }
                            }
                        },
                        Err(_) => Err(SchemaError::TODO("invalid lookup idx fmt".into()))
                    }
                },
                IndirectValue::Map(members) => {
                    match members.get(this_lookup) {
                        Some(inner) => {
                            let mut updated = members.clone();

                            updated.insert(
                                this_lookup.to_string(),
                                match next_lookup {
                                    Some(next) => {
                                        apply_via_lookup(inner, next, to)?
                                    },
                                    None => to,
                                }
                            );

                            Ok(IndirectValue::Map(updated))
                        },
                        None => Err(SchemaError::TODO("invalid map member key".into()))
                    }
                },
                _ => Ok(to)
            }
        }
    
        match self {
            IndirectMutation::Set(lookup, value) => {
                apply_via_lookup(target, lookup, value.clone())
            },
            IndirectMutation::Bulk(parts) => {
                let mut updated = target.clone();
                for mutation in parts {
                    updated = mutation.apply(&updated)?;
                }

                Ok(updated)
            }
        }
    }
}
