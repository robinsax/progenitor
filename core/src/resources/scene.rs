use serde::de::DeserializeOwned;

use crate::archetype::{Collection, Model};

use super::storage::{Store, ext::{StorageDriver, MongoDBDriver}, StoreOperationError};

pub struct Scene {
}

impl Scene {
    pub fn new() -> Self {
        Self{}
    }

    pub async fn store<'a, T: DeserializeOwned>(collection: &Collection, model: &'a Model) -> Result<Store<'a, T>, StoreOperationError> {
        let driver = MongoDBDriver::new(); // TODO collection dependent

        let backend = driver.open("".to_string(), model).await?;

        Ok(Store::<T>::new(model, backend))
    }
}
