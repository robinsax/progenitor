use std::marker::PhantomData;
use std::sync::Arc;

use super::handlers::{SyncHandler, Request, ResponseError, Response};
use super::common::CommunicationError;
use super::driver::CommunicationDriver;

pub struct Server<H: SyncHandler, D: CommunicationDriver<H>> {
    _d: PhantomData<D>,
    _h: PhantomData<H>
}

impl<H: SyncHandler, D: CommunicationDriver<H>> Server<H, D> {
    pub fn new() -> Self {
        Self {
            _d: PhantomData,
            _h: PhantomData
        }
    }

    pub async fn start(self) -> Result<(), CommunicationError> {
        D::new().handle_connections().await
    }
}
