#[allow(unused_imports)]
mod generated {
use spatialos_sdk::worker::internal::schema::{self, SchemaField, SchemaObject};
use spatialos_sdk::worker::{ComponentMetaclass, ComponentUpdate, ComponentData, ComponentVtable, ComponentId, TypeSerializer};
use std::collections::BTreeMap;

use super::generated as generated;

/* Enums. */
/* Types. */
/* Components. */ 

pub mod example {
use spatialos_sdk::worker::internal::schema::{self, SchemaField, SchemaObject};
use spatialos_sdk::worker::{ComponentMetaclass, ComponentUpdate, ComponentData, ComponentVtable, ComponentId, TypeSerializer};
use std::collections::BTreeMap;

use super::super::generated as generated;

/* Enums. */
/* Types. */
#[derive(Debug)]
pub struct CommandData {
    value: i32,
}
impl TypeSerializer<CommandData> for CommandData {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        output.field::<i32>(1).add(input.value);
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            value: input.field::<i32>(1).get_or_default(),
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
        output.field::<f32>(1).add(input.x);
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            x: input.field::<f32>(1).get_or_default(),
        })
    }
}
impl ComponentData<Example> for Example {
    fn merge(&mut self, update: ExampleUpdate) {
        if let Some(value) = update.x { self.x = value; }
    }
}

#[derive(Debug)]
pub struct ExampleUpdate {
    x: Option<f32>,
}
impl TypeSerializer<ExampleUpdate> for ExampleUpdate {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        if let Some(ref value) = input.x {
            output.field::<f32>(1).add(value.clone());
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
            output.x = Some(field.get_or_default());
        }
        Ok(output)
    }
}
impl ComponentUpdate<Example> for ExampleUpdate {
    fn merge(&mut self, update: ExampleUpdate) {
        if update.x.is_some() { self.x = update.x; }
    }
}

impl ComponentMetaclass for Example {
    type Data = generated::example::Example;
    type Update = generated::example::ExampleUpdate;
    type CommandRequest = generated::example::ExampleCommandRequest;
    type CommandResponse = generated::example::ExampleCommandResponse;

    fn component_id() -> ComponentId {
        1000
    }
}

#[derive(Debug)]
pub enum ExampleCommandRequest {
    test_command(generated::example::CommandData),
}

#[derive(Debug)]
pub enum ExampleCommandResponse {
    test_command(generated::example::CommandData),
}

impl ComponentVtable<Example> for Example {
    fn serialize_data(data: &generated::example::Example) -> Result<schema::SchemaComponentData, String> {
        let mut serialized_data = schema::SchemaComponentData::new(Self::component_id());
        TypeSerializer::<generated::example::Example>::serialize(data, &mut serialized_data.fields_mut());
        Ok(serialized_data)
    }

    fn deserialize_data(data: &schema::SchemaComponentData) -> Result<generated::example::Example, String> {
        TypeSerializer::<generated::example::Example>::deserialize(&data.fields())
    }

    fn serialize_update(update: &generated::example::ExampleUpdate) -> Result<schema::SchemaComponentUpdate, String> {
        let mut serialized_update = schema::SchemaComponentUpdate::new(Self::component_id());
        TypeSerializer::<generated::example::ExampleUpdate>::serialize(update, &mut serialized_update.fields_mut());
        Ok(serialized_update)
    }

    fn deserialize_update(update: &schema::SchemaComponentUpdate) -> Result<generated::example::ExampleUpdate, String> {
        TypeSerializer::<generated::example::ExampleUpdate>::deserialize(&update.fields())
    }

    fn serialize_command_request(request: &generated::example::ExampleCommandRequest) -> Result<schema::SchemaCommandRequest, String> {
        let command_index = match request {
            ExampleCommandRequest::test_command(_) => 1,
            _ => unreachable!()
        };
        let mut serialized_request = schema::SchemaCommandRequest::new(Self::component_id(), command_index);
        match request {
            ExampleCommandRequest::test_command(ref data) => {
                TypeSerializer::<generated::example::CommandData>::serialize(data, &mut serialized_request.object_mut());
            },
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn deserialize_command_request(request: &schema::SchemaCommandRequest) -> Result<generated::example::ExampleCommandRequest, String> {
        match request.command_index() {
            1 => {
                let result = TypeSerializer::<generated::example::CommandData>::deserialize(&request.object());
                result.and_then(|deserialized| Ok(ExampleCommandRequest::test_command(deserialized)))
            },
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Example.", request.command_index()))
        }
    }

    fn serialize_command_response(response: &generated::example::ExampleCommandResponse) -> Result<schema::SchemaCommandResponse, String> {
        let command_index = match response {
            ExampleCommandResponse::test_command(_) => 1,
            _ => unreachable!()
        };
        let mut serialized_response = schema::SchemaCommandResponse::new(Self::component_id(), command_index);
        match response {
            ExampleCommandResponse::test_command(ref data) => {
                TypeSerializer::<generated::example::CommandData>::serialize(data, &mut serialized_response.object_mut());
            },
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn deserialize_command_response(response: &schema::SchemaCommandResponse) -> Result<generated::example::ExampleCommandResponse, String> {
        match response.command_index() {
            1 => {
                let result = TypeSerializer::<generated::example::CommandData>::deserialize(&response.object());
                result.and_then(|deserialized| Ok(ExampleCommandResponse::test_command(deserialized)))
            },
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Example.", response.command_index()))
        }
    }
}

}

pub mod improbable {
use spatialos_sdk::worker::internal::schema::{self, SchemaField, SchemaObject};
use spatialos_sdk::worker::{ComponentMetaclass, ComponentUpdate, ComponentData, ComponentVtable, ComponentId, TypeSerializer};
use std::collections::BTreeMap;

use super::super::generated as generated;

/* Enums. */
/* Types. */
#[derive(Debug)]
pub struct ComponentInterest {
    queries: Vec<generated::improbable::ComponentInterest_Query>,
}
impl TypeSerializer<ComponentInterest> for ComponentInterest {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        for element in input.queries.iter() { TypeSerializer::<generated::improbable::ComponentInterest_Query>::serialize(element, &mut output.field::<SchemaObject>(1).add()); };
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            queries: { let size = input.field::<SchemaObject>(1).count(); let mut l = Vec::with_capacity(size); for i in 0..size { l.push(TypeSerializer::<generated::improbable::ComponentInterest_Query>::deserialize(input.field::<SchemaObject>(1).index(i))); }; l },
        })
    }
}

#[derive(Debug)]
pub struct ComponentInterest_BoxConstraint {
    center: generated::improbable::Coordinates,
    edge_length: generated::improbable::EdgeLength,
}
impl TypeSerializer<ComponentInterest_BoxConstraint> for ComponentInterest_BoxConstraint {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        TypeSerializer::<generated::improbable::Coordinates>::serialize(input.center, &mut output.field::<SchemaObject>(1).add());
        TypeSerializer::<generated::improbable::EdgeLength>::serialize(input.edge_length, &mut output.field::<SchemaObject>(2).add());
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            center: TypeSerializer::<generated::improbable::Coordinates>::deserialize(input.field::<SchemaObject>(1).get_or_default()),
            edge_length: TypeSerializer::<generated::improbable::EdgeLength>::deserialize(input.field::<SchemaObject>(2).get_or_default()),
        })
    }
}

#[derive(Debug)]
pub struct ComponentInterest_CylinderConstraint {
    center: generated::improbable::Coordinates,
    radius: f64,
}
impl TypeSerializer<ComponentInterest_CylinderConstraint> for ComponentInterest_CylinderConstraint {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        TypeSerializer::<generated::improbable::Coordinates>::serialize(input.center, &mut output.field::<SchemaObject>(1).add());
        output.field::<f64>(2).add(input.radius);
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            center: TypeSerializer::<generated::improbable::Coordinates>::deserialize(input.field::<SchemaObject>(1).get_or_default()),
            radius: input.field::<f64>(2).get_or_default(),
        })
    }
}

#[derive(Debug)]
pub struct ComponentInterest_Query {
    constraint: generated::improbable::ComponentInterest_QueryConstraint,
    full_snapshot_result: Option<bool>,
    result_component_id: Vec<u32>,
    frequency: Option<f32>,
}
impl TypeSerializer<ComponentInterest_Query> for ComponentInterest_Query {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        TypeSerializer::<generated::improbable::ComponentInterest_QueryConstraint>::serialize(input.constraint, &mut output.field::<SchemaObject>(1).add());
        output.field::<bool>(2).add(input.full_snapshot_result);
        output.field::<u32>(3).add_list(input.result_component_id[..]);
        output.field::<f32>(4).add(input.frequency);
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            constraint: TypeSerializer::<generated::improbable::ComponentInterest_QueryConstraint>::deserialize(input.field::<SchemaObject>(1).get_or_default()),
            full_snapshot_result: input.field::<bool>(2).get().map(|v| { v }),
            result_component_id: { let size = input.field::<u32>(3).count(); let mut l = Vec::with_capacity(size); for i in 0..size { l.push(input.field::<u32>(3).index(i)); }; l },
            frequency: input.field::<f32>(4).get().map(|v| { v }),
        })
    }
}

#[derive(Debug)]
pub struct ComponentInterest_QueryConstraint {
    sphere_constraint: Option<generated::improbable::ComponentInterest_SphereConstraint>,
    cylinder_constraint: Option<generated::improbable::ComponentInterest_CylinderConstraint>,
    box_constraint: Option<generated::improbable::ComponentInterest_BoxConstraint>,
    relative_sphere_constraint: Option<generated::improbable::ComponentInterest_RelativeSphereConstraint>,
    relative_cylinder_constraint: Option<generated::improbable::ComponentInterest_RelativeCylinderConstraint>,
    relative_box_constraint: Option<generated::improbable::ComponentInterest_RelativeBoxConstraint>,
    entity_id_constraint: Option<i64>,
    component_constraint: Option<u32>,
    and_constraint: Vec<generated::improbable::ComponentInterest_QueryConstraint>,
    or_constraint: Vec<generated::improbable::ComponentInterest_QueryConstraint>,
}
impl TypeSerializer<ComponentInterest_QueryConstraint> for ComponentInterest_QueryConstraint {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        TypeSerializer::<generated::improbable::ComponentInterest_SphereConstraint>::serialize(input.sphere_constraint, &mut output.field::<SchemaObject>(1).add());
        TypeSerializer::<generated::improbable::ComponentInterest_CylinderConstraint>::serialize(input.cylinder_constraint, &mut output.field::<SchemaObject>(2).add());
        TypeSerializer::<generated::improbable::ComponentInterest_BoxConstraint>::serialize(input.box_constraint, &mut output.field::<SchemaObject>(3).add());
        TypeSerializer::<generated::improbable::ComponentInterest_RelativeSphereConstraint>::serialize(input.relative_sphere_constraint, &mut output.field::<SchemaObject>(4).add());
        TypeSerializer::<generated::improbable::ComponentInterest_RelativeCylinderConstraint>::serialize(input.relative_cylinder_constraint, &mut output.field::<SchemaObject>(5).add());
        TypeSerializer::<generated::improbable::ComponentInterest_RelativeBoxConstraint>::serialize(input.relative_box_constraint, &mut output.field::<SchemaObject>(6).add());
        output.field::<i64>(7).add(input.entity_id_constraint);
        output.field::<u32>(8).add(input.component_constraint);
        for element in input.and_constraint.iter() { TypeSerializer::<generated::improbable::ComponentInterest_QueryConstraint>::serialize(element, &mut output.field::<SchemaObject>(9).add()); };
        for element in input.or_constraint.iter() { TypeSerializer::<generated::improbable::ComponentInterest_QueryConstraint>::serialize(element, &mut output.field::<SchemaObject>(10).add()); };
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            sphere_constraint: input.field::<SchemaObject>(1).get().map(|v| { TypeSerializer::<generated::improbable::ComponentInterest_SphereConstraint>::deserialize(v) }),
            cylinder_constraint: input.field::<SchemaObject>(2).get().map(|v| { TypeSerializer::<generated::improbable::ComponentInterest_CylinderConstraint>::deserialize(v) }),
            box_constraint: input.field::<SchemaObject>(3).get().map(|v| { TypeSerializer::<generated::improbable::ComponentInterest_BoxConstraint>::deserialize(v) }),
            relative_sphere_constraint: input.field::<SchemaObject>(4).get().map(|v| { TypeSerializer::<generated::improbable::ComponentInterest_RelativeSphereConstraint>::deserialize(v) }),
            relative_cylinder_constraint: input.field::<SchemaObject>(5).get().map(|v| { TypeSerializer::<generated::improbable::ComponentInterest_RelativeCylinderConstraint>::deserialize(v) }),
            relative_box_constraint: input.field::<SchemaObject>(6).get().map(|v| { TypeSerializer::<generated::improbable::ComponentInterest_RelativeBoxConstraint>::deserialize(v) }),
            entity_id_constraint: input.field::<i64>(7).get().map(|v| { v }),
            component_constraint: input.field::<u32>(8).get().map(|v| { v }),
            and_constraint: { let size = input.field::<SchemaObject>(9).count(); let mut l = Vec::with_capacity(size); for i in 0..size { l.push(TypeSerializer::<generated::improbable::ComponentInterest_QueryConstraint>::deserialize(input.field::<SchemaObject>(9).index(i))); }; l },
            or_constraint: { let size = input.field::<SchemaObject>(10).count(); let mut l = Vec::with_capacity(size); for i in 0..size { l.push(TypeSerializer::<generated::improbable::ComponentInterest_QueryConstraint>::deserialize(input.field::<SchemaObject>(10).index(i))); }; l },
        })
    }
}

#[derive(Debug)]
pub struct ComponentInterest_RelativeBoxConstraint {
    edge_length: generated::improbable::EdgeLength,
}
impl TypeSerializer<ComponentInterest_RelativeBoxConstraint> for ComponentInterest_RelativeBoxConstraint {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        TypeSerializer::<generated::improbable::EdgeLength>::serialize(input.edge_length, &mut output.field::<SchemaObject>(1).add());
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            edge_length: TypeSerializer::<generated::improbable::EdgeLength>::deserialize(input.field::<SchemaObject>(1).get_or_default()),
        })
    }
}

#[derive(Debug)]
pub struct ComponentInterest_RelativeCylinderConstraint {
    radius: f64,
}
impl TypeSerializer<ComponentInterest_RelativeCylinderConstraint> for ComponentInterest_RelativeCylinderConstraint {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        output.field::<f64>(1).add(input.radius);
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            radius: input.field::<f64>(1).get_or_default(),
        })
    }
}

#[derive(Debug)]
pub struct ComponentInterest_RelativeSphereConstraint {
    radius: f64,
}
impl TypeSerializer<ComponentInterest_RelativeSphereConstraint> for ComponentInterest_RelativeSphereConstraint {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        output.field::<f64>(1).add(input.radius);
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            radius: input.field::<f64>(1).get_or_default(),
        })
    }
}

#[derive(Debug)]
pub struct ComponentInterest_SphereConstraint {
    center: generated::improbable::Coordinates,
    radius: f64,
}
impl TypeSerializer<ComponentInterest_SphereConstraint> for ComponentInterest_SphereConstraint {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        TypeSerializer::<generated::improbable::Coordinates>::serialize(input.center, &mut output.field::<SchemaObject>(1).add());
        output.field::<f64>(2).add(input.radius);
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            center: TypeSerializer::<generated::improbable::Coordinates>::deserialize(input.field::<SchemaObject>(1).get_or_default()),
            radius: input.field::<f64>(2).get_or_default(),
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
        output.field::<f64>(1).add(input.x);
        output.field::<f64>(2).add(input.y);
        output.field::<f64>(3).add(input.z);
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
        output.field::<f64>(1).add(input.x);
        output.field::<f64>(2).add(input.y);
        output.field::<f64>(3).add(input.z);
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
pub struct Vector3d {
    x: f64,
    y: f64,
    z: f64,
}
impl TypeSerializer<Vector3d> for Vector3d {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        output.field::<f64>(1).add(input.x);
        output.field::<f64>(2).add(input.y);
        output.field::<f64>(3).add(input.z);
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
        output.field::<f32>(1).add(input.x);
        output.field::<f32>(2).add(input.y);
        output.field::<f32>(3).add(input.z);
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
        output.field::<String>(1).add_list(input.attribute[..]);
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            attribute: { let size = input.field::<String>(1).count(); let mut l = Vec::with_capacity(size); for i in 0..size { l.push(input.field::<String>(1).index(i)); }; l },
        })
    }
}

#[derive(Debug)]
pub struct WorkerRequirementSet {
    attribute_set: Vec<generated::improbable::WorkerAttributeSet>,
}
impl TypeSerializer<WorkerRequirementSet> for WorkerRequirementSet {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        for element in input.attribute_set.iter() { TypeSerializer::<generated::improbable::WorkerAttributeSet>::serialize(element, &mut output.field::<SchemaObject>(1).add()); };
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            attribute_set: { let size = input.field::<SchemaObject>(1).count(); let mut l = Vec::with_capacity(size); for i in 0..size { l.push(TypeSerializer::<generated::improbable::WorkerAttributeSet>::deserialize(input.field::<SchemaObject>(1).index(i))); }; l },
        })
    }
}

/* Components. */ 
#[derive(Debug)]
pub struct EntityAcl {
    read_acl: generated::improbable::WorkerRequirementSet,
    component_write_acl: BTreeMap<u32, generated::improbable::WorkerRequirementSet>,
}
impl TypeSerializer<EntityAcl> for EntityAcl {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        TypeSerializer::<generated::improbable::WorkerRequirementSet>::serialize(input.read_acl, &mut output.field::<SchemaObject>(1).add());
        for (k, v) in input.component_write_acl { let object = output.field::<SchemaObject>(2).add(); object.field::<u32>(1).add(k); TypeSerializer::<generated::improbable::WorkerRequirementSet>::serialize(v, &mut object.field::<SchemaObject>(2).add()); };
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            read_acl: TypeSerializer::<generated::improbable::WorkerRequirementSet>::deserialize(input.field::<SchemaObject>(1).get_or_default()),
            component_write_acl: { let size = input.field::<SchemaObject>(2).count(); let mut m = BTreeMap::new(); for i in 0..size { let kv = input.field::<SchemaObject>(2).index(i); m.insert(kv.field::<SchemaObject>(1), TypeSerializer::<generated::improbable::WorkerRequirementSet>::deserialize(kv.field::<SchemaObject>(2))); }; m },
        })
    }
}
impl ComponentData<EntityAcl> for EntityAcl {
    fn merge(&mut self, update: EntityAclUpdate) {
        if let Some(value) = update.read_acl { self.read_acl = value; }
        if let Some(value) = update.component_write_acl { self.component_write_acl = value; }
    }
}

#[derive(Debug)]
pub struct EntityAclUpdate {
    read_acl: Option<generated::improbable::WorkerRequirementSet>,
    component_write_acl: Option<BTreeMap<u32, generated::improbable::WorkerRequirementSet>>,
}
impl TypeSerializer<EntityAclUpdate> for EntityAclUpdate {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        if let Some(ref value) = input.read_acl {
            TypeSerializer::<generated::improbable::WorkerRequirementSet>::serialize(value.clone(), &mut output.field::<SchemaObject>(1).add());
        }
        if let Some(ref value) = input.component_write_acl {
            for (k, v) in value.clone() { let object = output.field::<SchemaObject>(2).add(); object.field::<u32>(1).add(k); TypeSerializer::<generated::improbable::WorkerRequirementSet>::serialize(v, &mut object.field::<SchemaObject>(2).add()); };
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
            output.read_acl = Some(TypeSerializer::<generated::improbable::WorkerRequirementSet>::deserialize(field.get_or_default()));
        }
        let _field_component_write_acl = input.field::<SchemaObject>(2);
        if _field_component_write_acl.count() > 0 {
            let field = &_field_component_write_acl;
            output.component_write_acl = Some({ let size = field.count(); let mut m = BTreeMap::new(); for i in 0..size { let kv = field.index(i); m.insert(kv.field::<SchemaObject>(1), TypeSerializer::<generated::improbable::WorkerRequirementSet>::deserialize(kv.field::<SchemaObject>(2))); }; m });
        }
        Ok(output)
    }
}
impl ComponentUpdate<EntityAcl> for EntityAclUpdate {
    fn merge(&mut self, update: EntityAclUpdate) {
        if update.read_acl.is_some() { self.read_acl = update.read_acl; }
        if update.component_write_acl.is_some() { self.component_write_acl = update.component_write_acl; }
    }
}

impl ComponentMetaclass for EntityAcl {
    type Data = generated::improbable::EntityAcl;
    type Update = generated::improbable::EntityAclUpdate;
    type CommandRequest = generated::improbable::EntityAclCommandRequest;
    type CommandResponse = generated::improbable::EntityAclCommandResponse;

    fn component_id() -> ComponentId {
        50
    }
}

#[derive(Debug)]
pub enum EntityAclCommandRequest {
}

#[derive(Debug)]
pub enum EntityAclCommandResponse {
}

impl ComponentVtable<EntityAcl> for EntityAcl {
    fn serialize_data(data: &generated::improbable::EntityAcl) -> Result<schema::SchemaComponentData, String> {
        let mut serialized_data = schema::SchemaComponentData::new(Self::component_id());
        TypeSerializer::<generated::improbable::EntityAcl>::serialize(data, &mut serialized_data.fields_mut());
        Ok(serialized_data)
    }

    fn deserialize_data(data: &schema::SchemaComponentData) -> Result<generated::improbable::EntityAcl, String> {
        TypeSerializer::<generated::improbable::EntityAcl>::deserialize(&data.fields())
    }

    fn serialize_update(update: &generated::improbable::EntityAclUpdate) -> Result<schema::SchemaComponentUpdate, String> {
        let mut serialized_update = schema::SchemaComponentUpdate::new(Self::component_id());
        TypeSerializer::<generated::improbable::EntityAclUpdate>::serialize(update, &mut serialized_update.fields_mut());
        Ok(serialized_update)
    }

    fn deserialize_update(update: &schema::SchemaComponentUpdate) -> Result<generated::improbable::EntityAclUpdate, String> {
        TypeSerializer::<generated::improbable::EntityAclUpdate>::deserialize(&update.fields())
    }

    fn serialize_command_request(request: &generated::improbable::EntityAclCommandRequest) -> Result<schema::SchemaCommandRequest, String> {
        let command_index = match request {
            _ => unreachable!()
        };
        let mut serialized_request = schema::SchemaCommandRequest::new(Self::component_id(), command_index);
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn deserialize_command_request(request: &schema::SchemaCommandRequest) -> Result<generated::improbable::EntityAclCommandRequest, String> {
        match request.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component EntityAcl.", request.command_index()))
        }
    }

    fn serialize_command_response(response: &generated::improbable::EntityAclCommandResponse) -> Result<schema::SchemaCommandResponse, String> {
        let command_index = match response {
            _ => unreachable!()
        };
        let mut serialized_response = schema::SchemaCommandResponse::new(Self::component_id(), command_index);
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn deserialize_command_response(response: &schema::SchemaCommandResponse) -> Result<generated::improbable::EntityAclCommandResponse, String> {
        match response.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component EntityAcl.", response.command_index()))
        }
    }
}

#[derive(Debug)]
pub struct Interest {
    component_interest: BTreeMap<u32, generated::improbable::ComponentInterest>,
}
impl TypeSerializer<Interest> for Interest {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        for (k, v) in input.component_interest { let object = output.field::<SchemaObject>(1).add(); object.field::<u32>(1).add(k); TypeSerializer::<generated::improbable::ComponentInterest>::serialize(v, &mut object.field::<SchemaObject>(2).add()); };
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            component_interest: { let size = input.field::<SchemaObject>(1).count(); let mut m = BTreeMap::new(); for i in 0..size { let kv = input.field::<SchemaObject>(1).index(i); m.insert(kv.field::<SchemaObject>(1), TypeSerializer::<generated::improbable::ComponentInterest>::deserialize(kv.field::<SchemaObject>(2))); }; m },
        })
    }
}
impl ComponentData<Interest> for Interest {
    fn merge(&mut self, update: InterestUpdate) {
        if let Some(value) = update.component_interest { self.component_interest = value; }
    }
}

#[derive(Debug)]
pub struct InterestUpdate {
    component_interest: Option<BTreeMap<u32, generated::improbable::ComponentInterest>>,
}
impl TypeSerializer<InterestUpdate> for InterestUpdate {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        if let Some(ref value) = input.component_interest {
            for (k, v) in value.clone() { let object = output.field::<SchemaObject>(1).add(); object.field::<u32>(1).add(k); TypeSerializer::<generated::improbable::ComponentInterest>::serialize(v, &mut object.field::<SchemaObject>(2).add()); };
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
            output.component_interest = Some({ let size = field.count(); let mut m = BTreeMap::new(); for i in 0..size { let kv = field.index(i); m.insert(kv.field::<SchemaObject>(1), TypeSerializer::<generated::improbable::ComponentInterest>::deserialize(kv.field::<SchemaObject>(2))); }; m });
        }
        Ok(output)
    }
}
impl ComponentUpdate<Interest> for InterestUpdate {
    fn merge(&mut self, update: InterestUpdate) {
        if update.component_interest.is_some() { self.component_interest = update.component_interest; }
    }
}

impl ComponentMetaclass for Interest {
    type Data = generated::improbable::Interest;
    type Update = generated::improbable::InterestUpdate;
    type CommandRequest = generated::improbable::InterestCommandRequest;
    type CommandResponse = generated::improbable::InterestCommandResponse;

    fn component_id() -> ComponentId {
        58
    }
}

#[derive(Debug)]
pub enum InterestCommandRequest {
}

#[derive(Debug)]
pub enum InterestCommandResponse {
}

impl ComponentVtable<Interest> for Interest {
    fn serialize_data(data: &generated::improbable::Interest) -> Result<schema::SchemaComponentData, String> {
        let mut serialized_data = schema::SchemaComponentData::new(Self::component_id());
        TypeSerializer::<generated::improbable::Interest>::serialize(data, &mut serialized_data.fields_mut());
        Ok(serialized_data)
    }

    fn deserialize_data(data: &schema::SchemaComponentData) -> Result<generated::improbable::Interest, String> {
        TypeSerializer::<generated::improbable::Interest>::deserialize(&data.fields())
    }

    fn serialize_update(update: &generated::improbable::InterestUpdate) -> Result<schema::SchemaComponentUpdate, String> {
        let mut serialized_update = schema::SchemaComponentUpdate::new(Self::component_id());
        TypeSerializer::<generated::improbable::InterestUpdate>::serialize(update, &mut serialized_update.fields_mut());
        Ok(serialized_update)
    }

    fn deserialize_update(update: &schema::SchemaComponentUpdate) -> Result<generated::improbable::InterestUpdate, String> {
        TypeSerializer::<generated::improbable::InterestUpdate>::deserialize(&update.fields())
    }

    fn serialize_command_request(request: &generated::improbable::InterestCommandRequest) -> Result<schema::SchemaCommandRequest, String> {
        let command_index = match request {
            _ => unreachable!()
        };
        let mut serialized_request = schema::SchemaCommandRequest::new(Self::component_id(), command_index);
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn deserialize_command_request(request: &schema::SchemaCommandRequest) -> Result<generated::improbable::InterestCommandRequest, String> {
        match request.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Interest.", request.command_index()))
        }
    }

    fn serialize_command_response(response: &generated::improbable::InterestCommandResponse) -> Result<schema::SchemaCommandResponse, String> {
        let command_index = match response {
            _ => unreachable!()
        };
        let mut serialized_response = schema::SchemaCommandResponse::new(Self::component_id(), command_index);
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn deserialize_command_response(response: &schema::SchemaCommandResponse) -> Result<generated::improbable::InterestCommandResponse, String> {
        match response.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Interest.", response.command_index()))
        }
    }
}

#[derive(Debug)]
pub struct Metadata {
    entity_type: String,
}
impl TypeSerializer<Metadata> for Metadata {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        output.field::<String>(1).add(input.entity_type);
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            entity_type: input.field::<String>(1).get_or_default(),
        })
    }
}
impl ComponentData<Metadata> for Metadata {
    fn merge(&mut self, update: MetadataUpdate) {
        if let Some(value) = update.entity_type { self.entity_type = value; }
    }
}

#[derive(Debug)]
pub struct MetadataUpdate {
    entity_type: Option<String>,
}
impl TypeSerializer<MetadataUpdate> for MetadataUpdate {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        if let Some(ref value) = input.entity_type {
            output.field::<String>(1).add(value.clone());
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
            output.entity_type = Some(field.get_or_default());
        }
        Ok(output)
    }
}
impl ComponentUpdate<Metadata> for MetadataUpdate {
    fn merge(&mut self, update: MetadataUpdate) {
        if update.entity_type.is_some() { self.entity_type = update.entity_type; }
    }
}

impl ComponentMetaclass for Metadata {
    type Data = generated::improbable::Metadata;
    type Update = generated::improbable::MetadataUpdate;
    type CommandRequest = generated::improbable::MetadataCommandRequest;
    type CommandResponse = generated::improbable::MetadataCommandResponse;

    fn component_id() -> ComponentId {
        53
    }
}

#[derive(Debug)]
pub enum MetadataCommandRequest {
}

#[derive(Debug)]
pub enum MetadataCommandResponse {
}

impl ComponentVtable<Metadata> for Metadata {
    fn serialize_data(data: &generated::improbable::Metadata) -> Result<schema::SchemaComponentData, String> {
        let mut serialized_data = schema::SchemaComponentData::new(Self::component_id());
        TypeSerializer::<generated::improbable::Metadata>::serialize(data, &mut serialized_data.fields_mut());
        Ok(serialized_data)
    }

    fn deserialize_data(data: &schema::SchemaComponentData) -> Result<generated::improbable::Metadata, String> {
        TypeSerializer::<generated::improbable::Metadata>::deserialize(&data.fields())
    }

    fn serialize_update(update: &generated::improbable::MetadataUpdate) -> Result<schema::SchemaComponentUpdate, String> {
        let mut serialized_update = schema::SchemaComponentUpdate::new(Self::component_id());
        TypeSerializer::<generated::improbable::MetadataUpdate>::serialize(update, &mut serialized_update.fields_mut());
        Ok(serialized_update)
    }

    fn deserialize_update(update: &schema::SchemaComponentUpdate) -> Result<generated::improbable::MetadataUpdate, String> {
        TypeSerializer::<generated::improbable::MetadataUpdate>::deserialize(&update.fields())
    }

    fn serialize_command_request(request: &generated::improbable::MetadataCommandRequest) -> Result<schema::SchemaCommandRequest, String> {
        let command_index = match request {
            _ => unreachable!()
        };
        let mut serialized_request = schema::SchemaCommandRequest::new(Self::component_id(), command_index);
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn deserialize_command_request(request: &schema::SchemaCommandRequest) -> Result<generated::improbable::MetadataCommandRequest, String> {
        match request.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Metadata.", request.command_index()))
        }
    }

    fn serialize_command_response(response: &generated::improbable::MetadataCommandResponse) -> Result<schema::SchemaCommandResponse, String> {
        let command_index = match response {
            _ => unreachable!()
        };
        let mut serialized_response = schema::SchemaCommandResponse::new(Self::component_id(), command_index);
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn deserialize_command_response(response: &schema::SchemaCommandResponse) -> Result<generated::improbable::MetadataCommandResponse, String> {
        match response.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Metadata.", response.command_index()))
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
impl ComponentData<Persistence> for Persistence {
    fn merge(&mut self, update: PersistenceUpdate) {
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
impl ComponentUpdate<Persistence> for PersistenceUpdate {
    fn merge(&mut self, update: PersistenceUpdate) {
    }
}

impl ComponentMetaclass for Persistence {
    type Data = generated::improbable::Persistence;
    type Update = generated::improbable::PersistenceUpdate;
    type CommandRequest = generated::improbable::PersistenceCommandRequest;
    type CommandResponse = generated::improbable::PersistenceCommandResponse;

    fn component_id() -> ComponentId {
        55
    }
}

#[derive(Debug)]
pub enum PersistenceCommandRequest {
}

#[derive(Debug)]
pub enum PersistenceCommandResponse {
}

impl ComponentVtable<Persistence> for Persistence {
    fn serialize_data(data: &generated::improbable::Persistence) -> Result<schema::SchemaComponentData, String> {
        let mut serialized_data = schema::SchemaComponentData::new(Self::component_id());
        TypeSerializer::<generated::improbable::Persistence>::serialize(data, &mut serialized_data.fields_mut());
        Ok(serialized_data)
    }

    fn deserialize_data(data: &schema::SchemaComponentData) -> Result<generated::improbable::Persistence, String> {
        TypeSerializer::<generated::improbable::Persistence>::deserialize(&data.fields())
    }

    fn serialize_update(update: &generated::improbable::PersistenceUpdate) -> Result<schema::SchemaComponentUpdate, String> {
        let mut serialized_update = schema::SchemaComponentUpdate::new(Self::component_id());
        TypeSerializer::<generated::improbable::PersistenceUpdate>::serialize(update, &mut serialized_update.fields_mut());
        Ok(serialized_update)
    }

    fn deserialize_update(update: &schema::SchemaComponentUpdate) -> Result<generated::improbable::PersistenceUpdate, String> {
        TypeSerializer::<generated::improbable::PersistenceUpdate>::deserialize(&update.fields())
    }

    fn serialize_command_request(request: &generated::improbable::PersistenceCommandRequest) -> Result<schema::SchemaCommandRequest, String> {
        let command_index = match request {
            _ => unreachable!()
        };
        let mut serialized_request = schema::SchemaCommandRequest::new(Self::component_id(), command_index);
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn deserialize_command_request(request: &schema::SchemaCommandRequest) -> Result<generated::improbable::PersistenceCommandRequest, String> {
        match request.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Persistence.", request.command_index()))
        }
    }

    fn serialize_command_response(response: &generated::improbable::PersistenceCommandResponse) -> Result<schema::SchemaCommandResponse, String> {
        let command_index = match response {
            _ => unreachable!()
        };
        let mut serialized_response = schema::SchemaCommandResponse::new(Self::component_id(), command_index);
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn deserialize_command_response(response: &schema::SchemaCommandResponse) -> Result<generated::improbable::PersistenceCommandResponse, String> {
        match response.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Persistence.", response.command_index()))
        }
    }
}

#[derive(Debug)]
pub struct Position {
    coords: generated::improbable::Coordinates,
}
impl TypeSerializer<Position> for Position {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        TypeSerializer::<generated::improbable::Coordinates>::serialize(input.coords, &mut output.field::<SchemaObject>(1).add());
        Ok(())
    }
    fn deserialize(input: &schema::SchemaObject) -> Result<Self, String> {
        Ok(Self {
            coords: TypeSerializer::<generated::improbable::Coordinates>::deserialize(input.field::<SchemaObject>(1).get_or_default()),
        })
    }
}
impl ComponentData<Position> for Position {
    fn merge(&mut self, update: PositionUpdate) {
        if let Some(value) = update.coords { self.coords = value; }
    }
}

#[derive(Debug)]
pub struct PositionUpdate {
    coords: Option<generated::improbable::Coordinates>,
}
impl TypeSerializer<PositionUpdate> for PositionUpdate {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) -> Result<(), String> {
        if let Some(ref value) = input.coords {
            TypeSerializer::<generated::improbable::Coordinates>::serialize(value.clone(), &mut output.field::<SchemaObject>(1).add());
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
            output.coords = Some(TypeSerializer::<generated::improbable::Coordinates>::deserialize(field.get_or_default()));
        }
        Ok(output)
    }
}
impl ComponentUpdate<Position> for PositionUpdate {
    fn merge(&mut self, update: PositionUpdate) {
        if update.coords.is_some() { self.coords = update.coords; }
    }
}

impl ComponentMetaclass for Position {
    type Data = generated::improbable::Position;
    type Update = generated::improbable::PositionUpdate;
    type CommandRequest = generated::improbable::PositionCommandRequest;
    type CommandResponse = generated::improbable::PositionCommandResponse;

    fn component_id() -> ComponentId {
        54
    }
}

#[derive(Debug)]
pub enum PositionCommandRequest {
}

#[derive(Debug)]
pub enum PositionCommandResponse {
}

impl ComponentVtable<Position> for Position {
    fn serialize_data(data: &generated::improbable::Position) -> Result<schema::SchemaComponentData, String> {
        let mut serialized_data = schema::SchemaComponentData::new(Self::component_id());
        TypeSerializer::<generated::improbable::Position>::serialize(data, &mut serialized_data.fields_mut());
        Ok(serialized_data)
    }

    fn deserialize_data(data: &schema::SchemaComponentData) -> Result<generated::improbable::Position, String> {
        TypeSerializer::<generated::improbable::Position>::deserialize(&data.fields())
    }

    fn serialize_update(update: &generated::improbable::PositionUpdate) -> Result<schema::SchemaComponentUpdate, String> {
        let mut serialized_update = schema::SchemaComponentUpdate::new(Self::component_id());
        TypeSerializer::<generated::improbable::PositionUpdate>::serialize(update, &mut serialized_update.fields_mut());
        Ok(serialized_update)
    }

    fn deserialize_update(update: &schema::SchemaComponentUpdate) -> Result<generated::improbable::PositionUpdate, String> {
        TypeSerializer::<generated::improbable::PositionUpdate>::deserialize(&update.fields())
    }

    fn serialize_command_request(request: &generated::improbable::PositionCommandRequest) -> Result<schema::SchemaCommandRequest, String> {
        let command_index = match request {
            _ => unreachable!()
        };
        let mut serialized_request = schema::SchemaCommandRequest::new(Self::component_id(), command_index);
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn deserialize_command_request(request: &schema::SchemaCommandRequest) -> Result<generated::improbable::PositionCommandRequest, String> {
        match request.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Position.", request.command_index()))
        }
    }

    fn serialize_command_response(response: &generated::improbable::PositionCommandResponse) -> Result<schema::SchemaCommandResponse, String> {
        let command_index = match response {
            _ => unreachable!()
        };
        let mut serialized_response = schema::SchemaCommandResponse::new(Self::component_id(), command_index);
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn deserialize_command_response(response: &schema::SchemaCommandResponse) -> Result<generated::improbable::PositionCommandResponse, String> {
        match response.command_index() {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Position.", response.command_index()))
        }
    }
}

}
}

pub use self::generated::example as example;
pub use self::generated::improbable as improbable;