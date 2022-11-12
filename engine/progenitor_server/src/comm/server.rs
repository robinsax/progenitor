use std::pin::Pin;
use std::sync::Arc;
use std::future::Future;

use bytes::Bytes;
use log::debug;

use progenitor::{InitConfig, EffectFn, State, EffectError, InitError, SerialValue};

use crate::lock::ServerLockAtomicFactory;
use super::driver::CommDriver;
use super::errors::CommError;
use super::io::{Request, Response};

// TODO: None of this cloning.

#[derive(Clone)]
pub struct Server<D>
where
    D: CommDriver + Clone
{
    entry_effect: EffectFn,
    driver: Arc<D>
}

impl<D> Server<D>
where
    D: CommDriver + Clone
{
    pub fn new(config: Box<dyn InitConfig>, entry_effect: EffectFn) -> Result<Self, InitError> {
        let driver = Arc::new(D::from_config(&config)?);
        
        Ok(Self {
            entry_effect,
            driver
        })
    }

    pub fn start(&self) -> Result<(), CommError> {
        self.driver.handle_connections((*self).clone())
    }

    pub fn err_response(&self, err: CommError) -> Response {
        // TODO: Not this.
        Response::new(SerialValue::Buffer(Bytes::from(format!("Server error: {:?}", err))))
    }

    pub fn handle(&self, request: Request) -> Pin<Box<dyn Future<Output = Response> + '_>> {
        debug!("svr begin handle");

        Box::pin(async move {
            let state = State::new(Box::new(ServerLockAtomicFactory::new()));

            if let Err(err) = state.set("req", request).await {
                return self.err_response(CommError::from(EffectError::from(err)));
            }

            if let Err(err) = (self.entry_effect)(&state).await {
                return self.err_response(err.into());
            }

            let resp = match state.get::<Response>("resp").await {
                Ok(resp) => resp,
                Err(err) => return self.err_response(EffectError::from(err).into())
            };

            debug!("svr end handle");

            resp.clone()
        })
    }
}
