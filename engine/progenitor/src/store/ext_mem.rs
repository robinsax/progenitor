use std::collections::LinkedList;
use std::sync::{Mutex, MutexGuard};
use std::any::Any;

use async_trait::async_trait;

use crate::errors::InitError;
use crate::env_config::{EnvConfig, FromEnvConfig};
use crate::serial::{DirectSerial, SerialValue, ext::PseudoSerial};
use crate::schema::{Value, Expression, Mutation};

use super::errors::StoreError;
use super::driver::StoreDriver;

static MEM_POOL: Mutex<LinkedList<(String, &'static Mutex<dyn Any + Send>)>> = Mutex::new(LinkedList::new());

// TODO: Clone here is temporary.
#[derive(Clone)]
pub struct MemStore {
    name: String
}

impl MemStore {
    // TODO: If this is actually going to be a production construct use UnsafeCell for the static.
    fn internal_mem<T: Send + 'static>(&self) -> Result<MutexGuard<'_, Vec<T>>, StoreError>
    where
        T: Send 
    {
        let mut mem_pool = MEM_POOL.lock()?;

        let uncast_mem = match mem_pool.iter().find(|(s, _)| s == &self.name) {
            Some(ext_mem) => ext_mem.1,
            None => {
                let new_mem_alloc: Box<Mutex<dyn Any + Send>> = Box::new(Mutex::new(Vec::<T>::new()));

                let new_mem = Box::leak(new_mem_alloc);

                mem_pool.push_back((self.name.clone(), new_mem));

                mem_pool.iter().find(|(s, _)| s == &self.name)
                    .ok_or_else(|| StoreError::Backend)?
                    .1
            }
        };

        let mem = unsafe { &*(uncast_mem as *const Mutex<dyn Any + Send> as *const Mutex<Vec<T>>) };

        Ok(mem.lock()?)
    }
}

impl FromEnvConfig for MemStore {
    fn try_from_config(env: EnvConfig) -> Result<Self, InitError> {
        Ok(Self { name: env.get_var("TODO")? })
    }
}

#[async_trait]
impl StoreDriver for MemStore {
    fn load<T>(&self, filter: Option<Expression>, offset: usize, limit: Option<usize>) -> Result<Vec<T>, StoreError>
    where
        T: DirectSerial
    {
        let data = self.internal_mem()?;
        
        let mut until_offs = 0;
        let mut found: Vec<T> = Vec::new();
        for item in data.as_slice() {
            if let Some(filter_expr) = &filter {
                if !filter_expr.evaluate(item)? {
                    continue;
                }
            }

            if until_offs < offset {
                until_offs += 1;
                continue;
            }

            let concrete = T::deserialize::<PseudoSerial>(SerialValue::Pseudo(item.clone()))?;

            found.push(concrete);

            if let Some(limit_value) = limit {
                if found.len() == limit_value {
                    break
                }
            }
        }

        Ok(found)
    }

    fn update(&self, filter: Expression, update: &Mutation) -> Result<usize, StoreError> {
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

    fn delete(&self, filter: Expression) -> Result<usize, StoreError> {
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

    fn insert<T>(&self, new_data: Vec<T>) -> Result<(), StoreError>
    where
        T: DirectSerial
    {
        let mut values: Vec<Value> = Vec::new();
        for item in new_data {
            match item.serialize::<PseudoSerial>()? {
                SerialValue::Pseudo(value) => values.push(value),
                _ => panic!("TODO: This is impossible right?")
            }

        }

        let mut data = self.internal_mem()?;
        for item in values {
            data.push(item);
        }

        Ok(())
    }
}
