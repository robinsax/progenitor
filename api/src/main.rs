#[macro_use]
extern crate macro_rules_attribute;

use std::collections::HashMap;

use tokio;

use progenitor::ext::{Http1Comm, JsonSerial};
use progenitor::{
    Type, Server, StreamSerial, Request, Response, EffectError, State, EffectExecutor,
    EnvConfig, SerialError, SerialReader, SerialWriter, DirectSerial, FromEnvConfig,
    InitError, HandlerError, Value, make_sequence_effect_fn
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
async fn greet_effect<'ef>(state: &'ef State) -> Result<(), EffectError> {
    let client = state.get::<ClientInfo>("client")?;

    let greeting = ServerOpinion {
        message: format!("hi, {}", client.name)
    };

    state.set::<ServerOpinion>("greeting", greeting)?;

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
    let greeting = state.get::<ServerOpinion>("greeting")?;

    let resp_data = greeting.clone().serialize::<JsonSerial>()?;

    state.set("resp", Response::new(resp_data))?;

    Ok(())
}

make_sequence_effect_fn!(greeting_flow_effect, vec!["req->client", "greet", "resp<-greeting"]);
make_sequence_effect_fn!(poke_flow_effect, vec!["req->client", "hate", "resp<-greeting"]);


#[apply(progenitor::handler_fn)]
async fn root_handler(req: Request) -> Result<Response, HandlerError> {
    let state = State::new();

    let path: String = req.route_ref().path_ref().into();
    state.set("req", req)?;

    let mut executor = EffectExecutor::new();

    executor.register("req->client", load_client_from_req_effect)?;
    executor.register("resp<-greeting", write_greeting_to_resp_effect)?;
    executor.register("greet", greet_effect)?;
    executor.register("hate", hate_effect)?;
    executor.register("greet_flow", greeting_flow_effect)?;
    executor.register("poke_flow_effect", poke_flow_effect)?;

    match path.as_str() {
        "/greet" => executor.execute("greet_flow", &state).await?,
        "/poke" => executor.execute("poke_flow_effect", &state).await?,
        _ => return Err(HandlerError::Interal),
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
