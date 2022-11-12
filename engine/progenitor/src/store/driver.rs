use async_trait::async_trait;

use crate::schema::{Value, Expression, Mutation};

use super::errors::StoreError;

#[async_trait]
pub trait StoreDriver
where
    Self: Sync + Send
{
    async fn load(&self, filter: Option<Expression>, offset: usize, limit: Option<usize>) -> Result<Vec<Value>, StoreError>;
    async fn update(&self, filter: Expression, update: &Mutation) -> Result<usize, StoreError>;
    async fn delete(&self, filter: Expression) -> Result<usize, StoreError>;
    // TODO: Will need a return value.
    async fn insert(&self, data: Vec<Value>) -> Result<(), StoreError>;
}
