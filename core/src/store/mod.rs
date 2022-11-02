// Persistant storage, generic against backend, which is provided by a driver.
// TODO: Some kind of support mask.
mod errors;
mod driver;
mod query;
mod store;

pub use errors::PersistenceError;
pub use query::Query;
pub use store::PersistentStore;

// TODO: Should be a seperate (extension) crate.
mod ext_inmem;

pub mod ext {
    pub use super::driver::PersistenceDriver;

    pub use super::ext_inmem::InMemoryPersistenceDriver;
}
