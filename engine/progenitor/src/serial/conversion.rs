use crate::schema::Type;

use super::errors::SerialError;
use super::value::SerialValue;
use super::format::{SerialReader, SerialWriter, SerialFormat};

// Conversion trait for concrete data models.
pub trait StreamSerial
where
    Self: Sized + Send + Sync
{
    fn schema() -> Type;
    fn stream_deserialize(reader: &mut impl SerialReader) -> Result<Self, SerialError>;
    fn stream_serialize(self, serial: &mut impl SerialWriter) -> Result<(), SerialError>;
}

pub trait DirectSerial
where
    Self: Sized + Send + Sync
{
    fn deserialize<F: SerialFormat>(serial: SerialValue) -> Result<Self, SerialError>;
    fn serialize<F: SerialFormat>(self) -> Result<SerialValue, SerialError>;
}

impl<T> DirectSerial for T
where
    T: StreamSerial
{
    fn deserialize<F: SerialFormat>(serial: SerialValue) -> Result<Self, SerialError> {
        T::stream_deserialize(&mut F::new_reader(serial))
    }

    fn serialize<F: SerialFormat>(self) -> Result<SerialValue, SerialError> {
        let mut writer = F::new_writer();

        self.stream_serialize(&mut writer)?;

        writer.flush()
    }
}

// Macros for common conversion tasks.
#[macro_export]
macro_rules! lookup_auto {
    ($s: ident, $k: expr) => {
        $s.lookup($k)?.try_into()?.try_into()?
    };
}
pub use lookup_auto;

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
pub use elements_auto;
