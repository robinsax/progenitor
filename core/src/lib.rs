// TODO: Break out into engine / framework / common.
#[macro_use]
extern crate macro_rules_attribute;

mod errors;
mod env_config;
mod schema;
mod serial;
mod logic;
mod comm;
mod store;

// TODO: no *
pub use env_config::EnvConfig;
pub use schema::*;
pub use serial::*;
pub use logic::*;
pub use store::*;
pub use comm::*;

// TODO: Temporary.
pub mod ext {
    pub use super::store::ext::*;
    pub use super::comm::ext::*;
    pub use super::serial::ext::*;
}
