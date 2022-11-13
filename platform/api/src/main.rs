// Currently just a testbed for framework constructs.
#[macro_use]
extern crate macro_rules_attribute;

use std::env;
use std::collections::HashMap;
use std::sync::Arc;

use progenitor::{
    InitError, EffectError, Value, Context, Registry,
    effect_fn, archetype_effect, sequence_effect
};
use progenitor::effect::{store_read, store_write, open_store};
use progenitor_server::{Server, Request};
use progenitor_server::effect::{read_req, write_resp};

use progenitor::ext::{JsonSerial, MemStore};
use progenitor_server::ext::Http1Comm;

#[apply(effect_fn)]
async fn greet<'ef>(context: &'ef mut Context) -> Result<(), EffectError> {
    let client = context.get::<Value>("client")?;

    let greeting = Value::Map(HashMap::from([
        ("message".into(), Value::Str(format!("hi, {}", String::try_from(client.lookup("name")?)?)))
    ]));

    drop(client);

    context.set("greeting", greeting)?;

    Ok(())
}

#[apply(effect_fn)]
async fn poke<'ef>(context: &'ef mut Context) -> Result<(), EffectError> {
    let client = context.get::<Value>("client")?;

    let greeting = Value::Map(HashMap::from([
        ("message".into(), Value::Str(format!("hi, {}", String::try_from(client.lookup("name")?)?)))
    ]));

    context.set("greeting", greeting)?;

    Ok(())
}

archetype_effect!(track_client, "store_write", Value::map_from([
    ("from_state".into(), Value::str_from("client")),
    ("to_store".into(), Value::str_from("visits"))
]));

archetype_effect!(store_read_visits, "store_read", Value::map_from([
    ("to_state".into(), Value::str_from("visits")),
    ("from_store".into(), Value::str_from("visits"))
]));

archetype_effect!(read_req_client, "read_req", Value::map_from([
    ("format".into(), Value::str_from("json")),
    ("to_state".into(), Value::str_from("client")),
    ("schema".into(), Value::map_from([
        ("name".into(), Value::str_from("james"))
    ]))
]));

archetype_effect!(open_visits_store, "open_store", Value::map_from([
    ("driver".into(), Value::str_from("memory")),
    ("name".into(), Value::str_from("visits")),
    ("schema".into(), Value::map_from([
        ("name".into(), Value::str_from("james"))
    ]))
]));

archetype_effect!(write_resp_greeting, "write_resp", Value::map_from([
    ("format".into(), Value::str_from("json")),
    ("from_state".into(), Value::str_from("greeting"))
]));

archetype_effect!(write_resp_visits, "write_resp", Value::map_from([
    ("format".into(), Value::str_from("json")),
    ("from_state".into(), Value::str_from("visits"))
]));

sequence_effect!(prep_client, vec![
    "open_visits_store",
    "read_req_client",
    "track_client"
]);

sequence_effect!(greet_flow, vec![
    "prep_client",
    "greet",
    "write_resp_greeting"
]);

sequence_effect!(poke_flow, vec![
    "prep_client",
    "poke",
    "write_resp_greeting"
]);

sequence_effect!(visits_flow, vec![
    "open_visits_store",
    "store_read_visits",
    "write_resp_visits"
]);

#[apply(effect_fn)]
async fn entrypoint<'ef>(context: &'ef mut Context) -> Result<(), EffectError> {
    let path = context.get::<Request>("req")?.route().path().to_owned();

    match path.as_str() {
        "/greet" => context.execute("greet_flow".into(), None).await,
        "/poke" => context.execute("poke_flow".into(), None).await,
        "/visits" => context.execute("visits_flow".into(), None).await,
        _ => return Err(EffectError::Missing(path)),
    }
}

use simple_logger;    
fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let registry = Arc::new(Registry::new(
        vec![
            ("store_read", store_read),
            ("store_write", store_write),
            ("open_store", open_store),
            ("read_req", read_req),
            ("write_resp", write_resp),
            ("read_req_client", read_req_client),
            ("open_visits_store", open_visits_store),
            ("write_resp_greeting", write_resp_greeting),
            ("write_resp_visits", write_resp_visits),
            ("track_client", track_client),
            ("prep_client", prep_client),
            ("store_read_visits", store_read_visits),
            ("greet_flow", greet_flow),
            ("poke_flow", poke_flow),
            ("visits_flow", visits_flow),
            ("poke", poke),
            ("greet", greet),
            ("main", entrypoint)
        ],
        vec![
            ("memory", Box::new(|_: &Registry, name: String| Box::new(MemStore::new(name.as_str()))))
        ],
        vec![
            ("json", Box::new(JsonSerial::new()))
        ],
        Box::new(|key: String| {
            let look_key = key.to_uppercase();
            env::var(look_key).or_else(|_| Err(InitError::Config(format!("invalid key {}", key).into())))
        })
    ));

    let server = Server::<Http1Comm>::new(registry).unwrap();

    server.start().unwrap();
}
