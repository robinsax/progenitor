mod resources;
pub mod archetype;

pub mod inst {
    pub use super::resources::*;
}

pub mod ext {
    pub use super::resources::ext::*;
}
