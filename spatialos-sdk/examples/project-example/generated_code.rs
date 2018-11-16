use spatialos_sdk::worker::internal::schema::{self, SchemaField};
use spatialos_sdk::worker::{ComponentMetaclass, ComponentUpdate, ComponentData, ComponentVtable, ComponentId};

// CommandData.
#[derive(Debug)]
pub struct CommandData {
    value: i32
}

// ExampleData (implicit).
#[derive(Debug)]
pub struct ExampleData {
    x: f32
}

// Example.
#[allow(dead_code)]
#[derive(Debug)]
pub struct Example;

#[derive(Debug)]
pub struct ExampleUpdate {
    x: Option<f32>
}

impl ComponentMetaclass for Example {
    type Data = ExampleData;
    type Update = ExampleUpdate;
    type CommandRequest = ExampleCommandRequest;
    type CommandResponse = ExampleCommandResponse;
    fn component_id() -> ComponentId { 1000 }
}

impl ComponentUpdate<Example> for ExampleUpdate {
    fn to_data(self) -> ExampleData {
        ExampleData {
            x: self.x.unwrap()
        }
    }

    fn merge(&mut self, update: ExampleUpdate) {
        if let Some(x) = update.x { self.x = Some(x); }
    }
}

impl ComponentData<Example> for ExampleData {
    fn to_update(self) -> ExampleUpdate {
        ExampleUpdate {
            x: Some(self.x)
        }
    }

    fn merge(&mut self, update: ExampleUpdate) {
        if let Some(x) = update.x { self.x = x; }
    }
}

pub enum ExampleCommandRequest {
    TestCommand(CommandData), // index = 1
}

pub enum ExampleCommandResponse {
    TestCommand(CommandData), // index = 1
}

// TODO: #[derive(...)] this.
impl ComponentVtable<Example> for Example {
    fn serialize_data(data: &ExampleData) -> schema::SchemaComponentData {
        let mut serialized_data = schema::SchemaComponentData::new(Example::component_id());
        let mut fields = serialized_data.fields_mut();
        fields.field::<f32>(0).add(data.x);
        serialized_data
    }

    fn deserialize_data(data: &schema::SchemaComponentData) -> Option<ExampleData> {
        let fields = data.fields();
        Some(ExampleData { x: fields.field::<f32>(1).get_or_default() })
    }

    fn serialize_update(update: &ExampleUpdate) -> schema::SchemaComponentUpdate {
        let mut serialized_update = schema::SchemaComponentUpdate::new(Example::component_id());
        let mut fields = serialized_update.fields_mut();
        if let Some(field_value) = update.x { fields.field::<f32>(1).add(field_value); }
        serialized_update
    }

    fn deserialize_update(update: &schema::SchemaComponentUpdate) -> Option<ExampleUpdate> {
        let fields = update.fields();
        Some(ExampleUpdate { x: fields.field::<f32>(1).get() })
    }

    fn serialize_command_request(request: &ExampleCommandRequest) -> schema::SchemaCommandRequest {
        let command_index = match request {
            ExampleCommandRequest::TestCommand(_) => 1
        };
        let mut serialized_request = schema::SchemaCommandRequest::new(Example::component_id(), command_index);
        let mut object = serialized_request.object_mut();
        match request {
            ExampleCommandRequest::TestCommand(data) => {
                object.field::<i32>(1).add(data.value)
            }
        }
        serialized_request
    }

    fn deserialize_command_request(request: &schema::SchemaCommandRequest) -> Option<ExampleCommandRequest> {
        let object = request.object();
        match request.command_index() {
            1 => {
                Some(ExampleCommandRequest::TestCommand(CommandData { value: object.field::<i32>(1).get_or_default() }))
            },
            _ => None
        }
    }

    fn serialize_command_response(response: &ExampleCommandResponse) -> schema::SchemaCommandResponse {
        let command_index = match response {
            ExampleCommandResponse::TestCommand(_) => 1
        };
        let mut serialized_response = schema::SchemaCommandResponse::new(Example::component_id(), command_index);
        let mut object = serialized_response.object_mut();
        match response {
            ExampleCommandResponse::TestCommand(data) => {
                object.field::<i32>(1).add(data.value)
            }
        }
        serialized_response
    }

    fn deserialize_command_response(response: &schema::SchemaCommandResponse) -> Option<ExampleCommandResponse> {
        let object = response.object();
        match response.command_index() {
            1 => {
                Some(ExampleCommandResponse::TestCommand(CommandData { value: object.field::<i32>(1).get_or_default() }))
            },
            _ => None
        }
    }
}
