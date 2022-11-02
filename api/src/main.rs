use std::collections::HashMap;

use tokio;
use async_trait::async_trait;

use progenitor::ext::{
    InMemoryPersistenceDriver, Http1CommunicationDriver, JsonSerial
};
use progenitor::{
    SyncHandler, Type, SerialFormat, SerialValue, SchemaError,
    Request, Response, PersistentStore, Server, SerialRepr, CommError
};

#[derive(Clone)]
pub struct Greeting {
    message: String
}

// xxx gen
impl SerialRepr for Greeting {
    fn schema() -> Type {
        let mut fields = HashMap::new();

        fields.insert("message".into(), Type::String);

        Type::Map(fields)
    }

    fn deserialize(serial: impl SerialFormat) -> Result<Self, SchemaError> {
        use progenitor::macros::lookup_auto;
        
        Ok(Self {
            message: lookup_auto!(serial, "message"),
        })
    }

    fn serialize(&self, serial: &mut impl SerialFormat) -> Result<SerialValue, SchemaError> {
        serial.write(self.message.clone().try_into()?)?;

        serial.flush()
    }
}
// end gen

pub struct ClientInfo {
    name: String
}

// xxx gen
impl SerialRepr for ClientInfo {
    fn schema() -> Type {
        let mut fields = HashMap::new();

        fields.insert("name".into(), Type::String);

        Type::Map(fields)
    }

    fn deserialize(serial: impl SerialFormat) -> Result<Self, SchemaError> {
        use progenitor::macros::lookup_auto;
        
        Ok(Self {
            name: lookup_auto!(serial, "name"),
        })
    }

    fn serialize(&self, serial: &mut impl SerialFormat) -> Result<SerialValue, SchemaError> {
        serial.write(self.name.clone().try_into()?)?;

        serial.flush()
    }
}
// end gen

pub struct HelloHandler {
    store: PersistentStore<Greeting, InMemoryPersistenceDriver>
}

#[async_trait]
impl SyncHandler for HelloHandler {
    fn new() -> Self {
        Self {
            store: PersistentStore::new(
                Greeting::schema(),
                InMemoryPersistenceDriver::new()
            )
        }
    }

    async fn handle_sync(&self, req: Request) -> Result<Response, CommError> {
        let who = ClientInfo::deserialize(JsonSerial::from(req.payload))?;

        let greeting = Greeting {
            message: format!("hi, {}", who.name)
        };

        Ok(Response::Success(greeting.serialize(&mut JsonSerial::new_writer())?))
    }
}

// xxx gen

pub struct Router {
    hello: HelloHandler,
}

#[async_trait]
impl SyncHandler for Router {
    fn new() -> Self {
        Self {
            hello: HelloHandler::new()
        }
    }

    async fn handle_sync(&self, req: Request) -> Result<Response, CommError> {
        match req.route.path {
            _ => self.hello.handle_sync(req).await,
        }
    }
}

// end gen

#[tokio::main]
async fn main() {
    let server = Server::<Router, Http1CommunicationDriver>::new();

    server.start().await;
}
