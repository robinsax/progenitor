use async_trait::async_trait;

use crate::archetype::{IndirectExpression, IndirectMutation, LiteralValue};

use super::PersistenceError;

#[async_trait]
pub trait PersistenceDriver: Send + Sync { // TODO replace literalvalue with performant alternative
    async fn load(&self, filter: &IndirectExpression, limit: usize, offset: usize) -> Result<Vec<LiteralValue>, PersistenceError>;
    async fn update(&self, filter: &IndirectExpression, update: &IndirectMutation) -> Result<usize, PersistenceError>;
    async fn delete(&self, filter: &IndirectExpression) -> Result<usize, PersistenceError>;
    async fn insert(&self, data: &[LiteralValue]) -> Result<(), PersistenceError>; // TODO rv for in db ids
}
