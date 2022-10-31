use std::net::SocketAddr;

use crate::schema::SchemaError;

#[derive(Clone)]
pub struct BindOptions {
    pub address: SocketAddr
}

#[derive(Debug)]
pub enum CommunicationError {
    TODO
}

impl From<SchemaError> for CommunicationError {
    fn from(_: SchemaError) -> Self {
        CommunicationError::TODO
    }
}
