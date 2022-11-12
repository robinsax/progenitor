// Currently just a testbed for framework constructs.
#[macro_use]
extern crate macro_rules_attribute;

mod model;

use std::collections::HashMap;

use progenitor::{
    EffectError, State, EffectExecutor, Type, Value, Store, SerialFormat,
    make_flow_effect_fn, effect_fn
};
use progenitor_server::{Server, Request, Response, ServerInitConfig};

use progenitor::ext::{JsonSerial, MemStore};
use progenitor_server::ext::Http1Comm;

// TODO: Explicit drops to prevent deadlock is an interesting problem.

fn message_type() -> Type {
    Type::Map(HashMap::from([
        ("message".into(), Type::String)
    ]))
}

fn client_type() -> Type {
    Type::Map(HashMap::from([
        ("name".into(), Type::String)
    ]))
}

#[apply(effect_fn)]
async fn include_store_effect<'ef>(state: &'ef State) -> Result<(), EffectError> {
    let store = Store::new(message_type(), Box::new(MemStore::new("messages")));

    state.set("store", store).await?;

    Ok(())
}

#[apply(effect_fn)]
async fn track_client_effect<'ef>(state: &'ef State) -> Result<(), EffectError> {
    let client = state.get::<Value>("client").await?.clone();

    let store = state.get::<Store>("store").await?;

    store.put(client).await?;

    Ok(())
}

#[apply(effect_fn)]
async fn greet_effect<'ef>(state: &'ef State) -> Result<(), EffectError> {
    let client = state.get::<Value>("client").await?;

    let greeting = Value::Map(HashMap::from([
        ("message".into(), Value::String(format!("hi, {}", String::try_from(client.lookup("name")?)?)))
    ]));

    drop(client);

    state.set("greeting", greeting).await?;

    Ok(())
}

#[apply(effect_fn)]
async fn hate_effect<'ef>(state: &'ef State) -> Result<(), EffectError> {
    let client = state.get::<Value>("client").await?;

    let greeting = Value::Map(HashMap::from([
        ("message".into(), Value::String(format!("hi, {}", String::try_from(client.lookup("name")?)?)))
    ]));

    drop(client);

    state.set("greeting", greeting).await?;

    Ok(())
}

#[apply(effect_fn)]
async fn load_client_from_req_effect<'ef>(state: &'ef State) -> Result<(), EffectError> {
    let req = state.get::<Request>("req").await?;

    let client = JsonSerial::parse(req.payload().clone())?;

    client_type().validate(&client)?;

    drop(req);

    state.set("client", client).await?;

    Ok(())
}

#[apply(effect_fn)]
async fn write_greeting_to_resp_effect<'ef>(state: &'ef State) -> Result<(), EffectError> {
    let greeting = state.get::<Value>("greeting").await?;

    let resp_data = JsonSerial::write(&greeting)?;

    drop(greeting);

    state.set("resp", Response::new(resp_data)).await?;

    Ok(())
}

#[apply(effect_fn)]
async fn report_visits<'ef>(state: &'ef State) -> Result<(), EffectError> {
    let store = state.get::<Store>("store").await?;

    let visit_items = store.query().all().await?;

    let visits = Value::Map(HashMap::from([
        ("visits".into(), Value::List(visit_items))
    ]));

    let resp_data = JsonSerial::write(&visits)?;

    drop(store);

    state.set("resp", Response::new(resp_data)).await?;

    Ok(())
}

make_flow_effect_fn!(greeting_flow_effect, vec!["req->client", "+store", "track", "greet", "resp<-greeting"]);
make_flow_effect_fn!(poke_flow_effect, vec!["req->client", "+store", "track", "hate", "resp<-greeting"]);
make_flow_effect_fn!(visits_flow_effect, vec!["+store", "visits"]);

#[apply(effect_fn)]
async fn root_handler<'ef>(state: &'ef State) -> Result<(), EffectError> {
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

    let path = state.get::<Request>("req").await?.route().path().to_owned();

    match path.as_str() {
        "/greet" => executor.execute("greet_flow", state).await,
        "/poke" => executor.execute("poke_flow", state).await,
        "/visits" => executor.execute("visits_flow", state).await,
        _ => return Err(EffectError::Missing(path)),
    }
}

use simple_logger;    
fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let server = Server::<Http1Comm>::new(
        Box::new(ServerInitConfig::new()),
        root_handler
    ).unwrap();

    server.start().unwrap();
}
