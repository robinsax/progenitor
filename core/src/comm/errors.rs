use crate::{errors::InitError, HandlerError};

#[derive(Debug)]
pub enum CommError {
    Init(InitError),
    // TODO: Diagnostic? Or bump invalid route encapsulation to HandlerError.
    NoHandler,
    Handler(HandlerError),
    NotImplemented(String)
}

impl From<InitError> for CommError {
    fn from(err: InitError) -> Self {
        Self::Init(err)
    }
}

impl From<HandlerError> for CommError {
    fn from(err: HandlerError) -> Self {
        Self::Handler(err)
    }
}
