// TODO express vs primitives probably (hoist)

use super::super::{
    component::{ComponentID, component, new_component_id},
    mutation::{ComponentMutation, ComponentMutationError},
    primitives::DataType
};
use super::model::{Model, Field, Object};

component! {
    pub struct ObjectCreate {
        collection_id: ComponentID,
        object_name: String,
    }
}

impl ObjectCreate {
    pub fn new(collection_id: ComponentID, object_name: String) -> Self {
        Self{ object_name, collection_id }
    }
}

impl ComponentMutation for ObjectCreate {
    fn apply(&self, model: &mut Model) -> Result<(), ComponentMutationError> {
        let collection = model.get_collection_mut(&self.collection_id)?;

        collection.objects.push(Object{
            component_id: new_component_id(),
            name: self.object_name.clone(),
            fields: Vec::new(),
        });

        Ok(())
    }
}

component! {
    pub struct ObjectFieldCreate {
        collection_id: String,
        object_id: String,
        field_name: String,
        field_type: DataType,
    }
}

impl ObjectFieldCreate {
    pub fn new(
        collection_id: ComponentID, object_id: ComponentID, field_name: String, field_type: DataType
    ) -> Self {
        Self{ collection_id, object_id, field_name, field_type }
    }
}

impl ComponentMutation for ObjectFieldCreate {
    fn apply(&self, model: &mut Model) -> Result<(), ComponentMutationError> {
        let collection = model.get_collection_mut(&self.collection_id)?;

        let object = collection.get_object_mut(&self.object_id)?;

        object.fields.push(Field{
            component_id: new_component_id(),
            name: self.field_name.clone(),
            data_type: self.field_type.clone()
        });

        Ok(())
    }
}
