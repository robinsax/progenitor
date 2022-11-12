use async_trait::async_trait;

use crate::schema::{Value, Type, Expression};

use super::errors::StoreError;
use super::driver::StoreDriver;
use super::query::{Query, QueryExecutor};

pub struct Store {
    driver: Box<dyn StoreDriver>,
    schema: Type
}

#[async_trait]
impl QueryExecutor for Store {
    async fn load(
        &self, filter: Option<Expression>, offset: usize, limit: Option<usize>
    ) -> Result<Vec<Value>, StoreError> {
        if let Some(filter_expr) = &filter {
            filter_expr.validate(&self.schema)?;
        }

        self.driver.load(filter, offset, limit).await
    }
}

impl Store {
    pub fn new(schema: Type, driver: Box<dyn StoreDriver>) -> Self {
        Self {
            schema,
            driver
        }
    }

    pub fn query(&self) -> Query<Self> {
        Query::new(&self)
    }

    pub async fn put(&self, item: Value) -> Result<(), StoreError> {
        self.driver.insert(Vec::from([item.clone()])).await
    }
}
