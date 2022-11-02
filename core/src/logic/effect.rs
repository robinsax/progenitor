// Abstract, composable, units of work that operate on state.
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, RwLock, PoisonError};

use crate::errors::InitError;

use super::state::{State, StateError};

#[derive(Debug, PartialEq)]
pub enum EffectError {
    Missing(String),
    State(StateError),
    Poisoned,
    Internal(String)
}

impl From<StateError> for EffectError {
    fn from(err: StateError) -> Self {
        Self::State(err)
    }
}

impl<T> From<PoisonError<T>> for EffectError {
    fn from(_: PoisonError<T>) -> Self {
        Self::Poisoned
    }
}

//  The type of an effect, the effect_fn macro can be used upgrade an
//
//  async fn effect<'ef>(&'ef State) -> Result<(), EffectError> 
//
//  to this type.
pub type EffectFn = fn(&State) -> Pin<Box<dyn Future<Output = Result<(), EffectError>> + Send + '_>>;

#[macro_export]
macro_rules! effect_fn {
    (
        $( #[$m: meta] )*
        $v: vis async fn $n: ident<$lt: lifetime>( $($a: tt)* ) $(-> $rv: ty)?
        {
            $($body: tt)*
        }
    ) => (
        $( #[$m] )*
        $v fn $n<$lt>( $($a)* ) -> 
            ::std::pin::Pin<::std::boxed::Box<
                dyn ::std::future::Future<Output = $($rv)?> + ::std::marker::Send + $lt
            >>
        {
            ::std::boxed::Box::pin(async move { $($body)* })
        }
    )
}
pub use effect_fn;

//  An executor able to register and execute effects by name. Effects have access
//  via state.
//  TODO: Better registry scope semantics, currently only effect-tree global.
pub struct EffectExecutor {
    registry: Arc<RwLock<HashMap<String, EffectFn>>>
}

impl Clone for EffectExecutor {
    fn clone(&self) -> Self {
        Self { registry: self.registry.clone() }
    }
}

impl EffectExecutor {
    pub fn new() -> Self {
        Self { registry: Arc::new(RwLock::new(HashMap::new())) }
    }

    pub fn register(&mut self, name_src: impl Into<String>, effect_fn: EffectFn) -> Result<(), InitError> {
        let mut registry = self.registry.write()?;
        let name = name_src.into();

        if registry.contains_key(&name) {
            return Err(InitError::DuplicateEffect(name));
        }

        registry.insert(name, effect_fn);

        Ok(())
    }


    pub async fn execute<'r>(&'r self, name_src: impl Into<String>, state: &'r State) -> Result<(), EffectError> {
        state.set::<EffectExecutor>("executor", self.clone())?;

        // Note to self: Block here to force guard destructor to run before the yield point.
        let effect_fn = {
            let registry = self.registry.read()?;
            let name = name_src.into();
    
            let effect_fn = match registry.get(&name) {
                None => return Err(EffectError::Missing(name)),
                Some(effect) => effect 
            };
            
            effect_fn.clone()
        };

        effect_fn(state).await
    }
}

// Macro for an effect function that is a sequence of effects:
//
//  make_sequence_effect_fn(my_sequence_effect, vec!["step_1", "step_2"])
#[macro_export]
macro_rules! make_sequence_effect_fn {
    ($n: ident, $s: expr) => {
        #[apply($crate::effect_fn)]
        pub async fn $n<'ef>(state: &'ef $crate::State) -> Result<(), $crate::EffectError> {
            let seq = $s;

            let executor = state.get::<$crate::EffectExecutor>("executor")?.clone();

            for s in seq {
                executor.execute(s, state).await?;
            }

            Ok(())
        }
    };
}
pub use make_sequence_effect_fn;

#[cfg(test)]
mod tests {
    use tokio;
    use super::*;

    #[apply(effect_fn)]
    async fn a<'ef>(state: &'ef State) -> Result<(), EffectError> {
        state.set::<String>("a", "a".into())?;

        Ok(())
    }

    #[apply(effect_fn)]
    async fn b<'ef>(state: &'ef State) -> Result<(), EffectError> {
        state.set::<String>("b", "b".into())?;

        Ok(())
    }

    #[apply(effect_fn)]
    async fn c<'ef>(state: &'ef State) -> Result<(), EffectError> {
        state.set::<String>("c", "c".into())?;

        Ok(())
    }

    make_sequence_effect_fn!(simple_seq, vec!["a", "b", "c"]);

    #[tokio::test]
    async fn execute_simple_sequence() {
        let mut executor = EffectExecutor::new();

        assert_eq!(executor.register("a", a), Ok(()));
        assert_eq!(executor.register("b", b), Ok(()));
        assert_eq!(executor.register("c", c), Ok(()));
        assert_eq!(executor.register("seq", simple_seq), Ok(()));

        let state = State::new();

        assert_eq!(executor.execute("seq", &state).await, Ok(()));

        assert!(state.get::<String>("a").is_ok());
        assert_eq!(state.get::<String>("a").unwrap().to_string(), "a".to_string());

        assert!(state.get::<String>("b").is_ok());
        assert_eq!(state.get::<String>("b").unwrap().to_string(), "b".to_string());

        assert!(state.get::<String>("c").is_ok());
        assert_eq!(state.get::<String>("c").unwrap().to_string(), "c".to_string());
    }
}
