//  Concurrency and type safe state container for progressive evolution through logical flow.
use std::sync::{RwLock, Mutex, MutexGuard, PoisonError};
use std::any::{Any, TypeId};
use std::fmt::Debug;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum StateError {
    Empty(String),
    InvalidType(String),
    Poisoned
}

impl<T> From<PoisonError<T>> for StateError {
    fn from(_: PoisonError<T>) -> Self {
        Self::Poisoned
    }
}

struct StateCell(TypeId, Box<Mutex<dyn Any + Send + Sync>>);

//  TODO: Better scope semantics; at least derived scopes.
pub struct State {
    state: RwLock<HashMap<String, StateCell>>
}

impl<'s> State {
    pub fn new() -> Self {
        Self {
            state: RwLock::new(HashMap::new())
        }
    }

    pub fn get<T>(&self, key_src: impl Into<String>) -> Result<MutexGuard<'s, T>, StateError>
    where
        T: Send + Sync + 'static
    {
        let state = self.state.read()?;
        let key: String = key_src.into();

        let cell = match state.get(&key) {
            None => return Err(StateError::Empty(key)),
            Some(cell) => cell
        };

        if TypeId::of::<T>() != cell.0 {
            return Err(StateError::InvalidType(key));
        }

        let cast_value = unsafe {
            &*(&cell.1
                as *const Box<Mutex<dyn Any + Send + Sync>>
                as *const Box<Mutex<T>>
            )
        };

        Ok(cast_value.lock()?)
    }

    pub fn set<T>(&self, key_src: impl Into<String>, value: T) -> Result<(), StateError>
    where
        T: Send + Sync + 'static
    {
        let mut state = self.state.write()?;
        let key: String = key_src.into();

        // TODO: Should this even be enforced?
        let type_id = TypeId::of::<T>();
        if let Some(cell) = state.get(&key) {
            if type_id != cell.0 {
                return Err(StateError::InvalidType(key));
            }
        }

        let new_cell_content = Box::new(Mutex::new(value));

        let abstract_value = new_cell_content
                as Box<Mutex<dyn Any + Send + Sync>>;

        state.insert(key, StateCell(type_id, abstract_value));

        Ok(())
    }

    pub fn take<T>(&self, key_src: impl Into<String>) -> Result<T, StateError>
    where
        // TODO: Make this an actual take and remove the Clone bound.
        T: Send + Sync + Clone + 'static
    {
        let mut state = self.state.write()?;
        let key: String = key_src.into();

        let cell = match state.remove(&key) {
            None => return Err(StateError::Empty(key)),
            Some(cell) => cell
        };

        if TypeId::of::<T>() != cell.0 {
            return Err(StateError::InvalidType(key));
        }

        let cast_value = unsafe {
            &*(&cell.1
                as *const Box<Mutex<dyn Any + Send + Sync>>
                as *const Box<Mutex<T>>
            )
        };

        Ok(cast_value.lock()?.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Bar(String);

    #[test]
    fn get_and_set() {
        let state = State::new();

        assert_eq!(state.get::<Bar>("bar").err(), Some(StateError::Empty("bar".into())));

        assert_eq!(state.get::<i32>("other").err(), Some(StateError::Empty("other".into())));

        assert!(state.set::<Bar>("bar", Bar("bar!".into())).is_ok());

        assert_eq!(state.get::<Bar>("bar").expect("failed to read valid state").0, "bar!".to_string());

        assert_eq!(state.get::<i32>("bar").err(), Some(StateError::InvalidType("bar".into())));
    }
}
