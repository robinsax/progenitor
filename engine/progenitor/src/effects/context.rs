use std::sync::Arc;
use std::pin::Pin;
use std::future::Future;

use super::super::ext::LockAtomicFactory;
use super::super::state::State;
use super::effect::EffectFn;
use super::errors::EffectError;

pub struct Context {
    state: Arc<State>
}

impl Context {
    pub fn new(lock_factory: Box<dyn LockAtomicFactory>) -> Self {
        Self {
            state: Arc::new(State::new(lock_factory))
        }
    }

    pub fn execute(&self, effect: EffectFn) -> Pin<Box<dyn Future<Output = Result<(), EffectError>> + '_>> {
        effect(self)
    }
}
