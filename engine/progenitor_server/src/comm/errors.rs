use std::fmt::Display;

use progenitor::{EffectError, InitError};

#[derive(Debug)]
pub enum CommError {
    Init(InitError),
    Effect(EffectError),
    Interface(String)
}

impl From<InitError> for CommError {
    fn from(err: InitError) -> Self {
        Self::Init(err)
    }
}

impl From<EffectError> for CommError {
    fn from(err: EffectError) -> Self {
        Self::Effect(err)
    }
}

impl Display for CommError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<comm error: {:?}>", self)
    }
}