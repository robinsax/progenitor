use super::macros::component;
use super::traits::Mutation;
use super::model::{Collection, Model, Field};

component! {
    pub struct ModelCreate {
        pub model: Model,
    }
}

impl ModelCreate {
    pub fn new(model: Model) -> Self {
        Self{ model }
    }
}

impl Mutation<Collection> for ModelCreate {
    fn apply(&self, collection: &Collection) -> Collection {
        let mut update = collection.clone();

        update.models.push(self.model.clone());

        update
    }
}

component! {
    pub struct ModelFieldCreate {
        pub field: Field,
    }
}

impl ModelFieldCreate {
    pub fn new(field: Field) -> Self {
        Self{ field }
    }
}

impl Mutation<Model> for ModelFieldCreate {
    fn apply(&self, model: &Model) -> Model {
        let mut update = model.clone();

        update.fields.push(self.field.clone());

        update
    }
}
