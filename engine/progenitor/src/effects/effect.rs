// Abstract, composable, units of work that operate on state.
use std::future::Future;
use std::pin::Pin;

use super::context::Context;
use super::errors::EffectError;

//  The type of an effect, the effect_fn macro can be used upgrade an
//
//  async fn effect<'ef>(&'ef State) -> Result<(), EffectError> 
//
//  to this type.
pub type EffectFn = fn(&mut Context) -> Pin<Box<dyn Future<Output = Result<(), EffectError>>>>;

#[macro_export]
macro_rules! effect_fn {
    (
        $( #[$m: meta] )*
        $v: vis async fn $n: ident<$lt: lifetime>( $($a: tt)* ) -> $rv: ty
        {
            $($body: tt)*
        }
    ) => (
        $( #[$m] )*
        $v fn $n<$lt>( $($a)* ) -> 
            ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = $rv> + $lt>>
        {
            ::std::boxed::Box::pin(async move { $($body)* })
        }
    )
}
pub use effect_fn;

/*
// Macro for an effect function that is a sequence of effects:
//
//  make_flow_effect_fn(my_sequence_effect, vec!["step_1", "step_2"])
#[macro_export]
macro_rules! make_flow_effect_fn {
    ($n: ident, $s: expr) => {
        #[apply($crate::effect_fn)]
        pub async fn $n<'ef>(context: &'ef $crate::State) -> Result<(), $crate::EffectError> {
            let seq = $s;

            for s in seq {
                executor.execute(s, state).await?;
            }

            Ok(())
        }
    };
}
pub use make_flow_effect_fn;
*/