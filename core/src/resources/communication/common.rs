use std::net::SocketAddr;

use crate::resources::SerialError;

#[derive(Clone)]
pub struct BindOptions {
    pub address: SocketAddr
}

#[derive(Debug)]
pub enum CommunicationError {
    TODO
}

impl From<SerialError> for CommunicationError {
    fn from(_: SerialError) -> Self {
        CommunicationError::TODO
    }
}