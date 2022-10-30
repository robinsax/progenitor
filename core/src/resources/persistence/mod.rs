mod common;
mod driver;
mod query;
mod store;

pub use common::PersistenceError;
pub use query::Query;
pub use store::PersistentStore;

// TODO as extensions
mod inmem_backend;

pub mod ext {
    pub use super::driver::{PersistenceDriver, PersistentStoreBackend};

    // TODO as extensions
    pub use super::inmem_backend::{InMemoryPersistenceDriver, InMemoryPersistentStoreBackend};
}
