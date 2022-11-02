use std::sync::PoisonError;

#[derive(Debug, PartialEq)]
pub enum InitError {
    DuplicateEffect(String),
    NotImplemented(String),
    Poisoned
}

impl<T> From<PoisonError<T>> for InitError {
    fn from(_: PoisonError<T>) -> Self {
        Self::Poisoned
    }
}
