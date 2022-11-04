use std::sync::Arc;
use std::pin::Pin;
use std::future::Future;
use std::marker::PhantomData;

use bytes::Bytes;

use crate::HandlerFn;
use crate::errors::InitError;
use crate::env_config::{EnvConfig, FromEnvConfig};

use super::errors::CommError;
use super::driver::CommDriver;
use super::io::{Request, Response};

pub struct Server<D>
where
    D: CommDriver
{
    env_config: EnvConfig,
    handler: Option<HandlerFn>,
    _driver_t: PhantomData<D>
}

impl<D> FromEnvConfig for Server<D>
where
    D: CommDriver
{
    fn try_from_config(env_config: EnvConfig) -> Result<Self, InitError> {
        Ok(Self {
            env_config,
            handler: None,
            _driver_t: PhantomData
        })
    }
}

impl<D> Server<D>
where
    D: CommDriver
{
    pub async fn start(mut self, handler: HandlerFn) -> Result<(), CommError> {
        self.handler = Some(handler);

        D::try_from_config(self.env_config.clone())?.handle_connections(Arc::new(self)).await
    }

    pub fn handle_err(&self, err: CommError) -> Response {
        // TODO: Not this.
        Response::new(Bytes::from(format!("Server error: {:?}", err)).into())
    }

    async fn handle_impl(&self, request: Request) -> Response {
        let handler_fn = match self.handler {
            Some(handler_fn) => handler_fn,
            None => return self.handle_err(CommError::NoHandler)
        };

        match handler_fn(request).await {
            Ok(response) => response,
            Err(err) => self.handle_err(err.into())
        }
    }

    pub fn handle(&self, request: Request) -> Pin<Box<dyn Future<Output = Response> + Send + '_>> {
        Box::pin(self.handle_impl(request))
    }
}
