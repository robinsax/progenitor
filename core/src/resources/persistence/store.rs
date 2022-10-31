use std::marker::PhantomData;

use crate::archetype::{DataType, LiteralValue};

use super::{
    common::PersistenceError,
    driver::PersistenceDriver,
    query::Query
};

pub struct PersistentStore<T> {
    phantom: PhantomData<T>,
    pub(crate) data_type: DataType,
    pub(crate) driver: Box<dyn PersistenceDriver>,
}

impl<T: From<LiteralValue> + Into<LiteralValue> + Clone> PersistentStore<T> {
    pub fn new(data_type: DataType, driver: Box<dyn PersistenceDriver>) -> Self {
        Self{
            phantom: PhantomData,
            data_type,
            driver
        }
    }

    pub fn query(&self) -> Query<T> {
        Query::new(&self)
    }

    pub async fn put(&self, item: T) -> Result<T, PersistenceError> {
        self.driver.insert(&[item.clone().into()]).await?;

        Ok(item)
    }
}
