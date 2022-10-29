mod traits;
mod model;
mod model_mutations;

pub use traits::{Mutation, Stored};
pub use model::{Collection, Model, Field, DataType};
pub use model_mutations::{ModelCreate, ModelFieldCreate};

mod macros {
    pub use serde::{Serialize, Deserialize};

    macro_rules! component {
        ($d: item) => {
            #[derive($crate::archetype::macros::Serialize, $crate::archetype::macros::Deserialize, Debug, Clone)]
            $d
        }
    }

    pub(crate) use component;
}
