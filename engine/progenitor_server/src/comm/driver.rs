use std::sync::Arc;

use progenitor::{Registry, InitError};

use super::errors::CommError;
use super::server::Server;

pub trait CommDriver
where
    Self: Sync + Send + Clone // TODO: Not clone.
{
    fn new(registry: Arc<Registry>) -> Result<Self, InitError>;
    fn handle_connections(&self, server: Server<Self>) -> Result<(), CommError>;
}
