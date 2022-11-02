use std::marker::PhantomData;

use async_trait::async_trait;

use crate::schema::{Type, Expression};
use crate::serial::StreamSerial;

use super::errors::PersistenceError;
use super::driver::PersistenceDriver;
use super::query::{Query, QueryExecutor};

pub struct PersistentStore<T, D>
where
    D: PersistenceDriver
{
    phantom: PhantomData<T>,
    driver: D,
    pub(crate) schema: Type,
}

#[async_trait]
impl<T, D> QueryExecutor<T> for PersistentStore<T, D>
where 
    T: StreamSerial + Clone + Send + Sync,
    D: PersistenceDriver
{
    async fn load(&self, filter: Expression, offset: usize, limit: usize) -> Result<Vec<T>, PersistenceError> {
        filter.validate(&self.schema)?;

        self.driver.load(filter, limit, offset).await
    }
}

impl<T, D> PersistentStore<T, D>
where 
    T: StreamSerial + Clone + Send + Sync,
    D: PersistenceDriver
{
    pub fn new(schema: Type, driver: D) -> Self {
        Self {
            phantom: PhantomData,
            schema,
            driver
        }
    }

    pub fn query(&self) -> Query<T, Self> {
        Query::new(&self)
    }

    pub async fn put(&self, item: T) -> Result<T, PersistenceError> {
        self.driver.insert(Vec::from([item.clone()])).await?;

        Ok(item)
    }
}
