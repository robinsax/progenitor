// (De)serialization. Unfortunately we have some unique constraints that prevent serde
// from being a long term option.
mod errors;
mod value;
mod format;

// TODO: Should be a separate (extension) crates eventually.
mod ext_json;

pub use errors::SerialError;
pub use value::SerialValue;
pub use format::SerialFormat;

pub mod ext {
    pub use super::ext_json::JsonSerial;
}
