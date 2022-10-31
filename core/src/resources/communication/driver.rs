use async_trait::async_trait;

use crate::archetype::LiteralValue;

use super::common::{BindOptions, CommunicationError};
use super::handlers::RootHandler;

#[async_trait]
pub trait CommunicationDriverFactory {
    async fn create_server(&self, options: &BindOptions) -> Result<Box<dyn CommunicationDriver>, CommunicationError>;
}

#[async_trait]
pub trait CommunicationDriver: Sync + Send {
    async fn handle_connections(&mut self, handler: &Box<dyn RootHandler>) -> Result<(), CommunicationError>;
}
