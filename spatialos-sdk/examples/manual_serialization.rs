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
}

impl SchemaObjectType for CustomComponent {
    fn from_schema_object(schema_object: &SchemaObject) -> Self {
        Self {
            name: schema_object.field::<String>(0),
            count: schema_object.field::<SchemaSfixed32>(1),
            targets: schema_object.field::<Vec<EntityId>>(2),
            target_names: schema_object.field::<BTreeMap<EntityId, String>>(3),
            byte_collection: schema_object.field::<Vec<Vec<u8>>>(4),
        }
    }
}

fn main() {}
