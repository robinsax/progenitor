// Values in state are read-only to prevent a whole class of contention problems during
// concurrency. The fact state is copy-on-write (within a Context) is the other half
// of this strategy. 
use std::any::{Any, TypeId};
use std::sync::Arc;
use std::collections::HashMap;

use log::debug;

use super::errors::StateError;

#[derive(Clone)]
pub struct StateCell {
    type_id: TypeId,
    value: Arc<dyn Any + Send + Sync>
}

impl StateCell {
    fn new(
        type_id: TypeId,
        value: Arc<dyn Any + Send + Sync>
    ) -> Self {
        Self {
            type_id,
            value
        }
    }
}

#[derive(Clone)]
pub struct State {
    cells: HashMap<String, StateCell>
}

impl<'st> State {
    pub fn new() -> Self {
        Self {
            cells: HashMap::new()
        }
    }

    pub fn get<T>(&'st self, key_src: impl Into<String>) -> Result<&'st T, StateError>
    where
        T: Send + Sync + 'static
    {
        let key: String = key_src.into();

        debug!("state.get {}", key);

        let cell = match self.cells.get(&key) {
            None => return Err(StateError::Empty(key)),
            Some(cell) => cell
        };

        if TypeId::of::<T>() != cell.type_id {
            return Err(StateError::InvalidType(key.to_owned()));
        }

        // SAFETY: TypeId assertion above.
        Ok(unsafe { &*(
            &cell.value
                as *const Arc<dyn Any + Send + Sync>
                as *const Arc<T>
        ) })
    }

    pub fn set<T>(&'st mut self, key_src: impl Into<String>, value: T) -> Result<(), StateError>
    where
        T: Send + Sync + 'static
    {
        let key: String = key_src.into();

        debug!("state.set {}", key);

        let cell = StateCell::new(
            TypeId::of::<T>(),
            Arc::new(value) as Arc<dyn Any + Send + Sync>
        );

        self.cells.insert(key, cell);

        Ok(())
    }
}
