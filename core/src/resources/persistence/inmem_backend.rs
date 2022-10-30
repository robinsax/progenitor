use std::sync::{RwLock, PoisonError};

use async_trait::async_trait;

use crate::archetype::{LiteralValue, IndirectExpression, IndirectMutation};

use super::common::PersistenceError;
use super::driver::{PersistenceDriver, PersistentStoreBackend};

pub struct InMemoryPersistenceDriver;

impl InMemoryPersistenceDriver {
    pub fn new() -> Self {
        Self{}
    }
}

impl<T> From<PoisonError<T>> for PersistenceError {
    fn from(_: PoisonError<T>) -> PersistenceError {
        PersistenceError::TODO
    }
}

#[async_trait]
impl PersistenceDriver for InMemoryPersistenceDriver {
    async fn open(&self, conn_str: String) -> Result<Box<dyn PersistentStoreBackend>, PersistenceError> {
        Ok(Box::new(InMemoryPersistentStoreBackend::new()))
    }
}

pub struct InMemoryPersistentStoreBackend {
    data: RwLock<Vec<LiteralValue>>
}

impl InMemoryPersistentStoreBackend {
    pub fn new() -> Self {
        Self{ data: RwLock::new(Vec::new()) }
    }
}

impl InMemoryPersistentStoreBackend {    
    // TODO a lot more complexity than this in reality (foreign refs)
    fn evaluate_filter(&self, item: &LiteralValue, filter: &IndirectExpression) -> Result<bool, PersistenceError> {
        match filter {
            IndirectExpression::Comparison{ op, left, right } => {
                Ok(op.realize(&left.realize(item)?, &right.realize(item)?))
            },
            IndirectExpression::Conjunctive{ op, inner } => {
                for cond in inner {
                    if !self.evaluate_filter(item, cond)? {
                        return Ok(false);
                    }
                }

                Ok(true)
            }
        }
    }
}

#[async_trait]
impl PersistentStoreBackend for InMemoryPersistentStoreBackend {
    async fn load(&self, filter: &IndirectExpression, limit: usize, offset: usize) -> Result<Vec<LiteralValue>, PersistenceError> {
        let data = self.data.read()?;
        
        let mut found: Vec<LiteralValue> = Vec::new();
        for item in data.as_slice() {
            if self.evaluate_filter(item, filter)? {
                found.push(item.clone());
                if found.len() == limit {
                    break
                }
            }
        }

        Ok(found)
    }

    async fn update(&self, filter: &IndirectExpression, update: &IndirectMutation) -> Result<usize, PersistenceError> {
        let mut data = self.data.write()?;

        let mut updates: Vec<(usize, LiteralValue)> = Vec::new();
        for (i, item) in data.as_slice().into_iter().enumerate() {
            if self.evaluate_filter(&item, filter)? {
                updates.push((i, update.apply(item)?));
            }
        }

        let update_count = updates.len();
        for (i, updated) in updates {
            data[i] = updated;
        }

        Ok(update_count)
    }

    async fn delete(&self, filter: &IndirectExpression) -> Result<usize, PersistenceError> {
        let mut data = self.data.write()?;

        let mut removals: Vec<usize> = Vec::new();
        for (i, item) in data.as_slice().into_iter().enumerate() {
            if self.evaluate_filter(&item, filter)? {
                removals.push(i);
            }
        }

        let removal_count = removals.len();
        for i in removals {
            data.remove(i);
        }

        Ok(removal_count)
    }

    async fn insert(&self, new_data: &[LiteralValue]) -> Result<(), PersistenceError> {
        let mut data = self.data.write()?;

        for item in new_data {
            data.push(item.clone()); // TODO consume inp
        }

        Ok(())
    }
}
