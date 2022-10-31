use uuid::Uuid;

pub type ComponentID = String;

pub fn new_component_id() -> ComponentID {
    Uuid::new_v4().to_string()
}

macro_rules! component {
    ($d: item) => {
        #[derive(Debug, Clone)] // TODO serialize et al
        $d
    }
}

pub(crate) use component;
