// (De)serialization. Unfortunately we have some unique constraints that prevent serde
// from being a long term option.
mod errors;
mod value;
mod format;
mod conversion;

// TODO: Should be a separate (extension) crates eventually.
mod ext_json;
mod ext_pseudo;

pub use errors::SerialError;
pub use value::SerialValue;
pub use format::{SerialFormat, SerialReader, SerialWriter};
pub use conversion::{StreamSerial, DirectSerial};

pub mod ext {
    pub use super::ext_json::{JsonSerial, JsonSerialReader, JsonSerialWriter};
    pub use super::ext_pseudo::{PseudoSerial, PseudoSerialReader, PseudoSerialWriter};
}
