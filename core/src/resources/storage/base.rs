use std::marker::PhantomData;

use mongodb::{bson}; // TODO no
use async_trait::async_trait;
use serde::de::DeserializeOwned;

use crate::archetype::Model;

#[derive(Debug)]
pub enum StoreOperationError {
    Connection,
    InvalidQuery,
    TODO
}

pub struct QueryDescriptor {

}

pub struct Query<'b, T> {
    phantom: PhantomData<T>,
    store: &'b Store<'b, T>,
    descriptor: QueryDescriptor,
}

impl<'b, T> Query<'b, T> where T: DeserializeOwned {
    pub fn new(store: &'b Store<'b, T>) -> Self {
        Self{
            phantom: PhantomData,
            store,
            descriptor: QueryDescriptor {  }
        }
    }

    pub async fn one(&self) -> Result<T, StoreOperationError> {
        let result_raw = self.store.backend.load_one(self.store.archetype, &self.descriptor).await?;

        let result: T = match bson::from_bson(result_raw) {
            Ok(r) => r,
            Err(_) => return Err(StoreOperationError::TODO)
        };

        Ok(result)
    }
}

#[async_trait]
pub trait StorageDriver {
    async fn open<'a>(&self, conn_str: String, archetype: &'a Model) -> Result<Box<dyn StoreBackend + 'a>, StoreOperationError>;
}

#[async_trait]
pub trait StoreBackend {
    async fn load_one(&self, archetype: &Model, query: &QueryDescriptor) -> Result<bson::Bson, StoreOperationError>;
}

pub struct Store<'a, T> {
    phantom: PhantomData<T>,
    archetype: &'a Model,
    backend: Box<dyn StoreBackend + 'a>,
}

impl<'a, T> Store<'a, T>
    where T: DeserializeOwned {
    pub fn new(archetype: &'a Model, backend: Box<dyn StoreBackend + 'a>) -> Self {
        Self{
            phantom: PhantomData,
            archetype,
            backend
        }
    }

    pub fn query(&self) -> Query<T> {
        Query::new(&self)
    }
}
