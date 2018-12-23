use spatialos_sdk::worker::internal::schema::{self, SchemaField, SchemaObject};
use spatialos_sdk::worker::{ComponentMetaclass, ComponentUpdate, ComponentData, ComponentVtable, ComponentId, TypeSerializer};
use std::collections::BTreeMap;

use self as generated_code;

/* Enums. */
/* Types. */
/* Components. */ 


mod example {
use spatialos_sdk::worker::internal::schema::{self, SchemaField, SchemaObject};
use spatialos_sdk::worker::{ComponentMetaclass, ComponentUpdate, ComponentData, ComponentVtable, ComponentId, TypeSerializer};
use std::collections::BTreeMap;

use super as generated_code;

/* Enums. */
/* Types. */
#[derive(Debug)]
pub struct CommandData {
    value: i32,
}
impl TypeSerializer<CommandData> for CommandData {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        output.field::<i32>(1).add(&input.value);
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            value: input.field::<i32>(1).get_or_default(),
        })
    }
}

#[derive(Debug)]
pub struct ExampleData {
    x: f32,
}
impl TypeSerializer<ExampleData> for ExampleData {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        output.field::<f32>(1).add(&input.x);
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            x: input.field::<f32>(1).get_or_default(),
        })
    }
}

/* Components. */ 
#[derive(Debug)]
pub struct Example {
    x: f32,
}
impl TypeSerializer<Example> for Example {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        output.field::<f32>(1).add(&input.x);
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            x: input.field::<f32>(1).get_or_default(),
        })
    }
}
impl ComponentData<Example> for generated_code::example::Example {
    fn merge(&mut self, update: generated_code::example::ExampleUpdate) {
        if let Some(value) = update.x { self.x = Some(value); }
    }
}

#[derive(Debug)]
pub struct ExampleUpdate {
    x: Option<f32>,
}
impl TypeSerializer<ExampleUpdate> for ExampleUpdate {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        if let Some(ref value) = input.x {
            output.field::<f32>(1).add(value);
        }
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        let mut output = Self {
            x: None,
        };
        let _field_x = input.field::<f32>(1);
        if _field_x.count() > 0 {
            let field = &_field_x;
            output.x = field.get_or_default();
        }
        Ok(output)
    }
}
impl ComponentUpdate<Example> for generated_code::example::ExampleUpdate {
    fn merge(&mut self, update: generated_code::example::ExampleUpdate) {
        if let Some(value) = update.x { self.x = value; }
    }
}

impl ComponentMetaclass for Example {
    type Data = generated_code::example::Example;
    type Update = generated_code::example::ExampleUpdate;
    type CommandRequest = generated_code::example::ExampleCommandRequest;
    type CommandResponse = generated_code::example::ExampleCommandResponse;
}

#[derive(Debug)]
pub enum ExampleCommandRequest {
    test_command(generated_code::example::CommandData),
}

#[derive(Debug)]
pub enum ExampleCommandResponse {
    test_command(generated_code::example::CommandData),
}

impl ComponentVtable<Example> for Example {
    fn serialize_data(data: &Self::Data) -> Result<schema::SchemaComponentData, String> {
        let mut serialized_data = schema::SchemaComponentData::new(Self::component_id());
        TypeSerializer::<Self::Data>::serialize(data, serialized_data.fields_mut());
        Ok(serialized_data)
    }

    fn deserialize_data(data: &schema::SchemaComponentData) -> Result<Self::Data, String> {
        TypeSerializer::<Self::Data>::deserialize(data.fields())
    }

    fn serialize_update(update: &Self::Update) -> Result<schema::SchemaComponentUpdate, String> {
        let mut serialized_update = schema::SchemaComponentUpdate::new(Self::component_id());
        TypeSerializer::<Self::Update>::serialize(update, serialized_update.fields_mut());
        Ok(serialized_update)
    }

    fn deserialize_update(update: &schema::SchemaComponentUpdate) -> Result<Self::Update, String> {
        TypeSerializer::<Self::Update>::deserialize(update.fields())
    }

    fn serialize_command_request(request: &Self::CommandRequest) -> Result<schema::SchemaCommandRequest, String> {
        let command_index = match request {
            ExampleCommandRequest::test_command(_) => 1,
            _ => unreachable!()
        };
        let mut serialized_request = schema::SchemaCommandRequest::new(Self::component_id(), command_index);
        match request {
            ExampleCommandRequest::test_command(ref data) => {
                TypeSerializer::<generated_code::example::CommandData>::serialize(data, serialized_request.object_mut());
            },
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn deserialize_command_request(request: &schema::SchemaCommandRequest) -> Result<Self::CommandRequest, String> {
        match request.command_index() {
            1 => {
                Some(ExampleCommandRequest::test_command(
                    TypeSerializer::<generated_code::example::CommandData>::deserialize(request.object());
                ))
            },
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Example.", request.command_index())
        }
    }

    fn serialize_command_response(response: &Self::CommandResponse) -> Result<schema::SchemaCommandResponse, String> {
        let command_index = match response {
            ExampleCommandResponse::test_command(_) => 1,
            _ => unreachable!()
        };
        let mut serialized_response = schema::SchemaCommandResponse::new(Self::component_id(), command_index);
        match response {
            ExampleCommandResponse::test_command(ref data) => {
                TypeSerializer::<generated_code::example::CommandData>::serialize(data, serialized_response.object_mut());
            },
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn deserialize_command_response(response: &schema::SchemaCommandResponse) -> Result<Self::CommandResponse, String> {
        match response.command_index() {
            1 => {
                Some(ExampleCommandResponse::test_command(
                    TypeSerializer::<generated_code::example::CommandData>::deserialize(response.object());
                ))
            },
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Example.", request.command_index())
        }
    }
}



}

mod improbable {
use spatialos_sdk::worker::internal::schema::{self, SchemaField, SchemaObject};
use spatialos_sdk::worker::{ComponentMetaclass, ComponentUpdate, ComponentData, ComponentVtable, ComponentId, TypeSerializer};
use std::collections::BTreeMap;

use super as generated_code;

/* Enums. */
/* Types. */
#[derive(Debug)]
pub struct ComponentInterest {
    queries: Vec<generated_code::improbable::ComponentInterest::Query>,
}
impl TypeSerializer<ComponentInterest> for ComponentInterest {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        for element in &input.queries.iter() { TypeSerializer::<generated_code::improbable::ComponentInterest::Query>::serialize(element, &mut output.field::<SchemaObject>(1).add()); };
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            queries: { let size = input.field::<SchemaObject>(1).count(); let mut l = Vec::with_capacity(size); for i in [0..size] { l.push(TypeSerializer::<generated_code::improbable::ComponentInterest::Query>::deserialize(input.field::<SchemaObject>(1).index(i))); }; l },
        })
    }
}

#[derive(Debug)]
pub struct Coordinates {
    x: f64,
    y: f64,
    z: f64,
}
impl TypeSerializer<Coordinates> for Coordinates {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        output.field::<f64>(1).add(&input.x);
        output.field::<f64>(2).add(&input.y);
        output.field::<f64>(3).add(&input.z);
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            x: input.field::<f64>(1).get_or_default(),
            y: input.field::<f64>(2).get_or_default(),
            z: input.field::<f64>(3).get_or_default(),
        })
    }
}

#[derive(Debug)]
pub struct EdgeLength {
    x: f64,
    y: f64,
    z: f64,
}
impl TypeSerializer<EdgeLength> for EdgeLength {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        output.field::<f64>(1).add(&input.x);
        output.field::<f64>(2).add(&input.y);
        output.field::<f64>(3).add(&input.z);
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            x: input.field::<f64>(1).get_or_default(),
            y: input.field::<f64>(2).get_or_default(),
            z: input.field::<f64>(3).get_or_default(),
        })
    }
}

#[derive(Debug)]
pub struct EntityAclData {
    read_acl: generated_code::improbable::WorkerRequirementSet,
    component_write_acl: BTreeMap<u32, generated_code::improbable::WorkerRequirementSet>,
}
impl TypeSerializer<EntityAclData> for EntityAclData {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        TypeSerializer::<generated_code::improbable::WorkerRequirementSet>::serialize(&input.read_acl, &mut output.field::<SchemaObject>(1).add());
        for (k, v) in &input.component_write_acl { let object = output.field::<SchemaObject>(2).add(); object.field::<u32>(1).add(k); TypeSerializer::<generated_code::improbable::WorkerRequirementSet>::serialize(v, &mut object.field::<SchemaObject>(2).add()); };
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            read_acl: TypeSerializer::<generated_code::improbable::WorkerRequirementSet>::deserialize(input.field::<SchemaObject>(1).get_or_default()),
            component_write_acl: { let size = input.field::<SchemaObject>(2).count(); let mut m = BTreeMap::new(); for i in [0..size] { let kv = input.field::<SchemaObject>(2).index(i); m.insert(kv.field::<SchemaObject>(1), TypeSerializer::<generated_code::improbable::WorkerRequirementSet>::deserialize(kv.field::<SchemaObject>(2))); }; m },
        })
    }
}

#[derive(Debug)]
pub struct InterestData {
    component_interest: BTreeMap<u32, generated_code::improbable::ComponentInterest>,
}
impl TypeSerializer<InterestData> for InterestData {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        for (k, v) in &input.component_interest { let object = output.field::<SchemaObject>(1).add(); object.field::<u32>(1).add(k); TypeSerializer::<generated_code::improbable::ComponentInterest>::serialize(v, &mut object.field::<SchemaObject>(2).add()); };
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            component_interest: { let size = input.field::<SchemaObject>(1).count(); let mut m = BTreeMap::new(); for i in [0..size] { let kv = input.field::<SchemaObject>(1).index(i); m.insert(kv.field::<SchemaObject>(1), TypeSerializer::<generated_code::improbable::ComponentInterest>::deserialize(kv.field::<SchemaObject>(2))); }; m },
        })
    }
}

#[derive(Debug)]
pub struct MetadataData {
    entity_type: String,
}
impl TypeSerializer<MetadataData> for MetadataData {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        output.field::<String>(1).add(&input.entity_type);
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            entity_type: input.field::<String>(1).get_or_default(),
        })
    }
}

#[derive(Debug)]
pub struct PersistenceData {
}
impl TypeSerializer<PersistenceData> for PersistenceData {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
        })
    }
}

#[derive(Debug)]
pub struct PositionData {
    coords: generated_code::improbable::Coordinates,
}
impl TypeSerializer<PositionData> for PositionData {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        TypeSerializer::<generated_code::improbable::Coordinates>::serialize(&input.coords, &mut output.field::<SchemaObject>(1).add());
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            coords: TypeSerializer::<generated_code::improbable::Coordinates>::deserialize(input.field::<SchemaObject>(1).get_or_default()),
        })
    }
}

#[derive(Debug)]
pub struct Vector3d {
    x: f64,
    y: f64,
    z: f64,
}
impl TypeSerializer<Vector3d> for Vector3d {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        output.field::<f64>(1).add(&input.x);
        output.field::<f64>(2).add(&input.y);
        output.field::<f64>(3).add(&input.z);
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            x: input.field::<f64>(1).get_or_default(),
            y: input.field::<f64>(2).get_or_default(),
            z: input.field::<f64>(3).get_or_default(),
        })
    }
}

#[derive(Debug)]
pub struct Vector3f {
    x: f32,
    y: f32,
    z: f32,
}
impl TypeSerializer<Vector3f> for Vector3f {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        output.field::<f32>(1).add(&input.x);
        output.field::<f32>(2).add(&input.y);
        output.field::<f32>(3).add(&input.z);
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            x: input.field::<f32>(1).get_or_default(),
            y: input.field::<f32>(2).get_or_default(),
            z: input.field::<f32>(3).get_or_default(),
        })
    }
}

#[derive(Debug)]
pub struct WorkerAttributeSet {
    attribute: Vec<String>,
}
impl TypeSerializer<WorkerAttributeSet> for WorkerAttributeSet {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        output.field::<String>(1).add_list(&input.attribute[..]);
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            attribute: { let size = input.field::<String>(1).count(); let mut l = Vec::with_capacity(size); for i in [0..size] { l.push(input.field::<String>(1).index(i)); }; l },
        })
    }
}

#[derive(Debug)]
pub struct WorkerRequirementSet {
    attribute_set: Vec<generated_code::improbable::WorkerAttributeSet>,
}
impl TypeSerializer<WorkerRequirementSet> for WorkerRequirementSet {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        for element in &input.attribute_set.iter() { TypeSerializer::<generated_code::improbable::WorkerAttributeSet>::serialize(element, &mut output.field::<SchemaObject>(1).add()); };
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            attribute_set: { let size = input.field::<SchemaObject>(1).count(); let mut l = Vec::with_capacity(size); for i in [0..size] { l.push(TypeSerializer::<generated_code::improbable::WorkerAttributeSet>::deserialize(input.field::<SchemaObject>(1).index(i))); }; l },
        })
    }
}

/* Components. */ 
#[derive(Debug)]
pub struct EntityAcl {
    read_acl: generated_code::improbable::WorkerRequirementSet,
    component_write_acl: BTreeMap<u32, generated_code::improbable::WorkerRequirementSet>,
}
impl TypeSerializer<EntityAcl> for EntityAcl {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        TypeSerializer::<generated_code::improbable::WorkerRequirementSet>::serialize(&input.read_acl, &mut output.field::<SchemaObject>(1).add());
        for (k, v) in &input.component_write_acl { let object = output.field::<SchemaObject>(2).add(); object.field::<u32>(1).add(k); TypeSerializer::<generated_code::improbable::WorkerRequirementSet>::serialize(v, &mut object.field::<SchemaObject>(2).add()); };
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            read_acl: TypeSerializer::<generated_code::improbable::WorkerRequirementSet>::deserialize(input.field::<SchemaObject>(1).get_or_default()),
            component_write_acl: { let size = input.field::<SchemaObject>(2).count(); let mut m = BTreeMap::new(); for i in [0..size] { let kv = input.field::<SchemaObject>(2).index(i); m.insert(kv.field::<SchemaObject>(1), TypeSerializer::<generated_code::improbable::WorkerRequirementSet>::deserialize(kv.field::<SchemaObject>(2))); }; m },
        })
    }
}
impl ComponentData<EntityAcl> for generated_code::improbable::EntityAcl {
    fn merge(&mut self, update: generated_code::improbable::EntityAclUpdate) {
        if let Some(value) = update.read_acl { self.read_acl = Some(value); }
        if let Some(value) = update.component_write_acl { self.component_write_acl = Some(value); }
    }
}

#[derive(Debug)]
pub struct EntityAclUpdate {
    read_acl: Option<generated_code::improbable::WorkerRequirementSet>,
    component_write_acl: Option<BTreeMap<u32, generated_code::improbable::WorkerRequirementSet>>,
}
impl TypeSerializer<EntityAclUpdate> for EntityAclUpdate {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        if let Some(ref value) = input.read_acl {
            TypeSerializer::<generated_code::improbable::WorkerRequirementSet>::serialize(value, &mut output.field::<SchemaObject>(1).add());
        }
        if let Some(ref value) = input.component_write_acl {
            for (k, v) in value { let object = output.field::<SchemaObject>(2).add(); object.field::<u32>(1).add(k); TypeSerializer::<generated_code::improbable::WorkerRequirementSet>::serialize(v, &mut object.field::<SchemaObject>(2).add()); };
        }
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        let mut output = Self {
            read_acl: None,
            component_write_acl: None,
        };
        let _field_read_acl = input.field::<SchemaObject>(1);
        if _field_read_acl.count() > 0 {
            let field = &_field_read_acl;
            output.read_acl = TypeSerializer::<generated_code::improbable::WorkerRequirementSet>::deserialize(field.get_or_default());
        }
        let _field_component_write_acl = input.field::<SchemaObject>(2);
        if _field_component_write_acl.count() > 0 {
            let field = &_field_component_write_acl;
            output.component_write_acl = { let size = field.count(); let mut m = BTreeMap::new(); for i in [0..size] { let kv = field.index(i); m.insert(kv.field::<SchemaObject>(1), TypeSerializer::<generated_code::improbable::WorkerRequirementSet>::deserialize(kv.field::<SchemaObject>(2))); }; m };
        }
        Ok(output)
    }
}
impl ComponentUpdate<EntityAcl> for generated_code::improbable::EntityAclUpdate {
    fn merge(&mut self, update: generated_code::improbable::EntityAclUpdate) {
        if let Some(value) = update.read_acl { self.read_acl = value; }
        if let Some(value) = update.component_write_acl { self.component_write_acl = value; }
    }
}

impl ComponentMetaclass for EntityAcl {
    type Data = generated_code::improbable::EntityAcl;
    type Update = generated_code::improbable::EntityAclUpdate;
    type CommandRequest = generated_code::improbable::EntityAclCommandRequest;
    type CommandResponse = generated_code::improbable::EntityAclCommandResponse;
}

#[derive(Debug)]
pub enum EntityAclCommandRequest {
}

#[derive(Debug)]
pub enum EntityAclCommandResponse {
}

impl ComponentVtable<EntityAcl> for EntityAcl {
    fn serialize_data(data: &Self::Data) -> Result<schema::SchemaComponentData, String> {
        let mut serialized_data = schema::SchemaComponentData::new(Self::component_id());
        TypeSerializer::<Self::Data>::serialize(data, serialized_data.fields_mut());
        Ok(serialized_data)
    }

    fn deserialize_data(data: &schema::SchemaComponentData) -> Result<Self::Data, String> {
        TypeSerializer::<Self::Data>::deserialize(data.fields())
    }

    fn serialize_update(update: &Self::Update) -> Result<schema::SchemaComponentUpdate, String> {
        let mut serialized_update = schema::SchemaComponentUpdate::new(Self::component_id());
        TypeSerializer::<Self::Update>::serialize(update, serialized_update.fields_mut());
        Ok(serialized_update)
    }

    fn deserialize_update(update: &schema::SchemaComponentUpdate) -> Result<Self::Update, String> {
        TypeSerializer::<Self::Update>::deserialize(update.fields())
    }

    fn serialize_command_request(request: &Self::CommandRequest) -> Result<schema::SchemaCommandRequest, String> {
        let command_index = match request {
            _ => unreachable!()
        };
        let mut serialized_request = schema::SchemaCommandRequest::new(Self::component_id(), command_index);
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn deserialize_command_request(request: &schema::SchemaCommandRequest) -> Result<Self::CommandRequest, String> {
        match request.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component EntityAcl.", request.command_index())
        }
    }

    fn serialize_command_response(response: &Self::CommandResponse) -> Result<schema::SchemaCommandResponse, String> {
        let command_index = match response {
            _ => unreachable!()
        };
        let mut serialized_response = schema::SchemaCommandResponse::new(Self::component_id(), command_index);
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn deserialize_command_response(response: &schema::SchemaCommandResponse) -> Result<Self::CommandResponse, String> {
        match response.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component EntityAcl.", request.command_index())
        }
    }
}

#[derive(Debug)]
pub struct Interest {
    component_interest: BTreeMap<u32, generated_code::improbable::ComponentInterest>,
}
impl TypeSerializer<Interest> for Interest {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        for (k, v) in &input.component_interest { let object = output.field::<SchemaObject>(1).add(); object.field::<u32>(1).add(k); TypeSerializer::<generated_code::improbable::ComponentInterest>::serialize(v, &mut object.field::<SchemaObject>(2).add()); };
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            component_interest: { let size = input.field::<SchemaObject>(1).count(); let mut m = BTreeMap::new(); for i in [0..size] { let kv = input.field::<SchemaObject>(1).index(i); m.insert(kv.field::<SchemaObject>(1), TypeSerializer::<generated_code::improbable::ComponentInterest>::deserialize(kv.field::<SchemaObject>(2))); }; m },
        })
    }
}
impl ComponentData<Interest> for generated_code::improbable::Interest {
    fn merge(&mut self, update: generated_code::improbable::InterestUpdate) {
        if let Some(value) = update.component_interest { self.component_interest = Some(value); }
    }
}

#[derive(Debug)]
pub struct InterestUpdate {
    component_interest: Option<BTreeMap<u32, generated_code::improbable::ComponentInterest>>,
}
impl TypeSerializer<InterestUpdate> for InterestUpdate {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        if let Some(ref value) = input.component_interest {
            for (k, v) in value { let object = output.field::<SchemaObject>(1).add(); object.field::<u32>(1).add(k); TypeSerializer::<generated_code::improbable::ComponentInterest>::serialize(v, &mut object.field::<SchemaObject>(2).add()); };
        }
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        let mut output = Self {
            component_interest: None,
        };
        let _field_component_interest = input.field::<SchemaObject>(1);
        if _field_component_interest.count() > 0 {
            let field = &_field_component_interest;
            output.component_interest = { let size = field.count(); let mut m = BTreeMap::new(); for i in [0..size] { let kv = field.index(i); m.insert(kv.field::<SchemaObject>(1), TypeSerializer::<generated_code::improbable::ComponentInterest>::deserialize(kv.field::<SchemaObject>(2))); }; m };
        }
        Ok(output)
    }
}
impl ComponentUpdate<Interest> for generated_code::improbable::InterestUpdate {
    fn merge(&mut self, update: generated_code::improbable::InterestUpdate) {
        if let Some(value) = update.component_interest { self.component_interest = value; }
    }
}

impl ComponentMetaclass for Interest {
    type Data = generated_code::improbable::Interest;
    type Update = generated_code::improbable::InterestUpdate;
    type CommandRequest = generated_code::improbable::InterestCommandRequest;
    type CommandResponse = generated_code::improbable::InterestCommandResponse;
}

#[derive(Debug)]
pub enum InterestCommandRequest {
}

#[derive(Debug)]
pub enum InterestCommandResponse {
}

impl ComponentVtable<Interest> for Interest {
    fn serialize_data(data: &Self::Data) -> Result<schema::SchemaComponentData, String> {
        let mut serialized_data = schema::SchemaComponentData::new(Self::component_id());
        TypeSerializer::<Self::Data>::serialize(data, serialized_data.fields_mut());
        Ok(serialized_data)
    }

    fn deserialize_data(data: &schema::SchemaComponentData) -> Result<Self::Data, String> {
        TypeSerializer::<Self::Data>::deserialize(data.fields())
    }

    fn serialize_update(update: &Self::Update) -> Result<schema::SchemaComponentUpdate, String> {
        let mut serialized_update = schema::SchemaComponentUpdate::new(Self::component_id());
        TypeSerializer::<Self::Update>::serialize(update, serialized_update.fields_mut());
        Ok(serialized_update)
    }

    fn deserialize_update(update: &schema::SchemaComponentUpdate) -> Result<Self::Update, String> {
        TypeSerializer::<Self::Update>::deserialize(update.fields())
    }

    fn serialize_command_request(request: &Self::CommandRequest) -> Result<schema::SchemaCommandRequest, String> {
        let command_index = match request {
            _ => unreachable!()
        };
        let mut serialized_request = schema::SchemaCommandRequest::new(Self::component_id(), command_index);
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn deserialize_command_request(request: &schema::SchemaCommandRequest) -> Result<Self::CommandRequest, String> {
        match request.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Interest.", request.command_index())
        }
    }

    fn serialize_command_response(response: &Self::CommandResponse) -> Result<schema::SchemaCommandResponse, String> {
        let command_index = match response {
            _ => unreachable!()
        };
        let mut serialized_response = schema::SchemaCommandResponse::new(Self::component_id(), command_index);
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn deserialize_command_response(response: &schema::SchemaCommandResponse) -> Result<Self::CommandResponse, String> {
        match response.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Interest.", request.command_index())
        }
    }
}

#[derive(Debug)]
pub struct Metadata {
    entity_type: String,
}
impl TypeSerializer<Metadata> for Metadata {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        output.field::<String>(1).add(&input.entity_type);
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            entity_type: input.field::<String>(1).get_or_default(),
        })
    }
}
impl ComponentData<Metadata> for generated_code::improbable::Metadata {
    fn merge(&mut self, update: generated_code::improbable::MetadataUpdate) {
        if let Some(value) = update.entity_type { self.entity_type = Some(value); }
    }
}

#[derive(Debug)]
pub struct MetadataUpdate {
    entity_type: Option<String>,
}
impl TypeSerializer<MetadataUpdate> for MetadataUpdate {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        if let Some(ref value) = input.entity_type {
            output.field::<String>(1).add(value);
        }
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        let mut output = Self {
            entity_type: None,
        };
        let _field_entity_type = input.field::<String>(1);
        if _field_entity_type.count() > 0 {
            let field = &_field_entity_type;
            output.entity_type = field.get_or_default();
        }
        Ok(output)
    }
}
impl ComponentUpdate<Metadata> for generated_code::improbable::MetadataUpdate {
    fn merge(&mut self, update: generated_code::improbable::MetadataUpdate) {
        if let Some(value) = update.entity_type { self.entity_type = value; }
    }
}

impl ComponentMetaclass for Metadata {
    type Data = generated_code::improbable::Metadata;
    type Update = generated_code::improbable::MetadataUpdate;
    type CommandRequest = generated_code::improbable::MetadataCommandRequest;
    type CommandResponse = generated_code::improbable::MetadataCommandResponse;
}

#[derive(Debug)]
pub enum MetadataCommandRequest {
}

#[derive(Debug)]
pub enum MetadataCommandResponse {
}

impl ComponentVtable<Metadata> for Metadata {
    fn serialize_data(data: &Self::Data) -> Result<schema::SchemaComponentData, String> {
        let mut serialized_data = schema::SchemaComponentData::new(Self::component_id());
        TypeSerializer::<Self::Data>::serialize(data, serialized_data.fields_mut());
        Ok(serialized_data)
    }

    fn deserialize_data(data: &schema::SchemaComponentData) -> Result<Self::Data, String> {
        TypeSerializer::<Self::Data>::deserialize(data.fields())
    }

    fn serialize_update(update: &Self::Update) -> Result<schema::SchemaComponentUpdate, String> {
        let mut serialized_update = schema::SchemaComponentUpdate::new(Self::component_id());
        TypeSerializer::<Self::Update>::serialize(update, serialized_update.fields_mut());
        Ok(serialized_update)
    }

    fn deserialize_update(update: &schema::SchemaComponentUpdate) -> Result<Self::Update, String> {
        TypeSerializer::<Self::Update>::deserialize(update.fields())
    }

    fn serialize_command_request(request: &Self::CommandRequest) -> Result<schema::SchemaCommandRequest, String> {
        let command_index = match request {
            _ => unreachable!()
        };
        let mut serialized_request = schema::SchemaCommandRequest::new(Self::component_id(), command_index);
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn deserialize_command_request(request: &schema::SchemaCommandRequest) -> Result<Self::CommandRequest, String> {
        match request.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Metadata.", request.command_index())
        }
    }

    fn serialize_command_response(response: &Self::CommandResponse) -> Result<schema::SchemaCommandResponse, String> {
        let command_index = match response {
            _ => unreachable!()
        };
        let mut serialized_response = schema::SchemaCommandResponse::new(Self::component_id(), command_index);
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn deserialize_command_response(response: &schema::SchemaCommandResponse) -> Result<Self::CommandResponse, String> {
        match response.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Metadata.", request.command_index())
        }
    }
}

#[derive(Debug)]
pub struct Persistence {
}
impl TypeSerializer<Persistence> for Persistence {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
        })
    }
}
impl ComponentData<Persistence> for generated_code::improbable::Persistence {
    fn merge(&mut self, update: generated_code::improbable::PersistenceUpdate) {
    }
}

#[derive(Debug)]
pub struct PersistenceUpdate {
}
impl TypeSerializer<PersistenceUpdate> for PersistenceUpdate {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        let mut output = Self {
        };
        Ok(output)
    }
}
impl ComponentUpdate<Persistence> for generated_code::improbable::PersistenceUpdate {
    fn merge(&mut self, update: generated_code::improbable::PersistenceUpdate) {
    }
}

impl ComponentMetaclass for Persistence {
    type Data = generated_code::improbable::Persistence;
    type Update = generated_code::improbable::PersistenceUpdate;
    type CommandRequest = generated_code::improbable::PersistenceCommandRequest;
    type CommandResponse = generated_code::improbable::PersistenceCommandResponse;
}

#[derive(Debug)]
pub enum PersistenceCommandRequest {
}

#[derive(Debug)]
pub enum PersistenceCommandResponse {
}

impl ComponentVtable<Persistence> for Persistence {
    fn serialize_data(data: &Self::Data) -> Result<schema::SchemaComponentData, String> {
        let mut serialized_data = schema::SchemaComponentData::new(Self::component_id());
        TypeSerializer::<Self::Data>::serialize(data, serialized_data.fields_mut());
        Ok(serialized_data)
    }

    fn deserialize_data(data: &schema::SchemaComponentData) -> Result<Self::Data, String> {
        TypeSerializer::<Self::Data>::deserialize(data.fields())
    }

    fn serialize_update(update: &Self::Update) -> Result<schema::SchemaComponentUpdate, String> {
        let mut serialized_update = schema::SchemaComponentUpdate::new(Self::component_id());
        TypeSerializer::<Self::Update>::serialize(update, serialized_update.fields_mut());
        Ok(serialized_update)
    }

    fn deserialize_update(update: &schema::SchemaComponentUpdate) -> Result<Self::Update, String> {
        TypeSerializer::<Self::Update>::deserialize(update.fields())
    }

    fn serialize_command_request(request: &Self::CommandRequest) -> Result<schema::SchemaCommandRequest, String> {
        let command_index = match request {
            _ => unreachable!()
        };
        let mut serialized_request = schema::SchemaCommandRequest::new(Self::component_id(), command_index);
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn deserialize_command_request(request: &schema::SchemaCommandRequest) -> Result<Self::CommandRequest, String> {
        match request.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Persistence.", request.command_index())
        }
    }

    fn serialize_command_response(response: &Self::CommandResponse) -> Result<schema::SchemaCommandResponse, String> {
        let command_index = match response {
            _ => unreachable!()
        };
        let mut serialized_response = schema::SchemaCommandResponse::new(Self::component_id(), command_index);
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn deserialize_command_response(response: &schema::SchemaCommandResponse) -> Result<Self::CommandResponse, String> {
        match response.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Persistence.", request.command_index())
        }
    }
}

#[derive(Debug)]
pub struct Position {
    coords: generated_code::improbable::Coordinates,
}
impl TypeSerializer<Position> for Position {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        TypeSerializer::<generated_code::improbable::Coordinates>::serialize(&input.coords, &mut output.field::<SchemaObject>(1).add());
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            coords: TypeSerializer::<generated_code::improbable::Coordinates>::deserialize(input.field::<SchemaObject>(1).get_or_default()),
        })
    }
}
impl ComponentData<Position> for generated_code::improbable::Position {
    fn merge(&mut self, update: generated_code::improbable::PositionUpdate) {
        if let Some(value) = update.coords { self.coords = Some(value); }
    }
}

#[derive(Debug)]
pub struct PositionUpdate {
    coords: Option<generated_code::improbable::Coordinates>,
}
impl TypeSerializer<PositionUpdate> for PositionUpdate {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        if let Some(ref value) = input.coords {
            TypeSerializer::<generated_code::improbable::Coordinates>::serialize(value, &mut output.field::<SchemaObject>(1).add());
        }
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        let mut output = Self {
            coords: None,
        };
        let _field_coords = input.field::<SchemaObject>(1);
        if _field_coords.count() > 0 {
            let field = &_field_coords;
            output.coords = TypeSerializer::<generated_code::improbable::Coordinates>::deserialize(field.get_or_default());
        }
        Ok(output)
    }
}
impl ComponentUpdate<Position> for generated_code::improbable::PositionUpdate {
    fn merge(&mut self, update: generated_code::improbable::PositionUpdate) {
        if let Some(value) = update.coords { self.coords = value; }
    }
}

impl ComponentMetaclass for Position {
    type Data = generated_code::improbable::Position;
    type Update = generated_code::improbable::PositionUpdate;
    type CommandRequest = generated_code::improbable::PositionCommandRequest;
    type CommandResponse = generated_code::improbable::PositionCommandResponse;
}

#[derive(Debug)]
pub enum PositionCommandRequest {
}

#[derive(Debug)]
pub enum PositionCommandResponse {
}

impl ComponentVtable<Position> for Position {
    fn serialize_data(data: &Self::Data) -> Result<schema::SchemaComponentData, String> {
        let mut serialized_data = schema::SchemaComponentData::new(Self::component_id());
        TypeSerializer::<Self::Data>::serialize(data, serialized_data.fields_mut());
        Ok(serialized_data)
    }

    fn deserialize_data(data: &schema::SchemaComponentData) -> Result<Self::Data, String> {
        TypeSerializer::<Self::Data>::deserialize(data.fields())
    }

    fn serialize_update(update: &Self::Update) -> Result<schema::SchemaComponentUpdate, String> {
        let mut serialized_update = schema::SchemaComponentUpdate::new(Self::component_id());
        TypeSerializer::<Self::Update>::serialize(update, serialized_update.fields_mut());
        Ok(serialized_update)
    }

    fn deserialize_update(update: &schema::SchemaComponentUpdate) -> Result<Self::Update, String> {
        TypeSerializer::<Self::Update>::deserialize(update.fields())
    }

    fn serialize_command_request(request: &Self::CommandRequest) -> Result<schema::SchemaCommandRequest, String> {
        let command_index = match request {
            _ => unreachable!()
        };
        let mut serialized_request = schema::SchemaCommandRequest::new(Self::component_id(), command_index);
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn deserialize_command_request(request: &schema::SchemaCommandRequest) -> Result<Self::CommandRequest, String> {
        match request.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Position.", request.command_index())
        }
    }

    fn serialize_command_response(response: &Self::CommandResponse) -> Result<schema::SchemaCommandResponse, String> {
        let command_index = match response {
            _ => unreachable!()
        };
        let mut serialized_response = schema::SchemaCommandResponse::new(Self::component_id(), command_index);
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn deserialize_command_response(response: &schema::SchemaCommandResponse) -> Result<Self::CommandResponse, String> {
        match response.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Position.", request.command_index())
        }
    }
}



mod ComponentInterest {
use spatialos_sdk::worker::internal::schema::{self, SchemaField, SchemaObject};
use spatialos_sdk::worker::{ComponentMetaclass, ComponentUpdate, ComponentData, ComponentVtable, ComponentId, TypeSerializer};
use std::collections::BTreeMap;

use super::super as generated_code;

/* Enums. */
/* Types. */
#[derive(Debug)]
pub struct BoxConstraint {
    center: generated_code::improbable::Coordinates,
    edge_length: generated_code::improbable::EdgeLength,
}
impl TypeSerializer<BoxConstraint> for BoxConstraint {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        TypeSerializer::<generated_code::improbable::Coordinates>::serialize(&input.center, &mut output.field::<SchemaObject>(1).add());
        TypeSerializer::<generated_code::improbable::EdgeLength>::serialize(&input.edge_length, &mut output.field::<SchemaObject>(2).add());
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            center: TypeSerializer::<generated_code::improbable::Coordinates>::deserialize(input.field::<SchemaObject>(1).get_or_default()),
            edge_length: TypeSerializer::<generated_code::improbable::EdgeLength>::deserialize(input.field::<SchemaObject>(2).get_or_default()),
        })
    }
}

#[derive(Debug)]
pub struct CylinderConstraint {
    center: generated_code::improbable::Coordinates,
    radius: f64,
}
impl TypeSerializer<CylinderConstraint> for CylinderConstraint {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        TypeSerializer::<generated_code::improbable::Coordinates>::serialize(&input.center, &mut output.field::<SchemaObject>(1).add());
        output.field::<f64>(2).add(&input.radius);
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            center: TypeSerializer::<generated_code::improbable::Coordinates>::deserialize(input.field::<SchemaObject>(1).get_or_default()),
            radius: input.field::<f64>(2).get_or_default(),
        })
    }
}

#[derive(Debug)]
pub struct Query {
    constraint: generated_code::improbable::ComponentInterest::QueryConstraint,
    full_snapshot_result: Option<bool>,
    result_component_id: Vec<u32>,
    frequency: Option<f32>,
}
impl TypeSerializer<Query> for Query {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        TypeSerializer::<generated_code::improbable::ComponentInterest::QueryConstraint>::serialize(&input.constraint, &mut output.field::<SchemaObject>(1).add());
        output.field::<bool>(2).add(&input.full_snapshot_result);
        output.field::<u32>(3).add_list(&input.result_component_id[..]);
        output.field::<f32>(4).add(&input.frequency);
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            constraint: TypeSerializer::<generated_code::improbable::ComponentInterest::QueryConstraint>::deserialize(input.field::<SchemaObject>(1).get_or_default()),
            full_snapshot_result: input.field::<bool>(2).get().map(|v| { v }),
            result_component_id: { let size = input.field::<u32>(3).count(); let mut l = Vec::with_capacity(size); for i in [0..size] { l.push(input.field::<u32>(3).index(i)); }; l },
            frequency: input.field::<f32>(4).get().map(|v| { v }),
        })
    }
}

#[derive(Debug)]
pub struct QueryConstraint {
    sphere_constraint: Option<generated_code::improbable::ComponentInterest::SphereConstraint>,
    cylinder_constraint: Option<generated_code::improbable::ComponentInterest::CylinderConstraint>,
    box_constraint: Option<generated_code::improbable::ComponentInterest::BoxConstraint>,
    relative_sphere_constraint: Option<generated_code::improbable::ComponentInterest::RelativeSphereConstraint>,
    relative_cylinder_constraint: Option<generated_code::improbable::ComponentInterest::RelativeCylinderConstraint>,
    relative_box_constraint: Option<generated_code::improbable::ComponentInterest::RelativeBoxConstraint>,
    entity_id_constraint: Option<i64>,
    component_constraint: Option<u32>,
    and_constraint: Vec<generated_code::improbable::ComponentInterest::QueryConstraint>,
    or_constraint: Vec<generated_code::improbable::ComponentInterest::QueryConstraint>,
}
impl TypeSerializer<QueryConstraint> for QueryConstraint {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        TypeSerializer::<generated_code::improbable::ComponentInterest::SphereConstraint>::serialize(&input.sphere_constraint, &mut output.field::<SchemaObject>(1).add());
        TypeSerializer::<generated_code::improbable::ComponentInterest::CylinderConstraint>::serialize(&input.cylinder_constraint, &mut output.field::<SchemaObject>(2).add());
        TypeSerializer::<generated_code::improbable::ComponentInterest::BoxConstraint>::serialize(&input.box_constraint, &mut output.field::<SchemaObject>(3).add());
        TypeSerializer::<generated_code::improbable::ComponentInterest::RelativeSphereConstraint>::serialize(&input.relative_sphere_constraint, &mut output.field::<SchemaObject>(4).add());
        TypeSerializer::<generated_code::improbable::ComponentInterest::RelativeCylinderConstraint>::serialize(&input.relative_cylinder_constraint, &mut output.field::<SchemaObject>(5).add());
        TypeSerializer::<generated_code::improbable::ComponentInterest::RelativeBoxConstraint>::serialize(&input.relative_box_constraint, &mut output.field::<SchemaObject>(6).add());
        output.field::<i64>(7).add(&input.entity_id_constraint);
        output.field::<u32>(8).add(&input.component_constraint);
        for element in &input.and_constraint.iter() { TypeSerializer::<generated_code::improbable::ComponentInterest::QueryConstraint>::serialize(element, &mut output.field::<SchemaObject>(9).add()); };
        for element in &input.or_constraint.iter() { TypeSerializer::<generated_code::improbable::ComponentInterest::QueryConstraint>::serialize(element, &mut output.field::<SchemaObject>(10).add()); };
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            sphere_constraint: input.field::<SchemaObject>(1).get().map(|v| { TypeSerializer::<generated_code::improbable::ComponentInterest::SphereConstraint>::deserialize(v) }),
            cylinder_constraint: input.field::<SchemaObject>(2).get().map(|v| { TypeSerializer::<generated_code::improbable::ComponentInterest::CylinderConstraint>::deserialize(v) }),
            box_constraint: input.field::<SchemaObject>(3).get().map(|v| { TypeSerializer::<generated_code::improbable::ComponentInterest::BoxConstraint>::deserialize(v) }),
            relative_sphere_constraint: input.field::<SchemaObject>(4).get().map(|v| { TypeSerializer::<generated_code::improbable::ComponentInterest::RelativeSphereConstraint>::deserialize(v) }),
            relative_cylinder_constraint: input.field::<SchemaObject>(5).get().map(|v| { TypeSerializer::<generated_code::improbable::ComponentInterest::RelativeCylinderConstraint>::deserialize(v) }),
            relative_box_constraint: input.field::<SchemaObject>(6).get().map(|v| { TypeSerializer::<generated_code::improbable::ComponentInterest::RelativeBoxConstraint>::deserialize(v) }),
            entity_id_constraint: input.field::<i64>(7).get().map(|v| { v }),
            component_constraint: input.field::<u32>(8).get().map(|v| { v }),
            and_constraint: { let size = input.field::<SchemaObject>(9).count(); let mut l = Vec::with_capacity(size); for i in [0..size] { l.push(TypeSerializer::<generated_code::improbable::ComponentInterest::QueryConstraint>::deserialize(input.field::<SchemaObject>(9).index(i))); }; l },
            or_constraint: { let size = input.field::<SchemaObject>(10).count(); let mut l = Vec::with_capacity(size); for i in [0..size] { l.push(TypeSerializer::<generated_code::improbable::ComponentInterest::QueryConstraint>::deserialize(input.field::<SchemaObject>(10).index(i))); }; l },
        })
    }
}

#[derive(Debug)]
pub struct RelativeBoxConstraint {
    edge_length: generated_code::improbable::EdgeLength,
}
impl TypeSerializer<RelativeBoxConstraint> for RelativeBoxConstraint {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        TypeSerializer::<generated_code::improbable::EdgeLength>::serialize(&input.edge_length, &mut output.field::<SchemaObject>(1).add());
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            edge_length: TypeSerializer::<generated_code::improbable::EdgeLength>::deserialize(input.field::<SchemaObject>(1).get_or_default()),
        })
    }
}

#[derive(Debug)]
pub struct RelativeCylinderConstraint {
    radius: f64,
}
impl TypeSerializer<RelativeCylinderConstraint> for RelativeCylinderConstraint {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        output.field::<f64>(1).add(&input.radius);
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            radius: input.field::<f64>(1).get_or_default(),
        })
    }
}

#[derive(Debug)]
pub struct RelativeSphereConstraint {
    radius: f64,
}
impl TypeSerializer<RelativeSphereConstraint> for RelativeSphereConstraint {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        output.field::<f64>(1).add(&input.radius);
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            radius: input.field::<f64>(1).get_or_default(),
        })
    }
}

#[derive(Debug)]
pub struct SphereConstraint {
    center: generated_code::improbable::Coordinates,
    radius: f64,
}
impl TypeSerializer<SphereConstraint> for SphereConstraint {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        TypeSerializer::<generated_code::improbable::Coordinates>::serialize(&input.center, &mut output.field::<SchemaObject>(1).add());
        output.field::<f64>(2).add(&input.radius);
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            center: TypeSerializer::<generated_code::improbable::Coordinates>::deserialize(input.field::<SchemaObject>(1).get_or_default()),
            radius: input.field::<f64>(2).get_or_default(),
        })
    }
}

/* Components. */ 


}

}
