use super::macros::component;

component! {
    pub struct Collection {
        pub name: String,
        pub models: Vec<Model>,
    }
}

component! {
    pub struct Model {
        pub name: String,
        pub fields: Vec<Field>,
    }
}

component! {
    pub struct Field {
        pub name: String,
        pub data_type: DataType,
    }
}

component! {
    pub enum DataType {
        Int{width: u32, signed: bool},
        Float{width: u32},
        String, // + constraints
        Array{inner: Box<DataType>},
        Embedded{model: Model}
    }
}
