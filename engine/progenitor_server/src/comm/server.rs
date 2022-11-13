use std::pin::Pin;
use std::sync::Arc;
use std::future::Future;

use bytes::Bytes;
use log::debug;

use progenitor::{EffectError, InitError, SerialValue, Registry, Context};

use super::driver::CommDriver;
use super::errors::CommError;
use super::io::{Request, Response};

// TODO: None of this cloning.

#[derive(Clone)]
pub struct Server<D>
where
    D: CommDriver + Clone
{
    registry: Arc<Registry>,
    driver: Arc<D>
}

impl<D> Server<D>
where
    D: CommDriver + Clone
{
    pub fn new(registry: Arc<Registry>) -> Result<Self, InitError> {
        let driver = Arc::new(D::new(registry.clone())?);

        Ok(Self {
            registry,
            driver
        })
    }

    pub fn start(&self) -> Result<(), CommError> {
        self.driver.handle_connections((*self).clone())
    }

    pub fn err_response(&self, err: CommError) -> Response {
        // TODO: Not this.
        Response::new(SerialValue::Buffer(Bytes::from(format!("server error\n\n{}", err))))
    }

    pub fn handle(&self, request: Request) -> Pin<Box<dyn Future<Output = Response> + '_>> {
        debug!("svr begin handle");

        Box::pin(async move {
            let mut context = Context::new(self.registry.clone());

            if let Err(err) = context.set("req", request) {
                return self.err_response(CommError::from(EffectError::from(err)));
            }

            let result = context.execute("main".into(), None).await;
            if let Err(err) = result {
                return self.err_response(CommError::from(err));
            };

            let resp = match context.get::<Response>("resp") {
                Ok(resp) => resp,
                Err(err) => return self.err_response(EffectError::from(err).into())
            };

            debug!("svr end handle");

            resp.clone()
        })
    }
}
