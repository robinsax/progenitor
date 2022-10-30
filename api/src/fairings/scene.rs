use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use rocket::fairing;
use serde::{Serialize, Deserialize};

use progenitor::apps::PersistentStore;
use progenitor::archetype::{DataType, LiteralValue};
use progenitor::ext::InMemoryPersistentStoreBackend;

#[derive(Clone, Serialize, Deserialize)]
pub struct Foo {
    pub bar: i32,
    pub baz: String
}

// TODO proc_macro all this in core obviously, w/ byte streams

fn foo_schema() -> HashMap<String, DataType> {
    let mut foo_fields = HashMap::new();
    foo_fields.insert("bar".to_string(), DataType::Int32);
    foo_fields.insert("baz".to_string(), DataType::String);
    
    foo_fields
}


impl From<LiteralValue> for Foo {
    fn from(val: LiteralValue) -> Self {
        let bar = match val.lookup("bar").unwrap() {
            LiteralValue::Int32{ value } => value,
            _ => panic!("TODO handling pattern"),
        };
        let baz = match val.lookup("baz").unwrap() {
            LiteralValue::String{ value } => value,
            _ => panic!("TODO handling pattern"),
        };
        
        Self{ bar, baz }
    }
}

impl From<Foo> for LiteralValue {
    fn from(obj: Foo) -> Self {
        let mut schema: HashMap<String, LiteralValue> = HashMap::new();

        schema.insert("bar".into(), LiteralValue::Int32 { value: obj.bar });
        schema.insert("baz".into(), LiteralValue::String { value: obj.baz });

        LiteralValue::Object{ value: schema, type_schema: foo_schema() }
    }
}

pub struct SceneState { // TODO actually turn into a scene once core has it
    pub foo_type: DataType,
    pub foo_store: PersistentStore<Foo>,
}

impl SceneState {
    fn new() -> Self {

        Self{
            foo_store: PersistentStore::new(DataType::Object{ schema: foo_schema() }, Box::new(InMemoryPersistentStoreBackend::new())),
            foo_type: DataType::Object{ schema: foo_schema() }
        }
    }
}

pub fn scene_stage() -> fairing::AdHoc {
    fairing::AdHoc::on_ignite("endpoints", |rocket| async {
        rocket
            .manage(Arc::new(Mutex::new(SceneState::new())))
    })
}
