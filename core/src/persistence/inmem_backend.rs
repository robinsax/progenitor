use std::sync::{RwLock, PoisonError};

use async_trait::async_trait;

use crate::SerialRepr;
use crate::schema::{SerialFormat, SerialValue, SchemaError, IndirectValue, IndirectExpression, IndirectMutation};

use super::common::PersistenceError;
use super::driver::PersistenceDriver;

impl<T> From<PoisonError<T>> for PersistenceError {
    fn from(_: PoisonError<T>) -> PersistenceError {
        PersistenceError::TODO
    }
}


/// TODO tmp dummy impl ...
struct DummySerial(IndirectValue);

impl From<SerialValue> for DummySerial {
    fn from(_: SerialValue) -> Self {
        panic!("na");
    }
}

impl From<DummySerial> for SerialValue {
    fn from(_: DummySerial) -> Self {
        panic!("na");
    }
}

impl TryFrom<IndirectValue> for DummySerial {
    type Error = SchemaError;

    fn try_from(value: IndirectValue) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}

impl TryFrom<DummySerial> for IndirectValue {
    type Error = SchemaError;

    fn try_from(dummy: DummySerial) -> Result<Self, Self::Error> {
        Ok(dummy.0)
    }
}

impl SerialFormat for DummySerial {
    fn new_writer() -> Self {
        panic!("na")
    }

    fn new_reader(_: bytes::Bytes) -> Self {
        panic!("na")
    }

    fn elements(&self) -> Result<Vec<Self>, SchemaError> {
        match &self.0 {
            IndirectValue::List(value) => {
                let mut format_wraps = Vec::new();

                for element in value {
                    format_wraps.push(element.clone().try_into()?);
                }

                Ok(format_wraps)
            },
            _ => Err(SchemaError::TODO("dummy elements lookup".into()))
        }
    }

    fn lookup(&self, key: &str) -> Result<Self, SchemaError> {
        Ok(Self(self.0.lookup(key)?))
    }

    fn write(&mut self, indirect: IndirectValue) -> Result<(), SchemaError> {
        self.0 = indirect;

        Ok(())
    }

    fn flush(&self) -> Result<SerialValue, SchemaError> {
        Ok(SerialValue::empty())
    }
}

/// ...dummy impl

pub struct InMemoryPersistenceDriver {
    data: RwLock<Vec<IndirectValue>>
}

impl InMemoryPersistenceDriver {
    pub fn new() -> Self {
        Self{ data: RwLock::new(Vec::new()) }
    }
}

#[async_trait]
impl PersistenceDriver for InMemoryPersistenceDriver {
    async fn load<T: SerialRepr>(&self, filter: IndirectExpression, limit: usize, offset: usize) -> Result<Vec<T>, PersistenceError> {
        let data = self.data.read()?;
        
        let mut until_offs = 0;
        let mut found: Vec<T> = Vec::new();
        for item in data.as_slice() {
            if filter.evaluate(item)? {
                if until_offs < offset {
                    until_offs += 1;
                    continue;
                }

                let concrete = T::deserialize(DummySerial(IndirectValue::Null))?;

                found.push(concrete);

                if found.len() == limit {
                    break
                }
            }
        }

        Ok(found)
    }

    async fn update(&self, filter: IndirectExpression, update: &IndirectMutation) -> Result<usize, PersistenceError> {
        let mut data = self.data.write()?;

        let mut updates: Vec<(usize, IndirectValue)> = Vec::new();
        for (i, item) in data.as_slice().into_iter().enumerate() {
            if filter.evaluate(item)? {
                updates.push((i, update.apply(item)?));
            }
        }

        let update_count = updates.len();
        for (i, updated) in updates {
            data[i] = updated;
        }

        Ok(update_count)
    }

    async fn delete(&self, filter: IndirectExpression) -> Result<usize, PersistenceError> {
        let mut data = self.data.write()?;

        let mut removals: Vec<usize> = Vec::new();
        for (i, item) in data.as_slice().into_iter().enumerate() {
            if filter.evaluate(item)? {
                removals.push(i);
            }
        }

        let removal_count = removals.len();
        for i in removals {
            data.remove(i);
        }

        Ok(removal_count)
    }

    async fn insert<T: SerialRepr>(&self, new_data: Vec<T>) -> Result<(), PersistenceError> {
        let mut indirects: Vec<IndirectValue> = Vec::new();
        for item in new_data {
            let mut ser = DummySerial(IndirectValue::Null);

            // TODO horrible hack
            item.serialize(&mut ser)?;

            indirects.push(ser.0);
        }

        let mut data = self.data.write()?;
        for item in indirects {
            data.push(item);
        }

        Ok(())
    }
}
