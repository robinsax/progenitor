mod common;
mod driver;
mod query;
mod store;

pub use common::StorageOpError;
pub use query::Query;
pub use store::Store;

// TODO as extensions
mod mongodb_backend;
mod inmem_backend;

pub mod ext {
    pub use super::driver::{StorageDriver, StoreBackend};

    // TODO as extensions
    pub use super::mongodb_backend::{MongoDBStoreBackend, MongoDBStorageDriver};
    pub use super::inmem_backend::{InMemoryStorageDriver, InMemoryStoreBackend};
}
