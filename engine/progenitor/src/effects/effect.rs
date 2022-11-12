// Abstract, composable, units of work that operate on state.
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use log::debug;

use crate::init::InitError;
use crate::state::State;

use super::errors::EffectError;

//  The type of an effect, the effect_fn macro can be used upgrade an
//
//  async fn effect<'ef>(&'ef State) -> Result<(), EffectError> 
//
//  to this type.
pub type EffectFn = fn(&State) -> Pin<Box<dyn Future<Output = Result<(), EffectError>> + '_>>;

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
            ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = $($rv)?> + $lt>>
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
    registry: HashMap<String, EffectFn>
}

impl Clone for EffectExecutor {
    fn clone(&self) -> Self {
        Self { registry: self.registry.clone() }
    }
}

impl EffectExecutor {
    pub fn new() -> Self {
        Self { registry: HashMap::new() }
    }

    pub fn register(&mut self, name_src: impl Into<String>, effect_fn: EffectFn) -> Result<(), EffectError> {
        let name = name_src.into();

        if self.registry.contains_key(&name) {
            return Err(InitError::Archetype(format!("duplicate effect: {}", name).into()).into());
        }

        self.registry.insert(name, effect_fn);

        Ok(())
    }

    pub async fn execute<'r>(&'r self, name_src: impl Into<String>, state: &'r State) -> Result<(), EffectError> {
        state.set::<EffectExecutor>("executor", self.clone()).await?;

        let name = name_src.into();

        debug!("exec effect {}", name);

        let effect_fn = match self.registry.get(&name) {
            None => return Err(EffectError::Missing(name)),
            Some(effect) => effect 
        };

        let rv = effect_fn(state).await;

        debug!("effect end {}", name);

        rv
    }
}

// Macro for an effect function that is a sequence of effects:
//
//  make_flow_effect_fn(my_sequence_effect, vec!["step_1", "step_2"])
#[macro_export]
macro_rules! make_flow_effect_fn {
    ($n: ident, $s: expr) => {
        #[apply($crate::effect_fn)]
        pub async fn $n<'ef>(state: &'ef $crate::State) -> Result<(), $crate::EffectError> {
            let seq = $s;

            let executor = state.get::<$crate::EffectExecutor>("executor").await?.clone();

            for s in seq {
                executor.execute(s, state).await?;
            }

            Ok(())
        }
    };
}
pub use make_flow_effect_fn;
