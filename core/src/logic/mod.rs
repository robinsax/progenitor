// Abstract encapsulation of logical flow and supporting state.
mod effect;
mod state;

pub use state::{State, StateError};
pub use effect::{EffectError, EffectExecutor, EffectFn};
