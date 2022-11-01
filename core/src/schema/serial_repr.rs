use super::errors::SchemaError;
use super::serial::{SerialValue, SerialFormat};
use super::indirect_type::IndirectType;

// TODO clean up sized etc constraint base, it'll be relevant but is poorly expressed
pub trait SerialRepr: Sized + Send + Sync {
    fn schema() -> IndirectType; // TODO use to gen next 2
    fn deserialize(serial: impl SerialFormat) -> Result<Self, SchemaError>;
    fn serialize(&self, serial: &mut impl SerialFormat) -> Result<SerialValue, SchemaError>;
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
