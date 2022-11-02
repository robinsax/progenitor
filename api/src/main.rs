// Currently just a testbed for framework constructs.
#[macro_use]
extern crate macro_rules_attribute;

use std::collections::HashMap;

use tokio;

use progenitor::ext::{Http1Comm, JsonSerial, MemStore};
use progenitor::{
    Type, Server, StreamSerial, Request, Response, EffectError, State, EffectExecutor,
    EnvConfig, SerialError, SerialReader, SerialWriter, DirectSerial, FromEnvConfig,
    InitError, HandlerError, Value, make_flow_effect_fn, Store
};

#[derive(Clone)]
pub struct ServerOpinion {
    message: String
}

impl StreamSerial for ServerOpinion {
    fn schema() -> Type {
        let mut fields = HashMap::new();

        fields.insert("message".into(), Type::String);

        Type::Map(fields)
    }

    fn stream_deserialize(serial: &mut impl SerialReader) -> Result<Self, SerialError> {
        use progenitor::lookup_auto;
        
        Ok(Self {
            message: lookup_auto!(serial, "message"),
        })
    }

    fn stream_serialize(self, serial: &mut impl SerialWriter) -> Result<(), SerialError> {
        let mut fields = HashMap::new();

        fields.insert("message".into(), self.message.clone().try_into()?);

        serial.write(Value::Map(fields))
    }
}

#[derive(Clone)]
pub struct ClientInfo {
    name: String
}

impl StreamSerial for ClientInfo {
    fn schema() -> Type {
        let mut fields = HashMap::new();

        fields.insert("name".into(), Type::String);

        Type::Map(fields)
    }

    fn stream_deserialize(serial: &mut impl SerialReader) -> Result<Self, SerialError> {
        use progenitor::lookup_auto;
        
        Ok(Self {
            name: lookup_auto!(serial, "name"),
        })
    }

    fn stream_serialize(self, serial: &mut impl SerialWriter) -> Result<(), SerialError> {
        let mut fields = HashMap::new();

        fields.insert("name".into(), self.name.clone().try_into()?);

        serial.write(Value::Map(fields))
    }
}

#[apply(progenitor::effect_fn)]
async fn include_store_effect<'ef>(state: &'ef State) -> Result<(), EffectError> {
    let store = Store::<ClientInfo, MemStore>::try_from_config(state.get::<EnvConfig>("env")?.clone())?;

    state.set("store", store)?;

    Ok(())
}

#[apply(progenitor::effect_fn)]
async fn track_client_effect<'ef>(state: &'ef State) -> Result<(), EffectError> {
    // TODO: Having to know the driver type for this is no bueno.
    let store = state.get::<Store<ClientInfo, MemStore>>("store")?.clone();

    let client = {
        state.get::<ClientInfo>("client")?.clone()
    };

    store.put(client).await?;

    Ok(())
}

#[apply(progenitor::effect_fn)]
async fn greet_effect<'ef>(state: &'ef State) -> Result<(), EffectError> {
    let client = state.get::<ClientInfo>("client")?;

    let greeting = ServerOpinion {
        message: format!("hi, {}", client.name)
    };

    state.set("greeting", greeting)?;

    Ok(())
}

#[apply(progenitor::effect_fn)]
async fn hate_effect<'ef>(state: &'ef State) -> Result<(), EffectError> {
    let client = state.get::<ClientInfo>("client")?;

    let greeting = ServerOpinion {
        message: format!("you suck, {}!", client.name)
    };

    state.set::<ServerOpinion>("greeting", greeting)?;

    Ok(())
}

#[apply(progenitor::effect_fn)]
async fn load_client_from_req_effect<'ef>(state: &'ef State) -> Result<(), EffectError> {
    let payload_data = state.get::<Request>("req")?.payload_ref().try_clone_buffer()?;

    let client = ClientInfo::deserialize::<JsonSerial>(payload_data)?;

    state.set("client", client)?;

    Ok(())
}

#[apply(progenitor::effect_fn)]
async fn write_greeting_to_resp_effect<'ef>(state: &'ef State) -> Result<(), EffectError> {
    let greeting = state.take::<ServerOpinion>("greeting")?;

    let resp_data = greeting.serialize::<JsonSerial>()?;

    state.set("resp", Response::new(resp_data))?;

    Ok(())
}

// TODO: Think about what happened here...
struct Visits {
    visits: Vec<ClientInfo>
}

impl StreamSerial for Visits {
    fn schema() -> Type {
        Type::List(Box::new(ClientInfo::schema()))
    }

    fn stream_deserialize(reader: &mut impl SerialReader) -> Result<Self, SerialError> {
        let visits_list = reader.lookup("visits")?.elements()?;

        let mut visits: Vec<ClientInfo> = Vec::with_capacity(visits_list.len());
        for mut visit_serial in visits_list {
            let visit = ClientInfo::stream_deserialize(&mut visit_serial)?;

            visits.push(visit);
        }

        Ok(Self { visits })
    }

    fn stream_serialize(self, writer: &mut impl SerialWriter) -> Result<(), SerialError> {
        let mut visits: Vec<Value> = Vec::with_capacity(self.visits.len());
        for visit in self.visits {
            let mut fields = HashMap::new();

            fields.insert("name".into(), visit.name.clone().try_into()?);
    
            visits.push(Value::Map(fields));
        }

        writer.write(Value::List(visits))?;

        Ok(())
    }
}

#[apply(progenitor::effect_fn)]
async fn report_visits<'ef>(state: &'ef State) -> Result<(), EffectError> {
    let store = state.get::<Store<ClientInfo, MemStore>>("store")?.clone();

    let visits = store.query().all().await?;

    let resp_data = Visits{ visits }.serialize::<JsonSerial>()?;

    state.set("resp", Response::new(resp_data))?;

    Ok(())
}

make_flow_effect_fn!(greeting_flow_effect, vec!["req->client", "+store", "track", "greet", "resp<-greeting"]);
make_flow_effect_fn!(poke_flow_effect, vec!["req->client", "+store", "track", "hate", "resp<-greeting"]);
make_flow_effect_fn!(visits_flow_effect, vec!["+store", "visits"]);

#[apply(progenitor::handler_fn)]
async fn root_handler(req: Request) -> Result<Response, HandlerError> {
    let state = State::new();

    // TODO: Weird... maybe handlers take an initial state?
    state.set("env", EnvConfig::new())?;

    let path: String = req.route_ref().path_ref().into();
    state.set("req", req)?;

    let mut executor = EffectExecutor::new();

    executor.register("req->client", load_client_from_req_effect)?;
    executor.register("resp<-greeting", write_greeting_to_resp_effect)?;
    executor.register("+store", include_store_effect)?;
    executor.register("track", track_client_effect)?;
    executor.register("greet", greet_effect)?;
    executor.register("hate", hate_effect)?;
    executor.register("visits", report_visits)?;
    executor.register("greet_flow", greeting_flow_effect)?;
    executor.register("poke_flow", poke_flow_effect)?;
    executor.register("visits_flow", visits_flow_effect)?;

    match path.as_str() {
        "/greet" => executor.execute("greet_flow", &state).await?,
        "/poke" => executor.execute("poke_flow", &state).await?,
        "/visits" => executor.execute("visits_flow", &state).await?,
        _ => return Err(HandlerError::Internal),
    };

    let resp = state.take::<Response>("resp")?;

    Ok(resp)
}

async fn run() -> Result<(), InitError> {
    let env_config = EnvConfig::new();

    let server = Server::<Http1Comm>::try_from_config(env_config)?;

    server.start(root_handler).await.unwrap();

    Ok(())
}

#[tokio::main]
async fn main() {
    run().await.unwrap();
}
