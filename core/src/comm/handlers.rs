// Top-level invoked units. Conceptually, application level code should very rarely
// live directly within a handler.
use std::pin::Pin;
use std::future::Future;

use crate::serial::SerialError;
use crate::schema::SchemaError;

use super::io::{Request, Response};

// TODO: Is this the correct construct?
#[derive(Debug)]
pub enum HandlerError {
    Serial(SerialError),
    Schema(SchemaError),
    // TODO: Diagnostic?
    Interal
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

// A synchronous communication handler that returns a response.
pub type HandlerFn = fn(Request) -> Pin<Box<dyn Future<Output = Result<Response, HandlerError>> + Send>>;

#[macro_export]
macro_rules! handler_fn {
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
pub use handler_fn;
