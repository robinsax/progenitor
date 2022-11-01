use std::sync::Arc;

use async_trait::async_trait;

use crate::{SerialValue, PersistentStore, ext::PersistenceDriver, SerialRepr};

use super::common::CommunicationError;

#[derive(Clone)]
pub struct Route {
    pub path: String,
}

impl Route {
    pub fn new(path: String) -> Self {
        Self {
            path
        }
    }
}

impl From<&'static str> for Route {
    fn from(path: &'static str) -> Self {
        Route::new(path.to_string())
    }
}

pub struct Request {
    pub route: Route,
    pub payload: SerialValue // TODO well see how this goes
}

#[derive(Debug)]
pub enum ResponseError {
    TODO(String)
}

pub enum Response {
    Success(SerialValue),
    Error(ResponseError)
}

impl From<CommunicationError> for Response {
    fn from(err: CommunicationError) -> Self {
        Self::Error(ResponseError::TODO(format!("resp from {:?}", err)))
    }
}

#[async_trait]
pub trait AsyncHandler: Send + Sync {
    fn new() -> Self;
    async fn handle_async(&self, req: Request) -> Result<(), CommunicationError>;
}

#[async_trait]
pub trait SyncHandler: Send + Sync {
    fn new() -> Self;
    async fn handle_sync(&self, req: Request) -> Result<Response, CommunicationError>;
}

#[async_trait]
impl<T: AsyncHandler> SyncHandler for T {
    fn new() -> Self {
        T::new()        
    }

    async fn handle_sync(&self, req: Request) -> Result<Response, CommunicationError> {
        self.handle_async(req).await?;

        Ok(Response::Success(SerialValue::empty()))
    }
}
