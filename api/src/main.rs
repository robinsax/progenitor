use tokio;
use async_trait::async_trait;

use progenitor::archetype::{LiteralValue, DataType}; // TODO another red flag
use progenitor::ext::InMemoryPersistenceDriver;
use progenitor::inst::{SyncHandler, Request, Response, ResponseError, PersistentStore, Route, Server};

pub struct Greeting {
    message: String
}

pub struct GreetingResponse {
    greeting: String,
    prev_greetings: Vec<String>
}

pub struct HelloHandler {
    store: PersistentStore<Greeting>
}

#[async_trait]
impl SyncHandler for HelloHandler {
    fn sync_route(&self) -> Route {
        "*".into()
    }
    
    async fn handle_sync(&self, req: &Request) -> Response {
        Response::Error(ResponseError::TODO, LiteralValue::Null { real_type: None })
    }
}

#[tokio::main]
async fn main() {
    let store = PersistentStore::new(
        DataType::Int32, // TODO
        Box::new(InMemoryPersistenceDriver::new())
    );

    let hello_handler = HelloHandler{ store }

    let server = Server::new();
}