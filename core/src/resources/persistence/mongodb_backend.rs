use async_trait::async_trait;
use mongodb::{error::Error as MongoError, options::ClientOptions, Client};

use crate::archetype::{IndirectExpression, LiteralValue, IndirectMutation};

use super::common::StorageOpError;
use super::driver::{StorageDriver, StoreBackend};

impl From<MongoError> for StorageOpError {
    fn from(_: MongoError) -> Self {
        Self::TODO
    }
}

pub struct MongoDBStorageDriver;

impl MongoDBStorageDriver {
    pub fn new() -> Self {
        Self{}
    }
}

#[async_trait]
impl StorageDriver for MongoDBStorageDriver {
    async fn open(&self, connection_options: String) -> Result<Box<dyn StoreBackend>, StorageOpError> {
        let options = ClientOptions::parse(connection_options).await?;

        let client = Client::with_options(options)?;

        Ok(Box::new(MongoDBStoreBackend{ client }))
    }
}

pub struct MongoDBStoreBackend {
    client: Client,
}

#[async_trait]
impl StoreBackend for MongoDBStoreBackend {
    async fn load(&self, filter: &IndirectExpression, limit: usize, offset: usize) -> Result<Vec<LiteralValue>, StorageOpError> {
        Err(StorageOpError::TODO)
    }

    async fn update(&self, filters: &IndirectExpression, update: &IndirectMutation) -> Result<usize, StorageOpError> {
        Err(StorageOpError::TODO)
    }

    async fn delete(&self, filters: &IndirectExpression) -> Result<usize, StorageOpError> {
        Err(StorageOpError::TODO)
    }

    async fn insert(&self, data: &[LiteralValue]) -> Result<(), StorageOpError> {
        Err(StorageOpError::TODO)
    }
}
