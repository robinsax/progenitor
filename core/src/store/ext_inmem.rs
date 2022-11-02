use std::sync::{RwLock, PoisonError};

use async_trait::async_trait;

use crate::DirectSerial;
use crate::serial::{SerialValue, ext::PseudoSerial};
use crate::schema::{Value, Expression, Mutation};

use super::errors::PersistenceError;
use super::driver::PersistenceDriver;

impl<T> From<PoisonError<T>> for PersistenceError {
    fn from(_: PoisonError<T>) -> PersistenceError {
        PersistenceError::Poisoned
    }
}

pub struct InMemoryPersistenceDriver {
    data: RwLock<Vec<Value>>
}

impl InMemoryPersistenceDriver {
    pub fn new() -> Self {
        Self{ data: RwLock::new(Vec::new()) }
    }
}

#[async_trait]
impl PersistenceDriver for InMemoryPersistenceDriver {
    async fn load<T: DirectSerial>(&self, filter: Expression, limit: usize, offset: usize) -> Result<Vec<T>, PersistenceError> {
        let data = self.data.read()?;
        
        let mut until_offs = 0;
        let mut found: Vec<T> = Vec::new();
        for item in data.as_slice() {
            if filter.evaluate(item)? {
                if until_offs < offset {
                    until_offs += 1;
                    continue;
                }

                let concrete = T::deserialize::<PseudoSerial>(SerialValue::Pseudo(item.clone()))?;

                found.push(concrete);

                if found.len() == limit {
                    break
                }
            }
        }

        Ok(found)
    }

    async fn update(&self, filter: Expression, update: &Mutation) -> Result<usize, PersistenceError> {
        let mut data = self.data.write()?;

        let mut updates: Vec<(usize, Value)> = Vec::new();
        for (i, item) in data.as_slice().into_iter().enumerate() {
            if filter.evaluate(item)? {
                updates.push((i, update.execute(item)?));
            }
        }

        let update_count = updates.len();
        for (i, updated) in updates {
            data[i] = updated;
        }

        Ok(update_count)
    }

    async fn delete(&self, filter: Expression) -> Result<usize, PersistenceError> {
        let mut data = self.data.write()?;

        let mut removals: Vec<usize> = Vec::new();
        for (i, item) in data.as_slice().into_iter().enumerate() {
            if filter.evaluate(item)? {
                removals.push(i);
            }
        }

        let removal_count = removals.len();
        for i in removals {
            data.remove(i);
        }

        Ok(removal_count)
    }

    async fn insert<T: DirectSerial>(&self, new_data: Vec<T>) -> Result<(), PersistenceError> {
        let mut values: Vec<Value> = Vec::new();
        for item in new_data {
            match item.serialize::<PseudoSerial>()? {
                SerialValue::Pseudo(value) => values.push(value),
                _ => panic!("TODO: This is impossible right?")
            }

        }

        let mut data = self.data.write()?;
        for item in values {
            data.push(item);
        }

        Ok(())
    }
}
