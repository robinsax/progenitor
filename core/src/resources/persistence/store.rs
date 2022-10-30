use std::marker::PhantomData;

use crate::archetype::{DataType, LiteralValue};

use super::{
    common::PersistenceError,
    driver::PersistentStoreBackend,
    query::Query
};

pub struct PersistentStore<T> {
    phantom: PhantomData<T>,
    pub(crate) data_type: DataType,
    pub(crate) backend: Box<dyn PersistentStoreBackend>,
}

impl<T: From<LiteralValue> + Into<LiteralValue> + Clone> PersistentStore<T> {
    pub fn new(data_type: DataType, backend: Box<dyn PersistentStoreBackend>) -> Self {
        Self{
            phantom: PhantomData,
            data_type,
            backend
        }
    }

    pub fn query(&self) -> Query<T> {
        Query::new(&self)
    }

    pub async fn put(&self, item: T) -> Result<T, PersistenceError> {
        self.backend.insert(&[item.clone().into()]).await?;

        Ok(item)
    }
}
