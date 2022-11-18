use async_trait::async_trait;

use crate::schema::{Value, Condition, Mutation};

use super::errors::StoreError;

#[async_trait]
pub trait StoreDriver
where
    Self: Sync + Send
{
    async fn load(&self, filter: Option<Condition>, offset: usize, limit: Option<usize>) -> Result<Vec<Value>, StoreError>;
    async fn update(&self, filter: Condition, update: &Mutation) -> Result<usize, StoreError>;
    async fn delete(&self, filter: Condition) -> Result<usize, StoreError>;
    // TODO: Will need a return value.
    async fn insert(&self, data: Vec<Value>) -> Result<(), StoreError>;
}
