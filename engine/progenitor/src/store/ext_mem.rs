use std::collections::LinkedList;
use std::sync::{Mutex, MutexGuard};

use async_trait::async_trait;

use crate::schema::{Value, Expression, Mutation};

use super::errors::StoreError;
use super::driver::StoreDriver;

type MemPoolEntry = (String, &'static Mutex<Vec<Value>>);

static MEM_POOL: Mutex<LinkedList<MemPoolEntry>> = Mutex::new(LinkedList::new());

// TODO: Clone here is temporary.
#[derive(Clone)]
pub struct MemStore {
    name: String
}

impl MemStore {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into()
        }
    }

    // TODO: If this is actually going to be a production construct use UnsafeCell for the static.
    fn internal_mem(&self) -> Result<MutexGuard<'_, Vec<Value>>, StoreError> {
        let mut mem_pool = match MEM_POOL.lock() {
            Ok(guard) => guard,
            Err(_) => return Err(StoreError::Backend("Memory pool lock poisoned".into()))
        };

        let mem_entry = match mem_pool.iter().find(|(s, _)| s == &self.name) {
            Some(ext_mem) => ext_mem.1,
            None => {
                let new_mem_alloc: Box<Mutex<Vec<Value>>> = Box::new(Mutex::new(Vec::new()));

                let new_mem = Box::leak(new_mem_alloc);

                mem_pool.push_back((self.name.clone(), new_mem));

                mem_pool.iter().find(|(s, _)| s == &self.name)
                    .ok_or_else(|| StoreError::Backend("Memory pool missing".into()))?
                    .1
            }
        };

        match mem_entry.lock() {
            Ok(guard) => Ok(guard),
            Err(_) => return Err(StoreError::Backend("Memory entry lock poisoned".into()))
        }
    }
}

#[async_trait]
impl StoreDriver for MemStore {
    async fn load(
        &self, filter: Option<Expression>, offset: usize, limit: Option<usize>
    ) -> Result<Vec<Value>, StoreError> {
        let data = self.internal_mem()?;
        
        let mut until_offs = 0;
        let mut found: Vec<Value> = Vec::new();
        for item in data.iter() {
            if let Some(filter_expr) = &filter {
                if !filter_expr.evaluate(item)? {
                    continue;
                }
            }

            if until_offs < offset {
                until_offs += 1;
                continue;
            }

            found.push(item.clone());

            if let Some(limit_value) = limit {
                if found.len() == limit_value {
                    break
                }
            }
        }

        Ok(found)
    }

    async fn update(&self, filter: Expression, update: &Mutation) -> Result<usize, StoreError> {
        let mut data = self.internal_mem()?;

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

    async fn delete(&self, filter: Expression) -> Result<usize, StoreError> {
        let mut data = self.internal_mem()?;

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

    async fn insert(&self, new_data: Vec<Value>) -> Result<(), StoreError> {
        let mut data = self.internal_mem()?;
        for item in new_data {
            data.push(item);
        }

        Ok(())
    }
}
