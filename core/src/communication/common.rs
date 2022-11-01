use std::net::SocketAddr;

use crate::schema::SchemaError;

#[derive(Clone)]
pub struct BindOptions {
    pub address: SocketAddr
}

#[derive(Debug)]
pub enum CommunicationError {
    TODO(String)
}

impl From<SchemaError> for CommunicationError {
    fn from(err: SchemaError) -> Self {
        CommunicationError::TODO(format!("comm err from schema {:?}", err))
    }
}
