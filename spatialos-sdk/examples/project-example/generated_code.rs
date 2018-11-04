use spatialos_sdk::worker::core::schema;
use spatialos_sdk::worker::core::schema::SchemaField;
use spatialos_sdk::worker::{ComponentMetaclass, ComponentUpdate, ComponentData, ComponentVtable, ComponentId};

#[allow(dead_code)]
pub struct Example;

pub struct ExampleData {
    x: f32
}

pub struct ExampleUpdate {
    x: Option<f32>
}

impl ComponentMetaclass for Example {
    type Data = ExampleData;
    type Update = ExampleUpdate;
    fn component_id() -> ComponentId { 1000 }
}

impl ComponentUpdate<Example> for ExampleUpdate {
    fn to_data(self) -> ExampleData {
        ExampleData {
            x: self.x.unwrap()
        }
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

impl ComponentVtable<Example> for Example {
    fn serialize_update(update: &ExampleUpdate) -> schema::SchemaComponentUpdate {
        let mut serialized_update = schema::SchemaComponentUpdate::new(Example::component_id());
        let mut fields = serialized_update.fields_mut();
        if let Some(field_value) = update.x { fields.field::<f32>(0).add(field_value); }
        serialized_update
    }

    fn deserialize_update(update: &schema::SchemaComponentUpdate) -> ExampleUpdate {
        let fields = update.fields();
        ExampleUpdate { x: fields.field::<f32>(0).get() }
    }

    fn serialize_data(data: &ExampleData) -> schema::SchemaComponentData {
        let mut serialized_data = schema::SchemaComponentData::new(Example::component_id());
        let mut fields = serialized_data.fields_mut();
        fields.field::<f32>(0).add(data.x);
        serialized_data
    }

    fn deserialize_data(data: &schema::SchemaComponentData) -> ExampleData {
        let fields = data.fields();
        ExampleData { x: fields.field::<f32>(0).get_or_default() }
    }
}