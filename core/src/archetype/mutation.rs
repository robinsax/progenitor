use super::model::Model; // TODO no, more generic
use super::component::ComponentID;

pub enum ComponentMutationError {
    MissingTarget{component_type: String, target_id: ComponentID},
    TODO
}

pub trait ComponentMutation {
    fn apply(&self, model: &mut Model) -> Result<(), ComponentMutationError>;
}

pub struct MutationResolver;

impl MutationResolver {
    pub fn new() -> Self {
        Self{}
    }

    pub fn resolve(model: &Model, mutations: Vec<Box<dyn ComponentMutation>>) -> Result<Model, ComponentMutationError> {
        let mut update = model.clone();
        for mutation in mutations.as_slice() {
            mutation.apply(&mut update)?;
        }
        
        Ok(update)
    }
}
