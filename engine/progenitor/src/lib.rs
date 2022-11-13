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

pub use errors::InitError;
pub use schema::{Type, SchemaError, Value, Expression, Mutation};
pub use serial::{SerialError, SerialFormat, SerialValue};
pub use store::{Store, StoreError};
pub use state::{State, StateError};
pub use effects::{EffectError, EffectFn};
pub use registry::Registry;

pub mod ext {
    // TODO: All of these should be crate external.
    pub use super::store::ext::*;
    pub use super::serial::ext::*;
}
