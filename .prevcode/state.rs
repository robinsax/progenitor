// might want to return to using strict state keys later:

use std::{
    sync::{RwLock, Mutex, MutexGuard, PoisonError}, any::{Any, TypeId},
    marker::PhantomData, fmt::Debug
};

#[derive(Debug, PartialEq)]
pub enum StateError {
    Empty(String),
    InvalidKey(String),
    InvalidType(String),
    Poisoned
}

impl<T> From<PoisonError<T>> for StateError {
    fn from(_: PoisonError<T>) -> Self {
        StateError::Poisoned
    }
}

pub trait StateKey
where
    Self: Debug
{
    fn state_shape() -> Vec<TypeId>;
    fn to_state_key(&self) -> usize;
}

impl<K> StateKey for &K
where
    K: StateKey
{
    fn state_shape() -> Vec<TypeId> {
        K::state_shape()
    }

    fn to_state_key(&self) -> usize {
        (*self).to_state_key()
    }
}

struct StateCell(TypeId, Option<Box<Mutex<dyn Any + Send + Sync>>>);

pub struct State<K>
where
    K: StateKey
{
    _key: PhantomData<K>,
    state: RwLock<Vec<StateCell>>
}

macro_rules! state_key_err {
    ($v: path, $k: ident) => {
        Err($v(format!("<key: {:?}>", $k)))
    }
}

impl<'s, K> State<K>
where
    K: StateKey
{
    pub fn new() -> Self {
        let state_shape = K::state_shape();
        let mut state = Vec::with_capacity(state_shape.len());

        for type_id in state_shape {
            state.push(StateCell(type_id, None));
        }

        Self {
            _key: PhantomData,
            state: RwLock::new(state)
        }
    }

    pub fn get<T: Send + Sync + 'static>(&self, key: impl StateKey) -> Result<MutexGuard<'s, T>, StateError> {
        let state = self.state.read()?;

        let key_idx = key.to_state_key();

        let cell = match state.get(key_idx) {
            None => return state_key_err!(StateError::InvalidKey, key),
            Some(cell) => cell
        };

        if TypeId::of::<T>() != cell.0 {
            return state_key_err!(StateError::InvalidType, key);
        }

        let cell_content = match &cell.1 {
            None => return state_key_err!(StateError::Empty, key),
            Some(content) => content
        };

        let cast_value = unsafe { &*(
            cell_content
                as *const Box<Mutex<dyn Any + Send + Sync>>
                as *const Box<Mutex<T>>
        ) };

        Ok(cast_value.lock()?)
    }

    pub fn set<T: Send + Sync + 'static>(&self, key: impl StateKey, value: T) -> Result<(), StateError> {
        let mut state = self.state.write()?;

        let key_idx = key.to_state_key();

        let cell = match state.get(key_idx) {
            None => return state_key_err!(StateError::InvalidKey, key),
            Some(cell) => cell
        };

        let type_id = cell.0;
        if TypeId::of::<T>() != type_id {
            return state_key_err!(StateError::InvalidType, key);
        }

        let new_cell_content = Box::new(Mutex::new(value));

        let abstract_value = new_cell_content
                as Box<Mutex<T>>
                as Box<Mutex<dyn Any + Send + Sync>>;

        drop(cell);
        state.insert(key_idx, StateCell(type_id, Some(abstract_value)));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    enum FooKey {
        A,
        B,
        Invalid
    }

    impl StateKey for FooKey {
        fn state_shape() -> Vec<TypeId> {
            vec![
                TypeId::of::<Bar>(),
                TypeId::of::<i32>(),
            ]
        }

        fn to_state_key(&self) -> usize {
            match self {
                Self::A => 0,
                Self::B => 1,
                _ => 32
            }
        }
    }

    #[derive(Debug)]
    struct Bar(String);

    #[test]
    fn get_and_set() {
        let state = State::<FooKey>::new();

        assert_eq!(state.get::<Bar>(&FooKey::A).err(), Some(StateError::Empty("<key: A>".into())));

        assert_eq!(state.get::<i32>(FooKey::A).err(), Some(StateError::InvalidType("<key: A>".into())));

        assert_eq!(state.get::<Bar>(FooKey::B).err(), Some(StateError::InvalidType("<key: B>".into())));

        assert!(state.set::<Bar>(FooKey::A, Bar("bar!".into())).is_ok());

        assert_eq!(state.get::<Bar>(FooKey::A).expect("failed to read valid state").0, "bar!".to_string());

        assert_eq!(state.get::<Bar>(FooKey::Invalid).err(), Some(StateError::InvalidKey("<key: Invalid>".into())));
    }
}
