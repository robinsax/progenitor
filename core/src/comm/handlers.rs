// Top-level invoked units. Conceptually, application level code should very rarely
// live directly within a handler.
use std::pin::Pin;
use std::future::Future;

use crate::errors::InitError;
use crate::logic::{StateError, EffectError};
use crate::serial::SerialError;
use crate::schema::SchemaError;

use super::io::{Request, Response};

// TODO: Is this the correct construct?
#[derive(Debug)]
pub enum HandlerError {
    Serial(SerialError),
    Schema(SchemaError),
    State(StateError),
    Init(InitError),
    Effect(EffectError),
    // TODO: Diagnostic?
    Internal
}

impl From<SchemaError> for HandlerError {
    fn from(err: SchemaError) -> Self {
        Self::Schema(err)
    }
}

impl From<SerialError> for HandlerError {
    fn from(err: SerialError) -> Self {
        Self::Serial(err)
    }
}

impl From<StateError> for HandlerError {
    fn from(err: StateError) -> Self {
        Self::State(err)
    }
}

impl From<InitError> for HandlerError {
    fn from(err: InitError) -> Self {
        Self::Init(err)
    }
}

impl From<EffectError> for HandlerError {
    fn from(err: EffectError) -> Self {
        Self::Effect(err)
    }
}

// A synchronous communication handler that returns a response.
pub type HandlerFn = fn(Request) -> Pin<Box<dyn Future<Output = Result<Response, HandlerError>> + Send>>;

#[macro_export]
macro_rules! handler_fn {
    (
        $( #[$m: meta] )*
        $v: vis async fn $n: ident( $($a: tt)* ) $(-> $rv: ty)?
        {
            $($body: tt)*
        }
    ) => (
        $( #[$m] )*
        $v fn $n( $($a)* ) -> 
            ::std::pin::Pin<::std::boxed::Box<
                dyn ::std::future::Future<Output = $($rv)?> + ::std::marker::Send
            >>
        {
            ::std::boxed::Box::pin(async move { $($body)* })
        }
    )
}
pub use handler_fn;
