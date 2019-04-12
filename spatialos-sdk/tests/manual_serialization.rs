use spatialos_sdk::worker::{component::*, schema::*, EntityId};
use std::collections::BTreeMap;

pub struct CustomComponent {
    pub name: String,
    pub count: i32,
    pub targets: Vec<EntityId>,
    pub target_names: BTreeMap<EntityId, String>,
    pub byte_collection: Vec<Vec<u8>>,
}

impl SchemaObjectType for CustomComponent {
    fn from_object(object: &SchemaObject) -> Self {
        Self {
            name: object.field::<String>(0),
            count: object.field::<SchemaSfixed32>(1),
            targets: object.field_array::<EntityId>(2),
            target_names: object.field::<BTreeMap<EntityId, String>>(3),
            byte_collection: object.field::<Vec<Vec<u8>>>(4),
        }
    }

    fn into_object<'a>(&'a self, object: &mut SchemaObject<'a>) {
        object.add_field::<String>(0, &self.name);
        object.add_field::<SchemaSfixed32>(1, &self.count);
        object.add_field_array::<EntityId>(2, &self.targets);
        object.add_field::<BTreeMap<EntityId, String>>(3, &self.target_names);
        object.add_field::<Vec<Vec<u8>>>(4, &self.byte_collection);
    }
}

impl Component for CustomComponent {
    type Update = CustomComponentUpdate;
    type CommandRequest = CustomComponentRequest;
    type CommandResponse = CustomComponentResponse;

    const ID: ComponentId = 1234;

    fn from_update(_update: &SchemaComponentUpdate) -> Result<Self::Update, String> {
        unimplemented!()
    }

    fn from_request(_request: &SchemaCommandRequest) -> Result<Self::CommandRequest, String> {
        unimplemented!()
    }

    fn from_response(_response: &SchemaCommandResponse) -> Result<Self::CommandResponse, String> {
        unimplemented!()
    }

    fn to_update(_update: &Self::Update) -> Result<SchemaComponentUpdate, String> {
        unimplemented!()
    }

    fn to_request(_request: &Self::CommandRequest) -> Result<SchemaCommandRequest, String> {
        unimplemented!()
    }

    fn to_response(_response: &Self::CommandResponse) -> Result<SchemaCommandResponse, String> {
        unimplemented!()
    }

    fn get_request_command_index(_request: &Self::CommandRequest) -> u32 {
        unimplemented!()
    }

    fn get_response_command_index(_response: &Self::CommandResponse) -> u32 {
        unimplemented!()
    }
}

pub struct CustomComponentUpdate {
    pub name: Option<String>,
    pub count: Option<i32>,
    pub targets: Option<Vec<EntityId>>,
    pub target_names: Option<BTreeMap<EntityId, String>>,
    pub byte_collection: Option<Vec<Vec<u8>>>,
}

impl ComponentUpdate<CustomComponent> for CustomComponentUpdate {
    fn merge(&mut self, update: Self) {
        unimplemented!();
    }
}

impl SchemaObjectType for CustomComponentUpdate {
    fn from_object(object: &SchemaObject) -> Self {
        unimplemented!()
        // Self {
        //     name: object.field::<Option<String>>(0),
        //     count: object.field::<Option<SchemaSfixed32>>(1),
        //     targets: object.field_array::<Option<EntityId>>(2),
        //     target_names: object.field::<Option<BTreeMap<EntityId, String>>>(3),
        //     byte_collection: object.field::<Option<Vec<Vec<u8>>>>(4),
        // }
    }

    fn into_object<'a>(&'a self, object: &mut SchemaObject<'a>) {
        unimplemented!();
        // object.add_field::<Option<String>>(0, &self.name);
        // object.add_field::<Option<SchemaSfixed32>>(1, &self.count);
        // object.add_field_array::<Option<EntityId>>(2, &self.targets);
        // object.add_field::<Option<BTreeMap<EntityId, String>>>(3, &self.target_names);
        // object.add_field::<Optoin<Vec<Vec<u8>>>>(4, &self.byte_collection);
    }
}

pub enum CustomComponentRequest {}

pub enum CustomComponentResponse {}

fn main() {}
