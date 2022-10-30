use uuid::Uuid;
pub use serde::{Serialize, Deserialize};

pub type ComponentID = String;

pub fn new_component_id() -> ComponentID {
    Uuid::new_v4().to_string()
}

macro_rules! component {
    ($d: item) => {
        #[derive($crate::archetype::component::Serialize, $crate::archetype::component::Deserialize, Debug, Clone)]
        $d
    }
}

pub(crate) use component;
