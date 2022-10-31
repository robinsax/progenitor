use std::marker::PhantomData;

use async_trait::async_trait;

use crate::schema::{IndirectType, IndirectExpression, SerialRepr};

use super::{common::PersistenceError, driver::PersistenceDriver, query::{Query, QueryExecutor}};

pub struct PersistentStore<T, D> where D: PersistenceDriver {
    phantom: PhantomData<T>,
    driver: D,
    pub(crate) schema: IndirectType,
}

#[async_trait]
impl<T: SerialRepr + Clone + Send + Sync, D: PersistenceDriver> QueryExecutor<T> for PersistentStore<T, D> {
    async fn load(&self, filter: IndirectExpression, offset: usize, limit: usize) -> Result<Vec<T>, PersistenceError> {
        filter.validate(&self.schema)?;

        self.driver.load(filter, limit, offset).await
    }
}

impl<T: SerialRepr + Clone + Send + Sync, D: PersistenceDriver> PersistentStore<T, D> {
    pub fn new(schema: IndirectType, driver: D) -> Self {
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
