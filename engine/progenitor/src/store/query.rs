use async_trait::async_trait;

use crate::schema::{Value, Condition};

use super::errors::StoreError;

#[async_trait]
pub trait QueryExecutor
where
    Self: Send + Sync
{
    async fn load(&self, filter: Option<Condition>, offset: usize, limit: Option<usize>) -> Result<Vec<Value>, StoreError>;
}

pub struct Query<'qy, E>
where
    E: QueryExecutor
{
    executor: &'qy E,
    filter: Option<Condition>,
    limit: Option<usize>,
    offset: usize
}

impl<'qy, E> Query<'qy, E>
where
    E: QueryExecutor
{
    pub fn new(executor: &'qy E) -> Self {
        Self{
            executor,
            filter: None,
            limit: None,
            offset: 0
        }
    }

    pub fn filter(mut self, condition: Condition) -> Query<'qy, E> {
        self.filter = Some(condition);

        self
    }

    pub fn limit(mut self, limit: usize) -> Query<'qy, E> {
        self.limit = Some(limit);

        self
    }

    pub async fn all(self) -> Result<Vec<Value>, StoreError> {
        self.executor.load(self.filter, self.offset, self.limit).await
    }

    pub async fn one(self) -> Result<Option<Value>, StoreError> {
        let loaded = self.limit(1).all().await?;

        match loaded.len() > 0 {
            true => Ok(Some(loaded[0].clone())), // TODO: Can't own it without mem::replace?
            false => Ok(None)
        }
    }
}
