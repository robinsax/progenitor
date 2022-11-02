use std::sync::Arc;

use async_trait::async_trait;

use crate::env_config::FromEnvConfig;

use super::errors::CommError;
use super::server::Server;

#[async_trait]
pub trait CommunicationDriver
where
    Self: Sync + Send + FromEnvConfig
{
    async fn handle_connections(self, server: Arc<Server<Self>>) -> Result<(), CommError>;
}
