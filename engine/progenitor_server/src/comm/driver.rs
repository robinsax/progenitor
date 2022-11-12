use progenitor::ConfigInit;

use super::errors::CommError;
use super::server::Server;

pub trait CommDriver
where
    Self: Sync + Send + ConfigInit + Clone // TODO: Not clone.
{
    fn handle_connections(&self, server: Server<Self>) -> Result<(), CommError>;
}
