use std::sync::PoisonError;
use std::env::VarError;

#[derive(Debug, PartialEq)]
pub enum InitError {
    DuplicateEffect(String),
    NotImplemented(String),
    EnvVar(VarError),
    Poisoned
}

impl<T> From<PoisonError<T>> for InitError {
    fn from(_: PoisonError<T>) -> Self {
        Self::Poisoned
    }
}

impl From<VarError> for InitError {
    fn from(err: VarError) -> Self {
        Self::EnvVar(err)
    }
}
