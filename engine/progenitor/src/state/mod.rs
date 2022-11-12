// The state synchronization system is designed to (eventually) meet the following
// requirements:
// * Work in WASM and standard targets.
// * Async acquisition.
// * Multiple readers.
// * Have guards that can span yield points in !Send futures (obviously).
mod errors;
mod lock;
mod futures;
mod state;

pub use errors::StateError;
pub use lock::{LockAtomic, LockAtomicGuard, LockAtomicFactory};
pub use state::{State, StateCellGuard};
