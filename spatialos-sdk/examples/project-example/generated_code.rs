use spatialos_sdk::worker::core::schema;
use spatialos_sdk::worker::core::schema::SchemaField;
use spatialos_sdk::worker::{ComponentMetaclass, ComponentUpdate, ComponentUpdateSerializer, ComponentData};

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
}

impl ComponentUpdate<Example> for ExampleUpdate {
    fn to_data(self) -> ExampleData {
        ExampleData {
            x: self.x.unwrap()
        }
    }
}

impl ComponentUpdateSerializer<ExampleUpdate> for ExampleUpdate {
    fn serialize(&self) -> schema::SchemaComponentUpdate {
        let mut update = schema::SchemaComponentUpdate::new(200);
        let mut fields = update.fields_mut();
        if let Some(field_value) = self.x { fields.field::<f32>(0).add(field_value); }
        update
    }

    fn deserialize(update: schema::SchemaComponentUpdate) -> ExampleUpdate {
        let fields = update.fields();
        ExampleUpdate { x: fields.field::<f32>(0).get() }
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