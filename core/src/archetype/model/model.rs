use super::super::{
    component::{component, ComponentID},
    mutation::ComponentMutationError,
    primitives::DataType
};

component! {
    pub struct Model {
        pub collections: Vec<Collection>,
    }
}

impl Model {
    pub fn get_collection_mut(&mut self, id: &ComponentID) -> Result<&mut Collection, ComponentMutationError> {
        self.collections.as_mut_slice().into_iter().find(
            |check| check.component_id == *id
        ).ok_or_else(|| ComponentMutationError::MissingTarget{
            component_type: "collection".to_string(),
            target_id: id.clone()
        })
    }
}

component! {
    pub struct Collection {
        pub component_id: ComponentID,
        pub name: String,
        pub objects: Vec<Object>,
    }
}

impl Collection {
    pub fn get_object_mut(&mut self, id: &ComponentID) -> Result<&mut Object, ComponentMutationError> {
        self.objects.as_mut_slice().into_iter().find(
            |check| check.component_id == *id
        ).ok_or_else(|| ComponentMutationError::MissingTarget{
            component_type: "object".to_string(),
            target_id: id.clone()
        })
    }
}

component! {
    pub struct Object {
        pub component_id: ComponentID,
        pub name: String,
        pub fields: Vec<Field>,
    }
}

impl Object {
    pub fn get_field_named(&self, name: &String) -> Option<&Field> {
        self.fields.as_slice().into_iter().find(
            |check| check.name == *name
        )
    }
}

component! {
    pub struct Field {
        pub component_id: ComponentID,
        pub name: String,
        pub data_type: DataType,
    }
}
