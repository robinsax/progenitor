use std::sync::{Arc, RwLock};

use async_trait::async_trait;

use crate::archetype::LiteralValue;

use super::common::CommunicationError;

// TODO abstraction on from / into needed in general. also literalvalue needs replacement
pub trait HandlerInput: From<LiteralValue> {

}

pub trait HandlerOutput: Into<LiteralValue> {

}

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
    pub payload: LiteralValue
}

pub enum ResponseError {
    InternalError,
    InvalidRequest,
    TODO
}

pub enum Response {
    Success(LiteralValue),
    Error(ResponseError, LiteralValue)
}

impl From<CommunicationError> for Response {
    fn from(_: CommunicationError) -> Self {
        // TODO
        Self::Error(ResponseError::InternalError, LiteralValue::Null { real_type: None })
    }
}

// TODO better
impl From<Result<(), LiteralValue>> for Response {
    fn from(result: Result<(), LiteralValue>) -> Self {
        match result {
            Ok(_) => Self::Success(LiteralValue::Null { real_type: None }),
            Err(err) => Self::Error(ResponseError::TODO, err)
        }
    }
}

#[async_trait]
pub trait AsyncHandler: Send + Sync {
    fn async_route(&self) -> Route;
    async fn handle_async(&self, req: &Request) -> Result<(), LiteralValue>;
}

#[async_trait]
pub trait SyncHandler: Send + Sync {
    fn sync_route(&self) -> Route;
    async fn handle_sync(&self, req: &Request) -> Response;
}

#[async_trait]
impl<T: AsyncHandler> SyncHandler for T {
    fn sync_route(&self) -> Route {
        self.async_route()
    }
    
    async fn handle_sync(&self, req: &Request) -> Response {
        match self.handle_async(req).await {
            Ok(()) =>  Response::Success(LiteralValue::Null { real_type: None }),
            err => err.into()
        }
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

    async fn handle_sync(&self, req: &Request) -> Response {
        for (route, handler) in self.routes.as_slice() {
            if route.path == req.route.path {
                return handler.handle_sync(req).await;
            }
        }

        Response::Error(ResponseError::InvalidRequest, LiteralValue::Null { real_type: None })
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
