use crate::errors::InitError;

#[derive(Clone)]
pub struct EnvConfig;

pub trait FromEnvConfig
where
    Self: Sized
{
    fn try_from_config(env_config: EnvConfig) -> Result<Self, InitError>;
}
