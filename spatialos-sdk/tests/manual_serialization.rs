use spatialos_sdk::worker::{
    component::*,
    schema::{self, *},
    EntityId,
};
use std::collections::BTreeMap;

pub struct CustomComponent {
    pub name: Option<String>,
    pub count: i32,
    pub targets: Vec<EntityId>,
    pub target_names: BTreeMap<EntityId, String>,
    pub byte_collection: Vec<Vec<u8>>,
    pub id: Option<u32>,
    pub nested: NestedType,
}

impl SchemaObjectType for CustomComponent {
    fn from_object(object: &schema::Object) -> Self {
        Self {
            name: object.field::<Option<String>>(0),
            count: object.field::<SchemaSfixed32>(1),
            targets: object.field_array::<EntityId>(2),
            target_names: object.field::<BTreeMap<EntityId, String>>(3),
            byte_collection: object.field::<Vec<Vec<u8>>>(4),
            id: object.field::<Option<SchemaUint32>>(5),
            nested: object.field::<NestedType>(6),
        }
    }

    fn into_object(&self, object: &mut schema::Object) {
        object.add_field::<Option<String>>(0, &self.name);
        object.add_field::<SchemaSfixed32>(1, &self.count);
        object.add_field_array::<EntityId>(2, &self.targets);
        object.add_field::<BTreeMap<EntityId, String>>(3, &self.target_names);
        object.add_field::<Vec<Vec<u8>>>(4, &self.byte_collection);
        object.add_field::<Option<SchemaUint32>>(5, &self.id);
        object.add_field::<NestedType>(6, &self.nested);
    }
}

impl Component for CustomComponent {
    const ID: ComponentId = 1234;
    type Update = CustomComponentUpdate;
}

#[allow(clippy::option_option)]
pub struct CustomComponentUpdate {
    pub name: Option<Option<String>>,
    pub count: Option<i32>,
    pub targets: Option<Vec<EntityId>>,
    pub target_names: Option<BTreeMap<EntityId, String>>,
    pub byte_collection: Option<Vec<Vec<u8>>>,
    pub id: Option<Option<u32>>,
    pub nested: Option<NestedType>,
}

impl Update for CustomComponentUpdate {
    type Component = CustomComponent;

    fn from_update(update: &schema::ComponentUpdate) -> Self {
        let mut result = Self {
            name: update.field::<Option<String>>(0),
            count: update.field::<SchemaSfixed32>(1),
            targets: update.field::<Vec<EntityId>>(2),
            target_names: update.field::<BTreeMap<EntityId, String>>(3),
            byte_collection: update.field::<Vec<Vec<u8>>>(4),
            id: update.field::<Option<SchemaUint32>>(5),
            nested: update.field::<NestedType>(6),
        };

        for cleared in update.cleared() {
            match cleared {
                0 => {
                    result.name = Some(Default::default());
                }
                2 => {
                    result.targets = Some(Default::default());
                }
                3 => {
                    result.target_names = Some(Default::default());
                }
                4 => {
                    result.byte_collection = Some(Default::default());
                }
                5 => {
                    result.id = Some(Default::default());
                }

                _ => {}
            }
        }

        result
    }

    fn into_update(&self, _update: &mut schema::ComponentUpdate) {
        unimplemented!();
    }
}

pub struct NestedType {
    pub name: String,
}

impl SchemaObjectType for NestedType {
    fn from_object(_object: &schema::Object) -> Self {
        unimplemented!()
    }

    fn into_object(&self, _object: &mut schema::Object) {
        unimplemented!();
    }
}

fn main() {}
