use std::marker::PhantomData;

use crate::archetype::{DataType, LiteralValue};

use super::{
    common::StorageOpError,
    driver::StoreBackend,
    query::Query
};

pub struct Store<T> {
    phantom: PhantomData<T>,
    pub(crate) data_type: DataType,
    pub(crate) backend: Box<dyn StoreBackend>,
}

impl<T: From<LiteralValue> + Into<LiteralValue> + Clone> Store<T> {
    pub fn new(data_type: DataType, backend: Box<dyn StoreBackend>) -> Self {
        Self{
            phantom: PhantomData,
            data_type,
            backend
        }
    }

    pub fn query(&self) -> Query<T> {
        Query::new(&self)
    }

    pub async fn put(&self, item: T) -> Result<T, StorageOpError> {
        self.backend.insert(&[item.clone().into()]).await?;

        Ok(item)
    }
}
