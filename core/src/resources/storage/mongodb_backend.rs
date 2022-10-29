use async_trait::async_trait;
use mongodb::{bson, error::Error, options::ClientOptions, Client};

use crate::archetype::Model;

use super::base::{StoreOperationError, StoreBackend, QueryDescriptor, StorageDriver};

pub struct MongoDBStoreBackend<'a> {
    archetype: &'a Model,
    client: Client,
}

impl From<Error> for StoreOperationError {
    fn from(_: Error) -> Self {
        Self::Connection
    }
}

pub struct MongoDBDriver;

impl MongoDBDriver {
    pub fn new() -> Self {
        Self{}
    }
}

#[async_trait]
impl StorageDriver for MongoDBDriver {
    async fn open<'a>(&self, conn_str: String, archetype: &'a Model) -> Result<Box<dyn StoreBackend + 'a>, StoreOperationError> {
        let options = ClientOptions::parse(conn_str).await?;

        let client = Client::with_options(options)?;

        Ok(Box::new(MongoDBStoreBackend{ client, archetype }))
    }
}

#[async_trait]
impl<'a> StoreBackend for MongoDBStoreBackend<'a> {
    async fn load_one(&self, archetype: &Model, query: &QueryDescriptor) -> Result<bson::Bson, StoreOperationError> {
        let collection = self.client.database("TODO").collection(self.archetype.name.as_str());

        match collection.find_one(None, None).await {
            Ok(Some(r)) => Ok(r),
            _ => Err(StoreOperationError::TODO),
        }
    }
}
