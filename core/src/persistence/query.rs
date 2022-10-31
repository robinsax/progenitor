use std::marker::PhantomData;

use async_trait::async_trait;

use crate::schema::{IndirectExpression, SerialRepr};

use super::common::PersistenceError;

#[async_trait]
pub trait QueryExecutor<T: SerialRepr>: Send + Sync {
    async fn load(&self, filter: IndirectExpression, offset: usize, limit: usize) -> Result<Vec<T>, PersistenceError>;
}

pub struct Query<'b, T: SerialRepr, E: QueryExecutor<T>> {
    _data: PhantomData<T>,
    executor: &'b E,
    filter: Option<IndirectExpression>,
}

impl<'b, T: SerialRepr + Clone, E: QueryExecutor<T>> Query<'b, T, E> {
    pub fn new(executor: &'b E) -> Self {
        Self{
            _data: PhantomData,
            executor,
            filter: None
        }
    }

    pub fn filter(mut self, condition: IndirectExpression) -> Query<'b, T, E> {
        self.filter = Some(condition);

        self
    }

    async fn load(self, limit: usize, offset: usize) -> Result<Vec<T>, PersistenceError> {
        let filter = match self.filter {
            Some(f) => f,
            None => return Err(PersistenceError::TODO)
        };

        let literal_results = self.executor.load(filter, limit, offset).await?;
        
        Ok(literal_results.into_iter().map(|r| r.into()).collect())
    }

    pub async fn one(self) -> Result<Option<T>, PersistenceError> {
        let loaded = self.load(1, 0).await?;

        match loaded.len() > 0 {
            true => Ok(Some(loaded[0].clone())), // TODO wtf cant own it without mem::replace?
            false => Ok(None)
        }
    }
}
