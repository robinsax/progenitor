#[macro_use]
extern crate macro_rules_attribute;

mod init;
mod schema;
mod serial;
mod store;
mod state;
mod effects;

// TODO: Temporary.
pub use log;

pub use init::{InitError, InitConfig, ConfigInit};
pub use schema::{Type, SchemaError, Value, Expression, Mutation};
pub use serial::{SerialError, SerialFormat, SerialValue};
pub use store::{Store, StoreError};
pub use state::{State, StateCellGuard, StateError};
pub use effects::{EffectError, EffectFn, EffectExecutor};

pub mod ext {
    pub use super::state::{LockAtomic, LockAtomicFactory, LockAtomicGuard};

    // TODO: All of these should be crate external.
    pub use super::store::ext::*;
    pub use super::serial::ext::*;
}
