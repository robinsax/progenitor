use std::marker::PhantomData;

use async_trait::async_trait;

use crate::schema::Expression;
use crate::serial::StreamSerial;

use super::errors::PersistenceError;

#[async_trait]
pub trait QueryExecutor<T: StreamSerial>: Send + Sync {
    async fn load(&self, filter: Expression, offset: usize, limit: usize) -> Result<Vec<T>, PersistenceError>;
}

pub struct Query<'b, T: StreamSerial, E: QueryExecutor<T>> {
    _data: PhantomData<T>,
    executor: &'b E,
    filter: Option<Expression>,
}

impl<'b, T: StreamSerial + Clone, E: QueryExecutor<T>> Query<'b, T, E> {
    pub fn new(executor: &'b E) -> Self {
        Self{
            _data: PhantomData,
            executor,
            filter: None
        }
    }

    pub fn filter(mut self, condition: Expression) -> Query<'b, T, E> {
        self.filter = Some(condition);

        self
    }

    async fn load(self, limit: usize, offset: usize) -> Result<Vec<T>, PersistenceError> {
        let filter = match self.filter {
            Some(expr) => expr,
            None => return Err(PersistenceError::QueryInvalid("Filter required when loading".into()))
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
