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
pub type EffectFn = fn(&mut Context) -> Pin<Box<dyn Future<Output = Result<(), EffectError>> + '_>>;

#[macro_export]
macro_rules! effect_fn {
    (
        $( #[$m: meta] )*
        $v: vis async fn $n: ident<$lt: lifetime>( $($a: tt)* ) -> $rv: ty
        {
            $($b: tt)*
        }
    ) => (
        $( #[$m] )*
        $v fn $n<$lt>( $($a)* ) -> 
            ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = $rv> + $lt>>
        {
            ::std::boxed::Box::pin(async move { $($b)* })
        }
    )
}
pub use effect_fn;

#[macro_export]
macro_rules! archetype_effect {
    ($n: ident, $in: literal, $($a: tt)*) => (
        pub fn $n<'ef>(context: &'ef mut $crate::Context) -> 
            ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = Result<(), $crate::EffectError>> + 'ef>>
        {
            ::std::boxed::Box::pin(async move {
                context.execute($in.into(), Some($($a)*)).await?;

                Ok(())
            })
        }
    )
}
pub use archetype_effect;

#[macro_export]
macro_rules! sequence_effect {
    ($n: ident, $($s: tt)*) => (
        pub fn $n<'ef>(context: &'ef mut $crate::Context) -> 
            ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = Result<(), $crate::EffectError>> + 'ef>>
        {
            ::std::boxed::Box::pin(async move {
                let seq = $($s)*;

                for effect in seq {
                    context.execute(effect.into(), None).await?;
                }

                Ok(())
            })
        }
    )
}
pub use sequence_effect;
