use async_trait::async_trait;

use crate::schema::{IndirectExpression, IndirectMutation, SerialRepr};

use super::PersistenceError;

#[async_trait]
pub trait PersistenceDriver: Send + Sync { // TODO replace literalvalue with performant alternative
    async fn load<T: SerialRepr>(&self, filter: IndirectExpression, limit: usize, offset: usize) -> Result<Vec<T>, PersistenceError>;
    async fn update(&self, filter: IndirectExpression, update: &IndirectMutation) -> Result<usize, PersistenceError>;
    async fn delete(&self, filter: IndirectExpression) -> Result<usize, PersistenceError>;
    async fn insert<T: SerialRepr>(&self, data: Vec<T>) -> Result<(), PersistenceError>; // TODO rv for in db ids
}
