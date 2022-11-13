// Effects each get their own Context, which contains a reference to the appropriate
// State for the effect, which has interior mutability.
mod errors;
mod effect;
mod context;
mod primitives;

pub use self::errors::EffectError;
pub use self::effect::EffectFn;
pub use self::context::Context;
pub use self::primitives::{store_read, store_write, open_store};