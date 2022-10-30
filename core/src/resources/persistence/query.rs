use std::marker::PhantomData;

use crate::archetype::{LiteralValue, IndirectExpression};

use super::{common::PersistenceError, store::PersistentStore};

pub struct Query<'b, T> {
    phantom: PhantomData<T>,
    store: &'b PersistentStore<T>,
    filter: Option<IndirectExpression>,
}

impl<'b, T: From<LiteralValue> + Clone> Query<'b, T> {
    pub fn new(store: &'b PersistentStore<T>) -> Self {
        Self{
            phantom: PhantomData,
            store,
            filter: None
        }
    }

    pub fn filter(mut self, condition: IndirectExpression) -> Query<'b, T> {
        self.filter = Some(condition);

        self
    }

    async fn load(&self, limit: usize, offset: usize) -> Result<Vec<T>, PersistenceError> {
        let filter = match &self.filter {
            Some(f) => f,
            None => return Err(PersistenceError::TODO)
        };

        filter.validate_within(&self.store.data_type)?;

        let literal_results = self.store.backend.load(filter, limit, offset).await?;
        
        Ok(literal_results.into_iter().map(|r| r.into()).collect())
    }

    pub async fn one(&self) -> Result<Option<T>, PersistenceError> {
        let loaded = self.load(1, 0).await?;

        match loaded.len() > 0 {
            true => Ok(Some(loaded[0].clone())), // TODO wtf cant own it without mem::replace?
            false => Ok(None)
        }
    }
}
