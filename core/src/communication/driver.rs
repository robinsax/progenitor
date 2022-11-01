use async_trait::async_trait;

use crate::SyncHandler;

use super::common::CommunicationError;

#[async_trait]
pub trait CommunicationDriver<H: SyncHandler>: Sync + Send {
    fn new() -> Self;
    async fn handle_connections(self) -> Result<(), CommunicationError>;
}
