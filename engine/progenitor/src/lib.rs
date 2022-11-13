#[macro_use]
extern crate macro_rules_attribute;

mod errors;
mod schema;
mod serial;
mod store;
mod state;
mod effects;
mod registry;

// TODO: Temporary.
pub use log;

pub use self::errors::InitError;
pub use self::schema::{Type, SchemaError, Value, Expression, Mutation};
pub use self::serial::{SerialError, SerialFormat, SerialValue};
pub use self::store::{Store, StoreError};
pub use self::state::StateError;
pub use self::effects::{EffectError, EffectFn, Context};
pub use self::registry::Registry;

// TODO: Different packaging.
pub mod effect {
    pub use super::effects::{store_read, store_write, open_store};
}

pub mod ext {
    // TODO: All of these should be crate external.
    pub use super::store::ext::*;
    pub use super::serial::ext::*;
}
