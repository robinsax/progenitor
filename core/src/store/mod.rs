// Persistant storage, generic against backend, which is provided by a driver.
// TODO: Some kind of support mask.
mod errors;
mod driver;
mod query;
mod store;

pub use errors::StoreError;
pub use query::Query;
pub use store::Store;

// TODO: Should be a seperate (extension) crate.
mod ext_mem;

pub mod ext {
    pub use super::driver::StoreDriver;

    pub use super::ext_mem::MemStore;
}
