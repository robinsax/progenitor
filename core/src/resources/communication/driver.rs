use async_trait::async_trait;

use super::common::CommunicationError;
use super::handlers::RootHandler;

#[async_trait]
pub trait CommunicationDriver: Sync + Send {
    async fn handle_connections(&mut self, handler: &Box<dyn RootHandler>) -> Result<(), CommunicationError>;
}
