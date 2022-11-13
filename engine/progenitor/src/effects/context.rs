use std::sync::Arc;
use std::pin::Pin;
use std::future::Future;

use crate::Registry;

use super::super::Value;
use super::super::state::{State, StateError};
use super::errors::EffectError;

pub struct Context {
    registry: Arc<Registry>,
    state: Arc<State>,
    state_dirty: bool,
    archetype: Option<Value>
}

impl Context {
    pub fn new(registry: Registry) -> Self {
        Self {
            registry: Arc::new(registry),
            state: Arc::new(State::new()),
            state_dirty: false,
            archetype: None
        }
    }

    pub async fn execute(
        &mut self, effect_name: String, archetype: Option<Value>
    ) -> Pin<Box<dyn Future<Output = Result<(), EffectError>> + '_>> {
        Box::pin(async move {
            let effect = self.registry.get_effect(effect_name.as_str())?;

            let mut derived = Self {
                registry: Arc::clone(&self.registry),
                state: Arc::clone(&self.state),
                state_dirty: false,
                archetype
            };
            
            effect(&mut derived).await?;

            if derived.state_dirty {
                self.state = derived.state;
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
        let mut copy = (*self.state).clone();
        copy.set(key_src, value)?;

        self.state = Arc::new(copy);
        self.state_dirty = true;

        Ok(())
    }

    pub fn get<T>(&self, key_src: impl Into<String>) -> Result<&T, StateError>
    where
        T: Send + Sync + 'static
    {
        self.state.get::<T>(key_src)
    }
}
