use std::sync::Arc;
use std::pin::Pin;
use std::future::Future;

use log::debug;

use crate::Registry;

use super::super::Value;
use super::super::state::{State, StateError};
use super::errors::EffectError;

pub struct Context {
    registry: Arc<Registry>,
    state: Arc<State>,
    state_dirty: bool,
    archetype: Option<Value>,
    // TODO: Temp impl.
    stack: Vec<String>
}

impl Context {
    pub fn new(registry: Arc<Registry>) -> Self {
        Self {
            registry,
            state: Arc::new(State::new()),
            state_dirty: false,
            archetype: None,
            stack: Vec::new()
        }
    }

    pub fn execute(
        &mut self, effect_name: String, archetype: Option<Value>
    ) -> Pin<Box<dyn Future<Output = Result<(), EffectError>> + '_>> {
        Box::pin(async move {
            let effect = self.registry.get_effect(effect_name.as_str())?;

            let mut new_stack = self.stack.clone();
            new_stack.push(effect_name.clone());

            debug!("derive {:?} {:?}", new_stack, self.state);

            let mut derived = Self {
                registry: Arc::clone(&self.registry),
                state: Arc::clone(&self.state),
                state_dirty: false,
                archetype,
                stack: new_stack
            };

            if let Err(err) = effect(&mut derived).await {
                return Err(EffectError::Stack(effect_name, Box::new(err)));
            }

            if derived.state_dirty {
                debug!("state rectify <- {:?} ({:?})", derived.state, self.stack);

                self.state = derived.state;
                self.state_dirty = true;
            }

            Ok(())
        })
    }

    pub fn archetype(&self) -> Result<&'_ Value, EffectError> {
        match &self.archetype {
            Some(archetype) => Ok(archetype),
            None => Err(EffectError::Internal("missing archetype".into()))
        }
    }

    pub fn registry(&self) -> &'_ Registry {
        &self.registry
    }

    pub fn state(&mut self) -> &'_ mut Arc<State> {
        &mut self.state
    }

    pub fn set<T>(&mut self, key_src: impl Into<String>, value: T) -> Result<(), StateError>
    where
        T: Send + Sync + 'static
    {
        let key: String = key_src.into();

        debug!("state.set {} @ {:?}", key.clone(), self.stack);

        let mut copy = (*self.state).clone();
        copy.set(key, value)?;

        self.state = Arc::new(copy);
        self.state_dirty = true;

        Ok(())
    }

    pub fn get<T>(&self, key_src: impl Into<String>) -> Result<&T, StateError>
    where
        T: Send + Sync + 'static
    {
        let key: String = key_src.into();

        debug!("state.get {} @ {:?}", key.clone(), self.stack);

        self.state.get::<T>(key)
    }
}
