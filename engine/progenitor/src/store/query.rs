use std::marker::PhantomData;

use async_trait::async_trait;

use crate::schema::Expression;
use crate::serial::StreamSerial;

use super::errors::StoreError;

#[async_trait]
pub trait QueryExecutor<T: StreamSerial>: Send + Sync {
    async fn load(&self, filter: Option<Expression>, offset: usize, limit: Option<usize>) -> Result<Vec<T>, StoreError>;
}

pub struct Query<'b, T: StreamSerial, E: QueryExecutor<T>> {
    _data: PhantomData<T>,
    executor: &'b E,
    filter: Option<Expression>,
    limit: Option<usize>,
    offset: usize
}

impl<'b, T: StreamSerial + Clone, E: QueryExecutor<T>> Query<'b, T, E> {
    pub fn new(executor: &'b E) -> Self {
        Self{
            _data: PhantomData,
            executor,
            filter: None,
            limit: None,
            offset: 0
        }
    }

    pub fn filter(mut self, condition: Expression) -> Query<'b, T, E> {
        self.filter = Some(condition);

        self
    }

    pub fn limit(mut self, limit: usize) -> Query<'b, T, E> {
        self.limit = Some(limit);

        self
    }

    pub async fn all(self) -> Result<Vec<T>, StoreError> {
        let literal_results = self.executor.load(self.filter, self.offset, self.limit).await?;
        
        Ok(literal_results.into_iter().map(|r| r.into()).collect())
    }

    pub async fn one(self) -> Result<Option<T>, StoreError> {
        let loaded = self.limit(1).all().await?;

        match loaded.len() > 0 {
            true => Ok(Some(loaded[0].clone())), // TODO: Can't own it without mem::replace?
            false => Ok(None)
        }
    }
}
