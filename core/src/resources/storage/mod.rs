mod base;
mod mongodb_backend;

pub use base::{Store, Query, StoreOperationError};

pub mod ext {
    pub use super::base::{StoreBackend, StorageDriver};
    pub use super::mongodb_backend::{MongoDBStoreBackend, MongoDBDriver};
}