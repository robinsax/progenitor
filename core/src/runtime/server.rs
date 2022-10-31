use super::super::communication::{RootHandler, CommunicationError, ext::CommunicationDriver};

pub struct Server {
    driver: Box<dyn CommunicationDriver>,
    root_handler: Box<dyn RootHandler>
}

impl Server {
    pub fn new(
        driver: Box<dyn CommunicationDriver>,
        root_handler: Box<dyn RootHandler>
    ) -> Self {
        Self {
            driver, root_handler
        }
    }

    pub async fn start(&mut self) -> Result<(), CommunicationError> {
        self.driver.handle_connections(&self.root_handler).await
    }
}
