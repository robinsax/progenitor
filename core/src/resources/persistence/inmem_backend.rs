use std::sync::{RwLock, PoisonError};

use async_trait::async_trait;

use crate::archetype::{LiteralValue, IndirectExpression, IndirectMutation};

use super::common::StorageOpError;
use super::driver::{StorageDriver, StoreBackend};

pub struct InMemoryStorageDriver;

impl InMemoryStorageDriver {
    pub fn new() -> Self {
        Self{}
    }
}

impl<T> From<PoisonError<T>> for StorageOpError {
    fn from(_: PoisonError<T>) -> StorageOpError {
        StorageOpError::TODO
    }
}

#[async_trait]
impl StorageDriver for InMemoryStorageDriver {
    async fn open(&self, conn_str: String) -> Result<Box<dyn StoreBackend>, StorageOpError> {
        Ok(Box::new(InMemoryStoreBackend::new()))
    }
}

pub struct InMemoryStoreBackend {
    data: RwLock<Vec<LiteralValue>>
}

impl InMemoryStoreBackend {
    pub fn new() -> Self {
        Self{ data: RwLock::new(Vec::new()) }
    }
}

impl InMemoryStoreBackend {    
    // TODO a lot more complexity than this in reality (foreign refs)
    fn evaluate_filter(&self, item: &LiteralValue, filter: &IndirectExpression) -> Result<bool, StorageOpError> {
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
impl StoreBackend for InMemoryStoreBackend {
    async fn load(&self, filter: &IndirectExpression, limit: usize, offset: usize) -> Result<Vec<LiteralValue>, StorageOpError> {
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

    async fn update(&self, filter: &IndirectExpression, update: &IndirectMutation) -> Result<usize, StorageOpError> {
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

    async fn delete(&self, filter: &IndirectExpression) -> Result<usize, StorageOpError> {
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

    async fn insert(&self, new_data: &[LiteralValue]) -> Result<(), StorageOpError> {
        let mut data = self.data.write()?;

        for item in new_data {
            data.push(item.clone()); // TODO consume inp
        }

        Ok(())
    }
}
