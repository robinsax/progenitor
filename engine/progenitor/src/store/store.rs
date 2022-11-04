use std::marker::PhantomData;
use std::pin::Pin;
use std::future::Future;

use async_trait::async_trait;

use crate::{FromEnvConfig, EnvConfig};
use crate::errors::InitError;
use crate::schema::{Type, Expression};
use crate::serial::StreamSerial;

use super::errors::StoreError;
use super::driver::StoreDriver;
use super::query::{Query, QueryExecutor};

// TODO: Clone here is temporary.
#[derive(Clone)]
pub struct Store<T, D>
where
    D: StoreDriver
{
    phantom: PhantomData<T>,
    driver: D,
    pub(crate) schema: Type,
}

#[async_trait]
impl<T, D> QueryExecutor<T> for Store<T, D>
where 
    T: StreamSerial + Clone + Send + Sync,
    D: StoreDriver
{
    async fn load(&self, filter: Option<Expression>, offset: usize, limit: Option<usize>) -> Result<Vec<T>, StoreError> {
        if let Some(filter_expr) = &filter {
            filter_expr.validate(&self.schema)?;
        }

        self.driver.load(filter, offset, limit)
    }
}

impl<T, D> FromEnvConfig for Store<T, D>
where
    T: StreamSerial + Clone + Send + Sync,
    D: StoreDriver
{
    fn try_from_config(env_config: EnvConfig) -> Result<Self, InitError> {
        Ok(Self {
            phantom: PhantomData,
            schema: T::schema(),
            driver: D::try_from_config(env_config)?
        })
    }
}

impl<T, D> Store<T, D>
where 
    // TODO: Why Clone??
    T: StreamSerial + Clone + Send + Sync,
    D: StoreDriver
{
    pub fn query(&self) -> Query<T, Self> {
        Query::new(&self)
    }

    pub fn put(&self, item: T) -> Pin<Box<dyn Future<Output = Result<T, StoreError>> + Send + Sync + '_>> {
        Box::pin(async move {
            self.driver.insert(Vec::from([item.clone()]))?;

            Ok(item)
        })
    }
}
