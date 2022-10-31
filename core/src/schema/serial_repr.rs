use super::errors::SchemaError;
use super::serial::{SerialValue, SerialFormat};
use super::indirect_value::IndirectValue;

pub trait SerialRepr: Sized + Send + Sync + TryInto<IndirectValue, Error = SchemaError> { // TODO try into constraint sooo temporary
    fn deserialize(serial: impl SerialFormat) -> Result<Self, SchemaError>;
    fn serialize(&self, writer: impl SerialFormat) -> Result<SerialValue, SchemaError>;
}

pub mod macros {
    #[macro_export]
    macro_rules! lookup_auto {
        ($s: ident, $k: expr) => {
            $s.lookup($k)?.try_into()?.try_into()?
        };
    }

    #[macro_export]
    macro_rules! elements_auto {
        ($e: ident) => {
            {
                let mut result = Vec::new();

                for element in $e.elements()? {
                    result.push(element.try_into()?.try_into()?);
                }

                result
            }
        };
    }

    pub use lookup_auto;
    pub use elements_auto;
}
