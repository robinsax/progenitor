use async_trait::async_trait;

use crate::env_config::FromEnvConfig;
use crate::schema::{Expression, Mutation};
use crate::serial::StreamSerial;

use super::StoreError;

// TODO: Async obviously needed, just too lazy to figure out how to nicely deal with Sync for implementers it right now.
#[async_trait]
pub trait StoreDriver
where
    Self: Send + Sync + FromEnvConfig
{
    fn load<T: StreamSerial>(&self, filter: Option<Expression>, offset: usize, limit: Option<usize>) -> Result<Vec<T>, StoreError>;
    fn update(&self, filter: Expression, update: &Mutation) -> Result<usize, StoreError>;
    fn delete(&self, filter: Expression) -> Result<usize, StoreError>;
    // TODO: Will need a return value soon.
    fn insert<T: StreamSerial>(&self, data: Vec<T>) -> Result<(), StoreError>;
}
