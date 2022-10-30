use async_trait::async_trait;

use crate::archetype::{IndirectExpression, IndirectMutation, LiteralValue};

use super::StorageOpError;

#[async_trait]
pub trait StoreBackend: Send + Sync { // TODO replace literalvalue with performant alternative
    async fn load(&self, filter: &IndirectExpression, limit: usize, offset: usize) -> Result<Vec<LiteralValue>, StorageOpError>;
    async fn update(&self, filter: &IndirectExpression, update: &IndirectMutation) -> Result<usize, StorageOpError>;
    async fn delete(&self, filter: &IndirectExpression) -> Result<usize, StorageOpError>;
    async fn insert(&self, data: &[LiteralValue]) -> Result<(), StorageOpError>; // TODO rv for in db ids
}

#[async_trait]
pub trait StorageDriver {
    async fn open(&self, connection_options: String) -> Result<Box<dyn StoreBackend>, StorageOpError>;
}
