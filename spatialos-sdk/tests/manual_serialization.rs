//! Example code for the following schema defintions:
//!
//! ```schema
//! component CustomComponent {
//!     id = 1234;
//!
//!     option<string> name = 1;
//!     int32 count = 2;
//!     list<EntityId> targets = 3;
//!     list<bytes> byte_collection = 4;
//!     option<uint32> id = 5;
//!     NestedType nested = 6;
//!     list<bool> events = 7;
//!
//!     event NestedType nested_event;
//!     event CoolEvent cool_event;
//! }
//!
//! type NestedType {
//!     string name = 1;
//! }
//!
//! type CoolEvent {
//!     option<bytes> some_data = 1;
//! }
//! ```

use spatialos_sdk::worker::{
    component::*,
    schema::{self, *},
    EntityId,
};
use std::collections::BTreeMap;

#[derive(Debug, Default)]
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

    pub nested_event: Vec<NestedType>,
    pub cool_event: Vec<CoolEvent>,
}

impl Update for CustomComponentUpdate {
    type Component = CustomComponent;

    fn from_update(update: &schema::ComponentUpdate) -> Self {
        Self {
            name: update.field::<Option<String>>(0),
            count: update.field::<SchemaSfixed32>(1),
            targets: update.field_array::<EntityId>(2),
            target_names: update.field::<BTreeMap<EntityId, String>>(3),
            byte_collection: update.field::<Vec<Vec<u8>>>(4),
            id: update.field::<Option<SchemaUint32>>(5),
            nested: update.field::<NestedType>(6),

            nested_event: update.event::<NestedType>(1),
            cool_event: update.event::<CoolEvent>(2),
        }
    }

    fn into_update(&self, update: &mut schema::ComponentUpdate) {
        if let Some(name) = &self.name {
            update.add_field::<Option<String>>(0, name);
        }

        if let Some(count) = &self.count {
            update.add_field::<SchemaSfixed32>(1, count);
        }

        if let Some(target) = &self.targets {
            update.add_field_array::<EntityId>(2, target);
        }

        if let Some(target_names) = &self.target_names {
            update.add_field::<BTreeMap<EntityId, String>>(3, target_names);
        }

        if let Some(byte_collection) = &self.byte_collection {
            update.add_field::<Vec<Vec<u8>>>(4, byte_collection);
        }

        if let Some(id) = &self.id {
            update.add_field::<Option<SchemaUint32>>(5, id);
        }

        if let Some(nested) = &self.nested {
            update.add_field::<NestedType>(6, nested);
        }

        if !self.nested_event.is_empty() {
            update.add_event(1, &self.nested_event);
        }

        if !self.cool_event.is_empty() {
            update.add_event(2, &self.cool_event);
        }
    }
}

#[derive(Debug, Default)]
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

#[derive(Debug, Default)]
pub struct CoolEvent {
    pub some_data: Option<Vec<u8>>,
}

impl SchemaObjectType for CoolEvent {
    fn from_object(_object: &schema::Object) -> Self {
        unimplemented!()
    }

    fn into_object(&self, _object: &mut schema::Object) {
        unimplemented!();
    }
}

fn main() {}
