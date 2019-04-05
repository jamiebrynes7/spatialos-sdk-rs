use std::collections::BTreeMap;
use spatialos_sdk::worker::{
    EntityId,
    schema::*,
};

pub struct CustomComponent {
    pub name: String,
    pub count: i32,
    pub targets: Vec<EntityId>,
    pub target_names: BTreeMap<EntityId, String>,
    pub byte_collection: Vec<Vec<u8>>,
    pub nested: NestedType,
}

impl SchemaObjectType for CustomComponent {
    fn from_object(object: &SchemaObject) -> Self {
        Self {
            name: object.field::<String>(0),
            count: object.field::<SchemaSfixed32>(1),
            targets: object.field::<Vec<EntityId>>(2),
            target_names: object.field::<BTreeMap<EntityId, String>>(3),
            byte_collection: object.field::<Vec<Vec<u8>>>(4),
            nested: object.field::<NestedType>(5),
        }
    }
}

pub struct NestedType {
    pub something: Option<bool>,
}

impl SchemaObjectType for NestedType {
    fn from_object(object: &SchemaObject) -> Self {
        Self {
            something: object.field::<Option<bool>>(0),
        }
    }
}

fn main() {}
