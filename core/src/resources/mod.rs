mod net;
mod persistence;

pub use persistence::{PersistentStore, Query, PersistenceError};

pub mod ext {
    pub use super::persistence::ext::*;
}
