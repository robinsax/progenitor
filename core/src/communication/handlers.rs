use std::sync::Arc;

use async_trait::async_trait;

use crate::SerialValue;

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
    fn async_route(&self) -> Route;
    async fn handle_async(&self, req: &Request) -> Result<(), CommunicationError>;
}

#[async_trait]
pub trait SyncHandler: Send + Sync {
    fn sync_route(&self) -> Route;
    async fn handle_sync(&self, req: &Request) -> Result<Response, CommunicationError>;
}

#[async_trait]
impl<T: AsyncHandler> SyncHandler for T {
    fn sync_route(&self) -> Route {
        self.async_route()
    }
    
    async fn handle_sync(&self, req: &Request) -> Result<Response, CommunicationError> {
        self.handle_async(req).await?;

        Ok(Response::Success(SerialValue::empty()))
    }
}

#[derive(Clone)]
pub struct RoutingHandler {
    // TODO horrible perf
    routes: Arc<Vec<(Route, Box<dyn SyncHandler>)>>
}

#[async_trait]
impl SyncHandler for RoutingHandler {
    fn sync_route(&self) -> Route {
        "*".into()
    }

    async fn handle_sync(&self, req: &Request) -> Result<Response, CommunicationError> {
        for (route, handler) in self.routes.as_slice() {
            if route.path == req.route.path {
                return handler.handle_sync(req).await;
            }
        }

        Ok(Response::Error(ResponseError::TODO("no route match".into())))
    }
}

pub trait RootHandler: SyncHandler {
    fn deep_clone(&self) -> Box<dyn RootHandler>;
}

impl Clone for Box<dyn RootHandler> {
    fn clone(&self) -> Box<dyn RootHandler> {
        self.deep_clone()
    }
}

impl RoutingHandler {
    pub fn new(routes: Vec<(Route, Box<dyn SyncHandler>)>) -> Self {
        Self{
            routes: Arc::new(routes)
        }
    }
}

impl RootHandler for RoutingHandler {
    fn deep_clone(&self) -> Box<dyn RootHandler> {
        Box::new(self.clone())
    }
}
