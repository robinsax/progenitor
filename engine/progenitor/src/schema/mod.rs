// Schema representation via indirection.
// TODO: Rename to meta?
// TODO: Greatly expanded implementation needed.
// TODO: Support masks when validating indirects; e.g. references / comparators supported by a database.
mod errors;
mod primitives;
mod expr;
mod mutation;

pub use errors::SchemaError;
pub use primitives::{Type, Value};
pub use expr::{Comparator, Conjunctive, Condition, ValueReference};
pub use mutation::Mutation;
