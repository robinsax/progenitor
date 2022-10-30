mod storage;

pub use storage::{Store, Query, StorageOpError};

pub mod ext {
    pub use super::storage::ext::*;
}
