use spatialos_sdk::worker::{schema::*, EntityId};
use std::collections::BTreeMap;

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
            targets: object.field_array::<EntityId>(2),
            target_names: object.field::<BTreeMap<EntityId, String>>(3),
            byte_collection: object.field::<Vec<Vec<u8>>>(4),
            nested: object.field::<NestedType>(5),
        }
    }

    fn into_object<'a>(&'a self, object: &mut SchemaObject<'a>) {
        object.add_field::<String>(0, &self.name);
        object.add_field::<SchemaSfixed32>(1, &self.count);
        object.add_field_array::<EntityId>(2, &self.targets);
        object.add_field::<BTreeMap<EntityId, String>>(3, &self.target_names);
        object.add_field::<Vec<Vec<u8>>>(4, &self.byte_collection);
        object.add_field::<NestedType>(5, &self.nested);
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

    fn into_object(&self, object: &mut SchemaObject) {
        object.add_field::<Option<bool>>(0, &self.something);
    }
}

fn main() {}
