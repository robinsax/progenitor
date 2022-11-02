use async_trait::async_trait;

use crate::schema::{Expression, Mutation};
use crate::serial::StreamSerial;

use super::PersistenceError;

#[async_trait]
pub trait PersistenceDriver: Send + Sync { // TODO replace literalvalue with performant alternative
    async fn load<T: StreamSerial>(&self, filter: Expression, limit: usize, offset: usize) -> Result<Vec<T>, PersistenceError>;
    async fn update(&self, filter: Expression, update: &Mutation) -> Result<usize, PersistenceError>;
    async fn delete(&self, filter: Expression) -> Result<usize, PersistenceError>;
    async fn insert<T: StreamSerial>(&self, data: Vec<T>) -> Result<(), PersistenceError>; // TODO rv for in db ids
}
