#![allow(unused_imports)]
#![allow(unreachable_code)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_mut)]

use spatialos_sdk::worker::schema::*;
use spatialos_sdk::worker::component::*;
use std::{collections::BTreeMap, convert::TryFrom};

use super::generated as generated;

/* Enums. */
/* Types. */
/* Components. */ 


pub mod example {
use spatialos_sdk::worker::schema::*;
use spatialos_sdk::worker::component::*;
use std::{collections::BTreeMap, convert::TryFrom};

use super::super::generated as generated;

/* Enums. */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TestEnum {

    FIRST,
    SECOND,
}

impl EnumField for TestEnum {}

impl Default for TestEnum {
    fn default() -> Self {
        TestEnum::FIRST
    }
}

impl TryFrom<u32> for TestEnum {
    type Error = UnknownDiscriminantError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            
            0 => Ok(TestEnum::FIRST), 
            1 => Ok(TestEnum::SECOND), 
            _ => Err(UnknownDiscriminantError {
                type_name: std::any::type_name::<Self>(),
                value,
            }),
        }
    }
}

impl Into<u32> for TestEnum {
    fn into(self) -> u32 {
        match self {
            
            TestEnum::FIRST => 0, 
            TestEnum::SECOND => 1, 
        }
    }
}

impl_field_for_enum_field!(TestEnum);

/* Types. */
#[derive(Debug, Clone)]
pub struct CommandData {
    pub value: i32,
}
impl ObjectField for CommandData {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            value: input.get::<SchemaInt32>(1),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaInt32>(1, &self.value);
    }
}

#[derive(Debug, Clone)]
pub struct TestType {
    pub value: i32,
}
impl ObjectField for TestType {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            value: input.get::<SchemaInt32>(1),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaInt32>(1, &self.value);
    }
}

#[derive(Debug, Clone)]
pub struct TestType_Inner {
    pub number: f32,
}
impl ObjectField for TestType_Inner {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            number: input.get::<SchemaFloat>(2),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaFloat>(2, &self.number);
    }
}

#[derive(Debug, Clone)]
pub struct Vector3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl ObjectField for Vector3d {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            x: input.get::<SchemaDouble>(1),
            y: input.get::<SchemaDouble>(2),
            z: input.get::<SchemaDouble>(3),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaDouble>(1, &self.x);
        output.add::<SchemaDouble>(2, &self.y);
        output.add::<SchemaDouble>(3, &self.z);
    }
}

/* Components. */ 
#[derive(Debug, Clone)]
pub struct EntityIdTest {
    pub eid: spatialos_sdk::worker::EntityId,
}
impl ObjectField for EntityIdTest {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            eid: input.get::<SchemaEntityId>(1),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaEntityId>(1, &self.eid);
    }
}
impl ComponentData<EntityIdTest> for EntityIdTest {
    fn merge(&mut self, update: EntityIdTestUpdate) {
        if let Some(value) = update.eid { self.eid = value; }
    }
}

#[derive(Debug, Clone, Default)]
pub struct EntityIdTestUpdate {
    pub eid: Option<spatialos_sdk::worker::EntityId>,
}
impl ObjectField for EntityIdTestUpdate {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            eid: if input.count::<SchemaEntityId>(1) > 0 { Some(input.get::<SchemaEntityId>(1)) } else { None },
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        if let Some(value) = &self.eid {
            output.add::<SchemaEntityId>(1, value);
        }
    }
}
impl ComponentUpdate<EntityIdTest> for EntityIdTestUpdate {
    fn merge(&mut self, update: EntityIdTestUpdate) {
        if update.eid.is_some() { self.eid = update.eid; }
    }
}

#[derive(Debug, Clone)]
pub enum EntityIdTestCommandRequest {
}

#[derive(Debug, Clone)]
pub enum EntityIdTestCommandResponse {
}

impl Component for EntityIdTest {
    type Update = generated::example::EntityIdTestUpdate;
    type CommandRequest = generated::example::EntityIdTestCommandRequest;
    type CommandResponse = generated::example::EntityIdTestCommandResponse;

    const ID: ComponentId = 2001;

    fn from_data(data: &SchemaComponentData) -> Result<generated::example::EntityIdTest, String> {
        Ok(<generated::example::EntityIdTest as ObjectField>::from_object(&data.fields()))
    }

    fn from_update(update: &SchemaComponentUpdate) -> Result<generated::example::EntityIdTestUpdate, String> {
        Ok(<generated::example::EntityIdTestUpdate as ObjectField>::from_object(&update.fields()))
    }

    fn from_request(command_index: CommandIndex, request: &SchemaCommandRequest) -> Result<generated::example::EntityIdTestCommandRequest, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component EntityIdTest.", command_index))
        }
    }

    fn from_response(command_index: CommandIndex, response: &SchemaCommandResponse) -> Result<generated::example::EntityIdTestCommandResponse, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component EntityIdTest.", command_index))
        }
    }

    fn to_data(data: &generated::example::EntityIdTest) -> Result<Owned<SchemaComponentData>, String> {
        let mut serialized_data = SchemaComponentData::new();
        <generated::example::EntityIdTest as ObjectField>::into_object(data, &mut serialized_data.fields_mut());
        Ok(serialized_data)
    }

    fn to_update(update: &generated::example::EntityIdTestUpdate) -> Result<Owned<SchemaComponentUpdate>, String> {
        let mut serialized_update = SchemaComponentUpdate::new();
        <generated::example::EntityIdTestUpdate as ObjectField>::into_object(update, &mut serialized_update.fields_mut());
        Ok(serialized_update)
    }

    fn to_request(request: &generated::example::EntityIdTestCommandRequest) -> Result<Owned<SchemaCommandRequest>, String> {
        let mut serialized_request = SchemaCommandRequest::new();
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn to_response(response: &generated::example::EntityIdTestCommandResponse) -> Result<Owned<SchemaCommandResponse>, String> {
        let mut serialized_response = SchemaCommandResponse::new();
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn get_request_command_index(request: &generated::example::EntityIdTestCommandRequest) -> u32 {
        match request {
            _ => unreachable!(),
        }
    }

    fn get_response_command_index(response: &generated::example::EntityIdTestCommandResponse) -> u32 {
        match response {
            _ => unreachable!(),
        }
    }
}

inventory::submit!(VTable::new::<EntityIdTest>());

#[derive(Debug, Clone)]
pub struct EnumTestComponent {
    pub test: generated::example::TestEnum,
}
impl ObjectField for EnumTestComponent {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            test: input.get::<generated::example::TestEnum>(1),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<generated::example::TestEnum>(1, &self.test);
    }
}
impl ComponentData<EnumTestComponent> for EnumTestComponent {
    fn merge(&mut self, update: EnumTestComponentUpdate) {
        if let Some(value) = update.test { self.test = value; }
    }
}

#[derive(Debug, Clone, Default)]
pub struct EnumTestComponentUpdate {
    pub test: Option<generated::example::TestEnum>,
}
impl ObjectField for EnumTestComponentUpdate {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            test: if input.count::<SchemaEnum>(1) > 0 { Some(input.get::<generated::example::TestEnum>(1)) } else { None },
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        if let Some(value) = &self.test {
            output.add::<generated::example::TestEnum>(1, value);
        }
    }
}
impl ComponentUpdate<EnumTestComponent> for EnumTestComponentUpdate {
    fn merge(&mut self, update: EnumTestComponentUpdate) {
        if update.test.is_some() { self.test = update.test; }
    }
}

#[derive(Debug, Clone)]
pub enum EnumTestComponentCommandRequest {
}

#[derive(Debug, Clone)]
pub enum EnumTestComponentCommandResponse {
}

impl Component for EnumTestComponent {
    type Update = generated::example::EnumTestComponentUpdate;
    type CommandRequest = generated::example::EnumTestComponentCommandRequest;
    type CommandResponse = generated::example::EnumTestComponentCommandResponse;

    const ID: ComponentId = 2002;

    fn from_data(data: &SchemaComponentData) -> Result<generated::example::EnumTestComponent, String> {
        Ok(<generated::example::EnumTestComponent as ObjectField>::from_object(&data.fields()))
    }

    fn from_update(update: &SchemaComponentUpdate) -> Result<generated::example::EnumTestComponentUpdate, String> {
        Ok(<generated::example::EnumTestComponentUpdate as ObjectField>::from_object(&update.fields()))
    }

    fn from_request(command_index: CommandIndex, request: &SchemaCommandRequest) -> Result<generated::example::EnumTestComponentCommandRequest, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component EnumTestComponent.", command_index))
        }
    }

    fn from_response(command_index: CommandIndex, response: &SchemaCommandResponse) -> Result<generated::example::EnumTestComponentCommandResponse, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component EnumTestComponent.", command_index))
        }
    }

    fn to_data(data: &generated::example::EnumTestComponent) -> Result<Owned<SchemaComponentData>, String> {
        let mut serialized_data = SchemaComponentData::new();
        <generated::example::EnumTestComponent as ObjectField>::into_object(data, &mut serialized_data.fields_mut());
        Ok(serialized_data)
    }

    fn to_update(update: &generated::example::EnumTestComponentUpdate) -> Result<Owned<SchemaComponentUpdate>, String> {
        let mut serialized_update = SchemaComponentUpdate::new();
        <generated::example::EnumTestComponentUpdate as ObjectField>::into_object(update, &mut serialized_update.fields_mut());
        Ok(serialized_update)
    }

    fn to_request(request: &generated::example::EnumTestComponentCommandRequest) -> Result<Owned<SchemaCommandRequest>, String> {
        let mut serialized_request = SchemaCommandRequest::new();
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn to_response(response: &generated::example::EnumTestComponentCommandResponse) -> Result<Owned<SchemaCommandResponse>, String> {
        let mut serialized_response = SchemaCommandResponse::new();
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn get_request_command_index(request: &generated::example::EnumTestComponentCommandRequest) -> u32 {
        match request {
            _ => unreachable!(),
        }
    }

    fn get_response_command_index(response: &generated::example::EnumTestComponentCommandResponse) -> u32 {
        match response {
            _ => unreachable!(),
        }
    }
}

inventory::submit!(VTable::new::<EnumTestComponent>());

#[derive(Debug, Clone)]
pub struct Example {
    pub x: f32,
}
impl ObjectField for Example {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            x: input.get::<SchemaFloat>(1),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaFloat>(1, &self.x);
    }
}
impl ComponentData<Example> for Example {
    fn merge(&mut self, update: ExampleUpdate) {
        if let Some(value) = update.x { self.x = value; }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ExampleUpdate {
    pub x: Option<f32>,
}
impl ObjectField for ExampleUpdate {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            x: if input.count::<SchemaFloat>(1) > 0 { Some(input.get::<SchemaFloat>(1)) } else { None },
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        if let Some(value) = &self.x {
            output.add::<SchemaFloat>(1, value);
        }
    }
}
impl ComponentUpdate<Example> for ExampleUpdate {
    fn merge(&mut self, update: ExampleUpdate) {
        if update.x.is_some() { self.x = update.x; }
    }
}

#[derive(Debug, Clone)]
pub enum ExampleCommandRequest {
    TestCommand(generated::example::CommandData),
}

#[derive(Debug, Clone)]
pub enum ExampleCommandResponse {
    TestCommand(generated::example::CommandData),
}

impl Component for Example {
    type Update = generated::example::ExampleUpdate;
    type CommandRequest = generated::example::ExampleCommandRequest;
    type CommandResponse = generated::example::ExampleCommandResponse;

    const ID: ComponentId = 1000;

    fn from_data(data: &SchemaComponentData) -> Result<generated::example::Example, String> {
        Ok(<generated::example::Example as ObjectField>::from_object(&data.fields()))
    }

    fn from_update(update: &SchemaComponentUpdate) -> Result<generated::example::ExampleUpdate, String> {
        Ok(<generated::example::ExampleUpdate as ObjectField>::from_object(&update.fields()))
    }

    fn from_request(command_index: CommandIndex, request: &SchemaCommandRequest) -> Result<generated::example::ExampleCommandRequest, String> {
        match command_index {
            1 => {
                let deserialized = <generated::example::CommandData as ObjectField>::from_object(&request.object());
                Ok(ExampleCommandRequest::TestCommand(deserialized))
            },
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Example.", command_index))
        }
    }

    fn from_response(command_index: CommandIndex, response: &SchemaCommandResponse) -> Result<generated::example::ExampleCommandResponse, String> {
        match command_index {
            1 => {
                let deserialized = <generated::example::CommandData as ObjectField>::from_object(&response.object());
                Ok(ExampleCommandResponse::TestCommand(deserialized))
            },
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Example.", command_index))
        }
    }

    fn to_data(data: &generated::example::Example) -> Result<Owned<SchemaComponentData>, String> {
        let mut serialized_data = SchemaComponentData::new();
        <generated::example::Example as ObjectField>::into_object(data, &mut serialized_data.fields_mut());
        Ok(serialized_data)
    }

    fn to_update(update: &generated::example::ExampleUpdate) -> Result<Owned<SchemaComponentUpdate>, String> {
        let mut serialized_update = SchemaComponentUpdate::new();
        <generated::example::ExampleUpdate as ObjectField>::into_object(update, &mut serialized_update.fields_mut());
        Ok(serialized_update)
    }

    fn to_request(request: &generated::example::ExampleCommandRequest) -> Result<Owned<SchemaCommandRequest>, String> {
        let mut serialized_request = SchemaCommandRequest::new();
        match request {
            ExampleCommandRequest::TestCommand(ref data) => {
                <generated::example::CommandData as ObjectField>::into_object(data, &mut serialized_request.object_mut());
            },
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn to_response(response: &generated::example::ExampleCommandResponse) -> Result<Owned<SchemaCommandResponse>, String> {
        let mut serialized_response = SchemaCommandResponse::new();
        match response {
            ExampleCommandResponse::TestCommand(ref data) => {
                <generated::example::CommandData as ObjectField>::into_object(data, &mut serialized_response.object_mut());
            },
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn get_request_command_index(request: &generated::example::ExampleCommandRequest) -> u32 {
        match request {
            ExampleCommandRequest::TestCommand(_) => 1,
            _ => unreachable!(),
        }
    }

    fn get_response_command_index(response: &generated::example::ExampleCommandResponse) -> u32 {
        match response {
            ExampleCommandResponse::TestCommand(_) => 1,
            _ => unreachable!(),
        }
    }
}

inventory::submit!(VTable::new::<Example>());

#[derive(Debug, Clone)]
pub struct Rotate {
    pub angle: f64,
    pub center: generated::example::Vector3d,
    pub radius: f64,
}
impl ObjectField for Rotate {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            angle: input.get::<SchemaDouble>(1),
            center: input.get::<generated::example::Vector3d>(2),
            radius: input.get::<SchemaDouble>(3),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaDouble>(1, &self.angle);
        output.add::<generated::example::Vector3d>(2, &self.center);
        output.add::<SchemaDouble>(3, &self.radius);
    }
}
impl ComponentData<Rotate> for Rotate {
    fn merge(&mut self, update: RotateUpdate) {
        if let Some(value) = update.angle { self.angle = value; }
        if let Some(value) = update.center { self.center = value; }
        if let Some(value) = update.radius { self.radius = value; }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RotateUpdate {
    pub angle: Option<f64>,
    pub center: Option<generated::example::Vector3d>,
    pub radius: Option<f64>,
}
impl ObjectField for RotateUpdate {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            angle: if input.count::<SchemaDouble>(1) > 0 { Some(input.get::<SchemaDouble>(1)) } else { None },
            center: if input.object_count(2) > 0 { Some(input.get::<generated::example::Vector3d>(2)) } else { None },
            radius: if input.count::<SchemaDouble>(3) > 0 { Some(input.get::<SchemaDouble>(3)) } else { None },
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        if let Some(value) = &self.angle {
            output.add::<SchemaDouble>(1, value);
        }
        if let Some(value) = &self.center {
            output.add::<generated::example::Vector3d>(2, value);
        }
        if let Some(value) = &self.radius {
            output.add::<SchemaDouble>(3, value);
        }
    }
}
impl ComponentUpdate<Rotate> for RotateUpdate {
    fn merge(&mut self, update: RotateUpdate) {
        if update.angle.is_some() { self.angle = update.angle; }
        if update.center.is_some() { self.center = update.center; }
        if update.radius.is_some() { self.radius = update.radius; }
    }
}

#[derive(Debug, Clone)]
pub enum RotateCommandRequest {
}

#[derive(Debug, Clone)]
pub enum RotateCommandResponse {
}

impl Component for Rotate {
    type Update = generated::example::RotateUpdate;
    type CommandRequest = generated::example::RotateCommandRequest;
    type CommandResponse = generated::example::RotateCommandResponse;

    const ID: ComponentId = 1001;

    fn from_data(data: &SchemaComponentData) -> Result<generated::example::Rotate, String> {
        Ok(<generated::example::Rotate as ObjectField>::from_object(&data.fields()))
    }

    fn from_update(update: &SchemaComponentUpdate) -> Result<generated::example::RotateUpdate, String> {
        Ok(<generated::example::RotateUpdate as ObjectField>::from_object(&update.fields()))
    }

    fn from_request(command_index: CommandIndex, request: &SchemaCommandRequest) -> Result<generated::example::RotateCommandRequest, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Rotate.", command_index))
        }
    }

    fn from_response(command_index: CommandIndex, response: &SchemaCommandResponse) -> Result<generated::example::RotateCommandResponse, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Rotate.", command_index))
        }
    }

    fn to_data(data: &generated::example::Rotate) -> Result<Owned<SchemaComponentData>, String> {
        let mut serialized_data = SchemaComponentData::new();
        <generated::example::Rotate as ObjectField>::into_object(data, &mut serialized_data.fields_mut());
        Ok(serialized_data)
    }

    fn to_update(update: &generated::example::RotateUpdate) -> Result<Owned<SchemaComponentUpdate>, String> {
        let mut serialized_update = SchemaComponentUpdate::new();
        <generated::example::RotateUpdate as ObjectField>::into_object(update, &mut serialized_update.fields_mut());
        Ok(serialized_update)
    }

    fn to_request(request: &generated::example::RotateCommandRequest) -> Result<Owned<SchemaCommandRequest>, String> {
        let mut serialized_request = SchemaCommandRequest::new();
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn to_response(response: &generated::example::RotateCommandResponse) -> Result<Owned<SchemaCommandResponse>, String> {
        let mut serialized_response = SchemaCommandResponse::new();
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn get_request_command_index(request: &generated::example::RotateCommandRequest) -> u32 {
        match request {
            _ => unreachable!(),
        }
    }

    fn get_response_command_index(response: &generated::example::RotateCommandResponse) -> u32 {
        match response {
            _ => unreachable!(),
        }
    }
}

inventory::submit!(VTable::new::<Rotate>());


}

pub mod improbable {
use spatialos_sdk::worker::schema::*;
use spatialos_sdk::worker::component::*;
use std::{collections::BTreeMap, convert::TryFrom};

use super::super::generated as generated;

/* Enums. */
/* Types. */
#[derive(Debug, Clone)]
pub struct ComponentInterest {
    pub queries: Vec<generated::improbable::ComponentInterest_Query>,
}
impl ObjectField for ComponentInterest {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            queries: input.get::<List<generated::improbable::ComponentInterest_Query>>(1),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<List<generated::improbable::ComponentInterest_Query>>(1, &self.queries);
    }
}

#[derive(Debug, Clone)]
pub struct ComponentInterest_BoxConstraint {
    pub center: generated::improbable::Coordinates,
    pub edge_length: generated::improbable::EdgeLength,
}
impl ObjectField for ComponentInterest_BoxConstraint {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            center: input.get::<generated::improbable::Coordinates>(1),
            edge_length: input.get::<generated::improbable::EdgeLength>(2),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<generated::improbable::Coordinates>(1, &self.center);
        output.add::<generated::improbable::EdgeLength>(2, &self.edge_length);
    }
}

#[derive(Debug, Clone)]
pub struct ComponentInterest_CylinderConstraint {
    pub center: generated::improbable::Coordinates,
    pub radius: f64,
}
impl ObjectField for ComponentInterest_CylinderConstraint {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            center: input.get::<generated::improbable::Coordinates>(1),
            radius: input.get::<SchemaDouble>(2),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<generated::improbable::Coordinates>(1, &self.center);
        output.add::<SchemaDouble>(2, &self.radius);
    }
}

#[derive(Debug, Clone)]
pub struct ComponentInterest_Query {
    pub constraint: generated::improbable::ComponentInterest_QueryConstraint,
    pub full_snapshot_result: Option<bool>,
    pub result_component_id: Vec<u32>,
    pub frequency: Option<f32>,
}
impl ObjectField for ComponentInterest_Query {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            constraint: input.get::<generated::improbable::ComponentInterest_QueryConstraint>(1),
            full_snapshot_result: input.get::<Optional<SchemaBool>>(2),
            result_component_id: input.get::<List<SchemaUint32>>(3),
            frequency: input.get::<Optional<SchemaFloat>>(4),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<generated::improbable::ComponentInterest_QueryConstraint>(1, &self.constraint);
        output.add::<Optional<SchemaBool>>(2, &self.full_snapshot_result);
        output.add::<List<SchemaUint32>>(3, &self.result_component_id);
        output.add::<Optional<SchemaFloat>>(4, &self.frequency);
    }
}

#[derive(Debug, Clone)]
pub struct ComponentInterest_QueryConstraint {
    pub sphere_constraint: Option<generated::improbable::ComponentInterest_SphereConstraint>,
    pub cylinder_constraint: Option<generated::improbable::ComponentInterest_CylinderConstraint>,
    pub box_constraint: Option<generated::improbable::ComponentInterest_BoxConstraint>,
    pub relative_sphere_constraint: Option<generated::improbable::ComponentInterest_RelativeSphereConstraint>,
    pub relative_cylinder_constraint: Option<generated::improbable::ComponentInterest_RelativeCylinderConstraint>,
    pub relative_box_constraint: Option<generated::improbable::ComponentInterest_RelativeBoxConstraint>,
    pub entity_id_constraint: Option<i64>,
    pub component_constraint: Option<u32>,
    pub and_constraint: Vec<generated::improbable::ComponentInterest_QueryConstraint>,
    pub or_constraint: Vec<generated::improbable::ComponentInterest_QueryConstraint>,
}
impl ObjectField for ComponentInterest_QueryConstraint {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            sphere_constraint: input.get::<Optional<generated::improbable::ComponentInterest_SphereConstraint>>(1),
            cylinder_constraint: input.get::<Optional<generated::improbable::ComponentInterest_CylinderConstraint>>(2),
            box_constraint: input.get::<Optional<generated::improbable::ComponentInterest_BoxConstraint>>(3),
            relative_sphere_constraint: input.get::<Optional<generated::improbable::ComponentInterest_RelativeSphereConstraint>>(4),
            relative_cylinder_constraint: input.get::<Optional<generated::improbable::ComponentInterest_RelativeCylinderConstraint>>(5),
            relative_box_constraint: input.get::<Optional<generated::improbable::ComponentInterest_RelativeBoxConstraint>>(6),
            entity_id_constraint: input.get::<Optional<SchemaInt64>>(7),
            component_constraint: input.get::<Optional<SchemaUint32>>(8),
            and_constraint: input.get::<List<generated::improbable::ComponentInterest_QueryConstraint>>(9),
            or_constraint: input.get::<List<generated::improbable::ComponentInterest_QueryConstraint>>(10),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<Optional<generated::improbable::ComponentInterest_SphereConstraint>>(1, &self.sphere_constraint);
        output.add::<Optional<generated::improbable::ComponentInterest_CylinderConstraint>>(2, &self.cylinder_constraint);
        output.add::<Optional<generated::improbable::ComponentInterest_BoxConstraint>>(3, &self.box_constraint);
        output.add::<Optional<generated::improbable::ComponentInterest_RelativeSphereConstraint>>(4, &self.relative_sphere_constraint);
        output.add::<Optional<generated::improbable::ComponentInterest_RelativeCylinderConstraint>>(5, &self.relative_cylinder_constraint);
        output.add::<Optional<generated::improbable::ComponentInterest_RelativeBoxConstraint>>(6, &self.relative_box_constraint);
        output.add::<Optional<SchemaInt64>>(7, &self.entity_id_constraint);
        output.add::<Optional<SchemaUint32>>(8, &self.component_constraint);
        output.add::<List<generated::improbable::ComponentInterest_QueryConstraint>>(9, &self.and_constraint);
        output.add::<List<generated::improbable::ComponentInterest_QueryConstraint>>(10, &self.or_constraint);
    }
}

#[derive(Debug, Clone)]
pub struct ComponentInterest_RelativeBoxConstraint {
    pub edge_length: generated::improbable::EdgeLength,
}
impl ObjectField for ComponentInterest_RelativeBoxConstraint {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            edge_length: input.get::<generated::improbable::EdgeLength>(1),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<generated::improbable::EdgeLength>(1, &self.edge_length);
    }
}

#[derive(Debug, Clone)]
pub struct ComponentInterest_RelativeCylinderConstraint {
    pub radius: f64,
}
impl ObjectField for ComponentInterest_RelativeCylinderConstraint {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            radius: input.get::<SchemaDouble>(1),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaDouble>(1, &self.radius);
    }
}

#[derive(Debug, Clone)]
pub struct ComponentInterest_RelativeSphereConstraint {
    pub radius: f64,
}
impl ObjectField for ComponentInterest_RelativeSphereConstraint {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            radius: input.get::<SchemaDouble>(1),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaDouble>(1, &self.radius);
    }
}

#[derive(Debug, Clone)]
pub struct ComponentInterest_SphereConstraint {
    pub center: generated::improbable::Coordinates,
    pub radius: f64,
}
impl ObjectField for ComponentInterest_SphereConstraint {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            center: input.get::<generated::improbable::Coordinates>(1),
            radius: input.get::<SchemaDouble>(2),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<generated::improbable::Coordinates>(1, &self.center);
        output.add::<SchemaDouble>(2, &self.radius);
    }
}

#[derive(Debug, Clone)]
pub struct Coordinates {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl ObjectField for Coordinates {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            x: input.get::<SchemaDouble>(1),
            y: input.get::<SchemaDouble>(2),
            z: input.get::<SchemaDouble>(3),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaDouble>(1, &self.x);
        output.add::<SchemaDouble>(2, &self.y);
        output.add::<SchemaDouble>(3, &self.z);
    }
}

#[derive(Debug, Clone)]
pub struct EdgeLength {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl ObjectField for EdgeLength {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            x: input.get::<SchemaDouble>(1),
            y: input.get::<SchemaDouble>(2),
            z: input.get::<SchemaDouble>(3),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaDouble>(1, &self.x);
        output.add::<SchemaDouble>(2, &self.y);
        output.add::<SchemaDouble>(3, &self.z);
    }
}

#[derive(Debug, Clone)]
pub struct WorkerAttributeSet {
    pub attribute: Vec<String>,
}
impl ObjectField for WorkerAttributeSet {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            attribute: input.get::<List<SchemaString>>(1),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<List<SchemaString>>(1, &self.attribute);
    }
}

#[derive(Debug, Clone)]
pub struct WorkerRequirementSet {
    pub attribute_set: Vec<generated::improbable::WorkerAttributeSet>,
}
impl ObjectField for WorkerRequirementSet {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            attribute_set: input.get::<List<generated::improbable::WorkerAttributeSet>>(1),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<List<generated::improbable::WorkerAttributeSet>>(1, &self.attribute_set);
    }
}

/* Components. */ 
#[derive(Debug, Clone)]
pub struct EntityAcl {
    pub read_acl: generated::improbable::WorkerRequirementSet,
    pub component_write_acl: BTreeMap<u32, generated::improbable::WorkerRequirementSet>,
}
impl ObjectField for EntityAcl {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            read_acl: input.get::<generated::improbable::WorkerRequirementSet>(1),
            component_write_acl: input.get::<Map<SchemaUint32, generated::improbable::WorkerRequirementSet>>(2),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<generated::improbable::WorkerRequirementSet>(1, &self.read_acl);
        output.add::<Map<SchemaUint32, generated::improbable::WorkerRequirementSet>>(2, &self.component_write_acl);
    }
}
impl ComponentData<EntityAcl> for EntityAcl {
    fn merge(&mut self, update: EntityAclUpdate) {
        if let Some(value) = update.read_acl { self.read_acl = value; }
        if let Some(value) = update.component_write_acl { self.component_write_acl = value; }
    }
}

#[derive(Debug, Clone, Default)]
pub struct EntityAclUpdate {
    pub read_acl: Option<generated::improbable::WorkerRequirementSet>,
    pub component_write_acl: Option<BTreeMap<u32, generated::improbable::WorkerRequirementSet>>,
}
impl ObjectField for EntityAclUpdate {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            read_acl: if input.object_count(1) > 0 { Some(input.get::<generated::improbable::WorkerRequirementSet>(1)) } else { None },
            component_write_acl: Some(input.get::<Map<SchemaUint32, generated::improbable::WorkerRequirementSet>>(2)),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        if let Some(value) = &self.read_acl {
            output.add::<generated::improbable::WorkerRequirementSet>(1, value);
        }
        if let Some(value) = &self.component_write_acl {
            output.add::<Map<SchemaUint32, generated::improbable::WorkerRequirementSet>>(2, value);
        }
    }
}
impl ComponentUpdate<EntityAcl> for EntityAclUpdate {
    fn merge(&mut self, update: EntityAclUpdate) {
        if update.read_acl.is_some() { self.read_acl = update.read_acl; }
        if update.component_write_acl.is_some() { self.component_write_acl = update.component_write_acl; }
    }
}

#[derive(Debug, Clone)]
pub enum EntityAclCommandRequest {
}

#[derive(Debug, Clone)]
pub enum EntityAclCommandResponse {
}

impl Component for EntityAcl {
    type Update = generated::improbable::EntityAclUpdate;
    type CommandRequest = generated::improbable::EntityAclCommandRequest;
    type CommandResponse = generated::improbable::EntityAclCommandResponse;

    const ID: ComponentId = 50;

    fn from_data(data: &SchemaComponentData) -> Result<generated::improbable::EntityAcl, String> {
        Ok(<generated::improbable::EntityAcl as ObjectField>::from_object(&data.fields()))
    }

    fn from_update(update: &SchemaComponentUpdate) -> Result<generated::improbable::EntityAclUpdate, String> {
        Ok(<generated::improbable::EntityAclUpdate as ObjectField>::from_object(&update.fields()))
    }

    fn from_request(command_index: CommandIndex, request: &SchemaCommandRequest) -> Result<generated::improbable::EntityAclCommandRequest, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component EntityAcl.", command_index))
        }
    }

    fn from_response(command_index: CommandIndex, response: &SchemaCommandResponse) -> Result<generated::improbable::EntityAclCommandResponse, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component EntityAcl.", command_index))
        }
    }

    fn to_data(data: &generated::improbable::EntityAcl) -> Result<Owned<SchemaComponentData>, String> {
        let mut serialized_data = SchemaComponentData::new();
        <generated::improbable::EntityAcl as ObjectField>::into_object(data, &mut serialized_data.fields_mut());
        Ok(serialized_data)
    }

    fn to_update(update: &generated::improbable::EntityAclUpdate) -> Result<Owned<SchemaComponentUpdate>, String> {
        let mut serialized_update = SchemaComponentUpdate::new();
        <generated::improbable::EntityAclUpdate as ObjectField>::into_object(update, &mut serialized_update.fields_mut());
        Ok(serialized_update)
    }

    fn to_request(request: &generated::improbable::EntityAclCommandRequest) -> Result<Owned<SchemaCommandRequest>, String> {
        let mut serialized_request = SchemaCommandRequest::new();
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn to_response(response: &generated::improbable::EntityAclCommandResponse) -> Result<Owned<SchemaCommandResponse>, String> {
        let mut serialized_response = SchemaCommandResponse::new();
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn get_request_command_index(request: &generated::improbable::EntityAclCommandRequest) -> u32 {
        match request {
            _ => unreachable!(),
        }
    }

    fn get_response_command_index(response: &generated::improbable::EntityAclCommandResponse) -> u32 {
        match response {
            _ => unreachable!(),
        }
    }
}

inventory::submit!(VTable::new::<EntityAcl>());

#[derive(Debug, Clone)]
pub struct Interest {
    pub component_interest: BTreeMap<u32, generated::improbable::ComponentInterest>,
}
impl ObjectField for Interest {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            component_interest: input.get::<Map<SchemaUint32, generated::improbable::ComponentInterest>>(1),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<Map<SchemaUint32, generated::improbable::ComponentInterest>>(1, &self.component_interest);
    }
}
impl ComponentData<Interest> for Interest {
    fn merge(&mut self, update: InterestUpdate) {
        if let Some(value) = update.component_interest { self.component_interest = value; }
    }
}

#[derive(Debug, Clone, Default)]
pub struct InterestUpdate {
    pub component_interest: Option<BTreeMap<u32, generated::improbable::ComponentInterest>>,
}
impl ObjectField for InterestUpdate {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            component_interest: Some(input.get::<Map<SchemaUint32, generated::improbable::ComponentInterest>>(1)),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        if let Some(value) = &self.component_interest {
            output.add::<Map<SchemaUint32, generated::improbable::ComponentInterest>>(1, value);
        }
    }
}
impl ComponentUpdate<Interest> for InterestUpdate {
    fn merge(&mut self, update: InterestUpdate) {
        if update.component_interest.is_some() { self.component_interest = update.component_interest; }
    }
}

#[derive(Debug, Clone)]
pub enum InterestCommandRequest {
}

#[derive(Debug, Clone)]
pub enum InterestCommandResponse {
}

impl Component for Interest {
    type Update = generated::improbable::InterestUpdate;
    type CommandRequest = generated::improbable::InterestCommandRequest;
    type CommandResponse = generated::improbable::InterestCommandResponse;

    const ID: ComponentId = 58;

    fn from_data(data: &SchemaComponentData) -> Result<generated::improbable::Interest, String> {
        Ok(<generated::improbable::Interest as ObjectField>::from_object(&data.fields()))
    }

    fn from_update(update: &SchemaComponentUpdate) -> Result<generated::improbable::InterestUpdate, String> {
        Ok(<generated::improbable::InterestUpdate as ObjectField>::from_object(&update.fields()))
    }

    fn from_request(command_index: CommandIndex, request: &SchemaCommandRequest) -> Result<generated::improbable::InterestCommandRequest, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Interest.", command_index))
        }
    }

    fn from_response(command_index: CommandIndex, response: &SchemaCommandResponse) -> Result<generated::improbable::InterestCommandResponse, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Interest.", command_index))
        }
    }

    fn to_data(data: &generated::improbable::Interest) -> Result<Owned<SchemaComponentData>, String> {
        let mut serialized_data = SchemaComponentData::new();
        <generated::improbable::Interest as ObjectField>::into_object(data, &mut serialized_data.fields_mut());
        Ok(serialized_data)
    }

    fn to_update(update: &generated::improbable::InterestUpdate) -> Result<Owned<SchemaComponentUpdate>, String> {
        let mut serialized_update = SchemaComponentUpdate::new();
        <generated::improbable::InterestUpdate as ObjectField>::into_object(update, &mut serialized_update.fields_mut());
        Ok(serialized_update)
    }

    fn to_request(request: &generated::improbable::InterestCommandRequest) -> Result<Owned<SchemaCommandRequest>, String> {
        let mut serialized_request = SchemaCommandRequest::new();
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn to_response(response: &generated::improbable::InterestCommandResponse) -> Result<Owned<SchemaCommandResponse>, String> {
        let mut serialized_response = SchemaCommandResponse::new();
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn get_request_command_index(request: &generated::improbable::InterestCommandRequest) -> u32 {
        match request {
            _ => unreachable!(),
        }
    }

    fn get_response_command_index(response: &generated::improbable::InterestCommandResponse) -> u32 {
        match response {
            _ => unreachable!(),
        }
    }
}

inventory::submit!(VTable::new::<Interest>());

#[derive(Debug, Clone)]
pub struct Metadata {
    pub entity_type: String,
}
impl ObjectField for Metadata {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            entity_type: input.get::<SchemaString>(1),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaString>(1, &self.entity_type);
    }
}
impl ComponentData<Metadata> for Metadata {
    fn merge(&mut self, update: MetadataUpdate) {
        if let Some(value) = update.entity_type { self.entity_type = value; }
    }
}

#[derive(Debug, Clone, Default)]
pub struct MetadataUpdate {
    pub entity_type: Option<String>,
}
impl ObjectField for MetadataUpdate {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            entity_type: if input.count::<SchemaString>(1) > 0 { Some(input.get::<SchemaString>(1)) } else { None },
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        if let Some(value) = &self.entity_type {
            output.add::<SchemaString>(1, value);
        }
    }
}
impl ComponentUpdate<Metadata> for MetadataUpdate {
    fn merge(&mut self, update: MetadataUpdate) {
        if update.entity_type.is_some() { self.entity_type = update.entity_type; }
    }
}

#[derive(Debug, Clone)]
pub enum MetadataCommandRequest {
}

#[derive(Debug, Clone)]
pub enum MetadataCommandResponse {
}

impl Component for Metadata {
    type Update = generated::improbable::MetadataUpdate;
    type CommandRequest = generated::improbable::MetadataCommandRequest;
    type CommandResponse = generated::improbable::MetadataCommandResponse;

    const ID: ComponentId = 53;

    fn from_data(data: &SchemaComponentData) -> Result<generated::improbable::Metadata, String> {
        Ok(<generated::improbable::Metadata as ObjectField>::from_object(&data.fields()))
    }

    fn from_update(update: &SchemaComponentUpdate) -> Result<generated::improbable::MetadataUpdate, String> {
        Ok(<generated::improbable::MetadataUpdate as ObjectField>::from_object(&update.fields()))
    }

    fn from_request(command_index: CommandIndex, request: &SchemaCommandRequest) -> Result<generated::improbable::MetadataCommandRequest, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Metadata.", command_index))
        }
    }

    fn from_response(command_index: CommandIndex, response: &SchemaCommandResponse) -> Result<generated::improbable::MetadataCommandResponse, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Metadata.", command_index))
        }
    }

    fn to_data(data: &generated::improbable::Metadata) -> Result<Owned<SchemaComponentData>, String> {
        let mut serialized_data = SchemaComponentData::new();
        <generated::improbable::Metadata as ObjectField>::into_object(data, &mut serialized_data.fields_mut());
        Ok(serialized_data)
    }

    fn to_update(update: &generated::improbable::MetadataUpdate) -> Result<Owned<SchemaComponentUpdate>, String> {
        let mut serialized_update = SchemaComponentUpdate::new();
        <generated::improbable::MetadataUpdate as ObjectField>::into_object(update, &mut serialized_update.fields_mut());
        Ok(serialized_update)
    }

    fn to_request(request: &generated::improbable::MetadataCommandRequest) -> Result<Owned<SchemaCommandRequest>, String> {
        let mut serialized_request = SchemaCommandRequest::new();
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn to_response(response: &generated::improbable::MetadataCommandResponse) -> Result<Owned<SchemaCommandResponse>, String> {
        let mut serialized_response = SchemaCommandResponse::new();
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn get_request_command_index(request: &generated::improbable::MetadataCommandRequest) -> u32 {
        match request {
            _ => unreachable!(),
        }
    }

    fn get_response_command_index(response: &generated::improbable::MetadataCommandResponse) -> u32 {
        match response {
            _ => unreachable!(),
        }
    }
}

inventory::submit!(VTable::new::<Metadata>());

#[derive(Debug, Clone)]
pub struct Persistence {
}
impl ObjectField for Persistence {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
    }
}
impl ComponentData<Persistence> for Persistence {
    fn merge(&mut self, update: PersistenceUpdate) {
    }
}

#[derive(Debug, Clone, Default)]
pub struct PersistenceUpdate {
}
impl ObjectField for PersistenceUpdate {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
    }
}
impl ComponentUpdate<Persistence> for PersistenceUpdate {
    fn merge(&mut self, update: PersistenceUpdate) {
    }
}

#[derive(Debug, Clone)]
pub enum PersistenceCommandRequest {
}

#[derive(Debug, Clone)]
pub enum PersistenceCommandResponse {
}

impl Component for Persistence {
    type Update = generated::improbable::PersistenceUpdate;
    type CommandRequest = generated::improbable::PersistenceCommandRequest;
    type CommandResponse = generated::improbable::PersistenceCommandResponse;

    const ID: ComponentId = 55;

    fn from_data(data: &SchemaComponentData) -> Result<generated::improbable::Persistence, String> {
        Ok(<generated::improbable::Persistence as ObjectField>::from_object(&data.fields()))
    }

    fn from_update(update: &SchemaComponentUpdate) -> Result<generated::improbable::PersistenceUpdate, String> {
        Ok(<generated::improbable::PersistenceUpdate as ObjectField>::from_object(&update.fields()))
    }

    fn from_request(command_index: CommandIndex, request: &SchemaCommandRequest) -> Result<generated::improbable::PersistenceCommandRequest, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Persistence.", command_index))
        }
    }

    fn from_response(command_index: CommandIndex, response: &SchemaCommandResponse) -> Result<generated::improbable::PersistenceCommandResponse, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Persistence.", command_index))
        }
    }

    fn to_data(data: &generated::improbable::Persistence) -> Result<Owned<SchemaComponentData>, String> {
        let mut serialized_data = SchemaComponentData::new();
        <generated::improbable::Persistence as ObjectField>::into_object(data, &mut serialized_data.fields_mut());
        Ok(serialized_data)
    }

    fn to_update(update: &generated::improbable::PersistenceUpdate) -> Result<Owned<SchemaComponentUpdate>, String> {
        let mut serialized_update = SchemaComponentUpdate::new();
        <generated::improbable::PersistenceUpdate as ObjectField>::into_object(update, &mut serialized_update.fields_mut());
        Ok(serialized_update)
    }

    fn to_request(request: &generated::improbable::PersistenceCommandRequest) -> Result<Owned<SchemaCommandRequest>, String> {
        let mut serialized_request = SchemaCommandRequest::new();
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn to_response(response: &generated::improbable::PersistenceCommandResponse) -> Result<Owned<SchemaCommandResponse>, String> {
        let mut serialized_response = SchemaCommandResponse::new();
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn get_request_command_index(request: &generated::improbable::PersistenceCommandRequest) -> u32 {
        match request {
            _ => unreachable!(),
        }
    }

    fn get_response_command_index(response: &generated::improbable::PersistenceCommandResponse) -> u32 {
        match response {
            _ => unreachable!(),
        }
    }
}

inventory::submit!(VTable::new::<Persistence>());

#[derive(Debug, Clone)]
pub struct Position {
    pub coords: generated::improbable::Coordinates,
}
impl ObjectField for Position {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            coords: input.get::<generated::improbable::Coordinates>(1),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<generated::improbable::Coordinates>(1, &self.coords);
    }
}
impl ComponentData<Position> for Position {
    fn merge(&mut self, update: PositionUpdate) {
        if let Some(value) = update.coords { self.coords = value; }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PositionUpdate {
    pub coords: Option<generated::improbable::Coordinates>,
}
impl ObjectField for PositionUpdate {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            coords: if input.object_count(1) > 0 { Some(input.get::<generated::improbable::Coordinates>(1)) } else { None },
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        if let Some(value) = &self.coords {
            output.add::<generated::improbable::Coordinates>(1, value);
        }
    }
}
impl ComponentUpdate<Position> for PositionUpdate {
    fn merge(&mut self, update: PositionUpdate) {
        if update.coords.is_some() { self.coords = update.coords; }
    }
}

#[derive(Debug, Clone)]
pub enum PositionCommandRequest {
}

#[derive(Debug, Clone)]
pub enum PositionCommandResponse {
}

impl Component for Position {
    type Update = generated::improbable::PositionUpdate;
    type CommandRequest = generated::improbable::PositionCommandRequest;
    type CommandResponse = generated::improbable::PositionCommandResponse;

    const ID: ComponentId = 54;

    fn from_data(data: &SchemaComponentData) -> Result<generated::improbable::Position, String> {
        Ok(<generated::improbable::Position as ObjectField>::from_object(&data.fields()))
    }

    fn from_update(update: &SchemaComponentUpdate) -> Result<generated::improbable::PositionUpdate, String> {
        Ok(<generated::improbable::PositionUpdate as ObjectField>::from_object(&update.fields()))
    }

    fn from_request(command_index: CommandIndex, request: &SchemaCommandRequest) -> Result<generated::improbable::PositionCommandRequest, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Position.", command_index))
        }
    }

    fn from_response(command_index: CommandIndex, response: &SchemaCommandResponse) -> Result<generated::improbable::PositionCommandResponse, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Position.", command_index))
        }
    }

    fn to_data(data: &generated::improbable::Position) -> Result<Owned<SchemaComponentData>, String> {
        let mut serialized_data = SchemaComponentData::new();
        <generated::improbable::Position as ObjectField>::into_object(data, &mut serialized_data.fields_mut());
        Ok(serialized_data)
    }

    fn to_update(update: &generated::improbable::PositionUpdate) -> Result<Owned<SchemaComponentUpdate>, String> {
        let mut serialized_update = SchemaComponentUpdate::new();
        <generated::improbable::PositionUpdate as ObjectField>::into_object(update, &mut serialized_update.fields_mut());
        Ok(serialized_update)
    }

    fn to_request(request: &generated::improbable::PositionCommandRequest) -> Result<Owned<SchemaCommandRequest>, String> {
        let mut serialized_request = SchemaCommandRequest::new();
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn to_response(response: &generated::improbable::PositionCommandResponse) -> Result<Owned<SchemaCommandResponse>, String> {
        let mut serialized_response = SchemaCommandResponse::new();
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn get_request_command_index(request: &generated::improbable::PositionCommandRequest) -> u32 {
        match request {
            _ => unreachable!(),
        }
    }

    fn get_response_command_index(response: &generated::improbable::PositionCommandResponse) -> u32 {
        match response {
            _ => unreachable!(),
        }
    }
}

inventory::submit!(VTable::new::<Position>());



pub mod restricted {
use spatialos_sdk::worker::schema::*;
use spatialos_sdk::worker::component::*;
use std::{collections::BTreeMap, convert::TryFrom};

use super::super::super::generated as generated;

/* Enums. */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Connection_ConnectionStatus {

    UNKNOWN,
    AWAITING_WORKER_CONNECTION,
    CONNECTED,
    DISCONNECTED,
}

impl EnumField for Connection_ConnectionStatus {}

impl Default for Connection_ConnectionStatus {
    fn default() -> Self {
        Connection_ConnectionStatus::UNKNOWN
    }
}

impl TryFrom<u32> for Connection_ConnectionStatus {
    type Error = UnknownDiscriminantError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            
            0 => Ok(Connection_ConnectionStatus::UNKNOWN), 
            1 => Ok(Connection_ConnectionStatus::AWAITING_WORKER_CONNECTION), 
            2 => Ok(Connection_ConnectionStatus::CONNECTED), 
            3 => Ok(Connection_ConnectionStatus::DISCONNECTED), 
            _ => Err(UnknownDiscriminantError {
                type_name: std::any::type_name::<Self>(),
                value,
            }),
        }
    }
}

impl Into<u32> for Connection_ConnectionStatus {
    fn into(self) -> u32 {
        match self {
            
            Connection_ConnectionStatus::UNKNOWN => 0, 
            Connection_ConnectionStatus::AWAITING_WORKER_CONNECTION => 1, 
            Connection_ConnectionStatus::CONNECTED => 2, 
            Connection_ConnectionStatus::DISCONNECTED => 3, 
        }
    }
}

impl_field_for_enum_field!(Connection_ConnectionStatus);

/* Types. */
#[derive(Debug, Clone)]
pub struct Connection {
    pub status: generated::improbable::restricted::Connection_ConnectionStatus,
    pub data_latency_ms: u32,
    pub connected_since_utc: u64,
}
impl ObjectField for Connection {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            status: input.get::<generated::improbable::restricted::Connection_ConnectionStatus>(1),
            data_latency_ms: input.get::<SchemaUint32>(2),
            connected_since_utc: input.get::<SchemaUint64>(3),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<generated::improbable::restricted::Connection_ConnectionStatus>(1, &self.status);
        output.add::<SchemaUint32>(2, &self.data_latency_ms);
        output.add::<SchemaUint64>(3, &self.connected_since_utc);
    }
}

#[derive(Debug, Clone)]
pub struct DisconnectRequest {
}
impl ObjectField for DisconnectRequest {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
    }
}

#[derive(Debug, Clone)]
pub struct DisconnectResponse {
}
impl ObjectField for DisconnectResponse {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
    }
}

#[derive(Debug, Clone)]
pub struct PlayerIdentity {
    pub player_identifier: String,
    pub provider: String,
    pub metadata: Vec<u8>,
}
impl ObjectField for PlayerIdentity {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            player_identifier: input.get::<SchemaString>(1),
            provider: input.get::<SchemaString>(2),
            metadata: input.get::<SchemaBytes>(3),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaString>(1, &self.player_identifier);
        output.add::<SchemaString>(2, &self.provider);
        output.add::<SchemaBytes>(3, &self.metadata);
    }
}

/* Components. */ 
#[derive(Debug, Clone)]
pub struct PlayerClient {
    pub player_identity: generated::improbable::restricted::PlayerIdentity,
}
impl ObjectField for PlayerClient {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            player_identity: input.get::<generated::improbable::restricted::PlayerIdentity>(1),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<generated::improbable::restricted::PlayerIdentity>(1, &self.player_identity);
    }
}
impl ComponentData<PlayerClient> for PlayerClient {
    fn merge(&mut self, update: PlayerClientUpdate) {
        if let Some(value) = update.player_identity { self.player_identity = value; }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PlayerClientUpdate {
    pub player_identity: Option<generated::improbable::restricted::PlayerIdentity>,
}
impl ObjectField for PlayerClientUpdate {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            player_identity: if input.object_count(1) > 0 { Some(input.get::<generated::improbable::restricted::PlayerIdentity>(1)) } else { None },
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        if let Some(value) = &self.player_identity {
            output.add::<generated::improbable::restricted::PlayerIdentity>(1, value);
        }
    }
}
impl ComponentUpdate<PlayerClient> for PlayerClientUpdate {
    fn merge(&mut self, update: PlayerClientUpdate) {
        if update.player_identity.is_some() { self.player_identity = update.player_identity; }
    }
}

#[derive(Debug, Clone)]
pub enum PlayerClientCommandRequest {
}

#[derive(Debug, Clone)]
pub enum PlayerClientCommandResponse {
}

impl Component for PlayerClient {
    type Update = generated::improbable::restricted::PlayerClientUpdate;
    type CommandRequest = generated::improbable::restricted::PlayerClientCommandRequest;
    type CommandResponse = generated::improbable::restricted::PlayerClientCommandResponse;

    const ID: ComponentId = 61;

    fn from_data(data: &SchemaComponentData) -> Result<generated::improbable::restricted::PlayerClient, String> {
        Ok(<generated::improbable::restricted::PlayerClient as ObjectField>::from_object(&data.fields()))
    }

    fn from_update(update: &SchemaComponentUpdate) -> Result<generated::improbable::restricted::PlayerClientUpdate, String> {
        Ok(<generated::improbable::restricted::PlayerClientUpdate as ObjectField>::from_object(&update.fields()))
    }

    fn from_request(command_index: CommandIndex, request: &SchemaCommandRequest) -> Result<generated::improbable::restricted::PlayerClientCommandRequest, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component PlayerClient.", command_index))
        }
    }

    fn from_response(command_index: CommandIndex, response: &SchemaCommandResponse) -> Result<generated::improbable::restricted::PlayerClientCommandResponse, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component PlayerClient.", command_index))
        }
    }

    fn to_data(data: &generated::improbable::restricted::PlayerClient) -> Result<Owned<SchemaComponentData>, String> {
        let mut serialized_data = SchemaComponentData::new();
        <generated::improbable::restricted::PlayerClient as ObjectField>::into_object(data, &mut serialized_data.fields_mut());
        Ok(serialized_data)
    }

    fn to_update(update: &generated::improbable::restricted::PlayerClientUpdate) -> Result<Owned<SchemaComponentUpdate>, String> {
        let mut serialized_update = SchemaComponentUpdate::new();
        <generated::improbable::restricted::PlayerClientUpdate as ObjectField>::into_object(update, &mut serialized_update.fields_mut());
        Ok(serialized_update)
    }

    fn to_request(request: &generated::improbable::restricted::PlayerClientCommandRequest) -> Result<Owned<SchemaCommandRequest>, String> {
        let mut serialized_request = SchemaCommandRequest::new();
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn to_response(response: &generated::improbable::restricted::PlayerClientCommandResponse) -> Result<Owned<SchemaCommandResponse>, String> {
        let mut serialized_response = SchemaCommandResponse::new();
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn get_request_command_index(request: &generated::improbable::restricted::PlayerClientCommandRequest) -> u32 {
        match request {
            _ => unreachable!(),
        }
    }

    fn get_response_command_index(response: &generated::improbable::restricted::PlayerClientCommandResponse) -> u32 {
        match response {
            _ => unreachable!(),
        }
    }
}

inventory::submit!(VTable::new::<PlayerClient>());

#[derive(Debug, Clone)]
pub struct System {
}
impl ObjectField for System {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
    }
}
impl ComponentData<System> for System {
    fn merge(&mut self, update: SystemUpdate) {
    }
}

#[derive(Debug, Clone, Default)]
pub struct SystemUpdate {
}
impl ObjectField for SystemUpdate {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
    }
}
impl ComponentUpdate<System> for SystemUpdate {
    fn merge(&mut self, update: SystemUpdate) {
    }
}

#[derive(Debug, Clone)]
pub enum SystemCommandRequest {
}

#[derive(Debug, Clone)]
pub enum SystemCommandResponse {
}

impl Component for System {
    type Update = generated::improbable::restricted::SystemUpdate;
    type CommandRequest = generated::improbable::restricted::SystemCommandRequest;
    type CommandResponse = generated::improbable::restricted::SystemCommandResponse;

    const ID: ComponentId = 59;

    fn from_data(data: &SchemaComponentData) -> Result<generated::improbable::restricted::System, String> {
        Ok(<generated::improbable::restricted::System as ObjectField>::from_object(&data.fields()))
    }

    fn from_update(update: &SchemaComponentUpdate) -> Result<generated::improbable::restricted::SystemUpdate, String> {
        Ok(<generated::improbable::restricted::SystemUpdate as ObjectField>::from_object(&update.fields()))
    }

    fn from_request(command_index: CommandIndex, request: &SchemaCommandRequest) -> Result<generated::improbable::restricted::SystemCommandRequest, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component System.", command_index))
        }
    }

    fn from_response(command_index: CommandIndex, response: &SchemaCommandResponse) -> Result<generated::improbable::restricted::SystemCommandResponse, String> {
        match command_index {
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component System.", command_index))
        }
    }

    fn to_data(data: &generated::improbable::restricted::System) -> Result<Owned<SchemaComponentData>, String> {
        let mut serialized_data = SchemaComponentData::new();
        <generated::improbable::restricted::System as ObjectField>::into_object(data, &mut serialized_data.fields_mut());
        Ok(serialized_data)
    }

    fn to_update(update: &generated::improbable::restricted::SystemUpdate) -> Result<Owned<SchemaComponentUpdate>, String> {
        let mut serialized_update = SchemaComponentUpdate::new();
        <generated::improbable::restricted::SystemUpdate as ObjectField>::into_object(update, &mut serialized_update.fields_mut());
        Ok(serialized_update)
    }

    fn to_request(request: &generated::improbable::restricted::SystemCommandRequest) -> Result<Owned<SchemaCommandRequest>, String> {
        let mut serialized_request = SchemaCommandRequest::new();
        match request {
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn to_response(response: &generated::improbable::restricted::SystemCommandResponse) -> Result<Owned<SchemaCommandResponse>, String> {
        let mut serialized_response = SchemaCommandResponse::new();
        match response {
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn get_request_command_index(request: &generated::improbable::restricted::SystemCommandRequest) -> u32 {
        match request {
            _ => unreachable!(),
        }
    }

    fn get_response_command_index(response: &generated::improbable::restricted::SystemCommandResponse) -> u32 {
        match response {
            _ => unreachable!(),
        }
    }
}

inventory::submit!(VTable::new::<System>());

#[derive(Debug, Clone)]
pub struct Worker {
    pub worker_id: String,
    pub worker_type: String,
    pub connection: generated::improbable::restricted::Connection,
}
impl ObjectField for Worker {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            worker_id: input.get::<SchemaString>(1),
            worker_type: input.get::<SchemaString>(2),
            connection: input.get::<generated::improbable::restricted::Connection>(3),
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaString>(1, &self.worker_id);
        output.add::<SchemaString>(2, &self.worker_type);
        output.add::<generated::improbable::restricted::Connection>(3, &self.connection);
    }
}
impl ComponentData<Worker> for Worker {
    fn merge(&mut self, update: WorkerUpdate) {
        if let Some(value) = update.worker_id { self.worker_id = value; }
        if let Some(value) = update.worker_type { self.worker_type = value; }
        if let Some(value) = update.connection { self.connection = value; }
    }
}

#[derive(Debug, Clone, Default)]
pub struct WorkerUpdate {
    pub worker_id: Option<String>,
    pub worker_type: Option<String>,
    pub connection: Option<generated::improbable::restricted::Connection>,
}
impl ObjectField for WorkerUpdate {
    fn from_object(input: &SchemaObject) -> Self {
        Self {
            worker_id: if input.count::<SchemaString>(1) > 0 { Some(input.get::<SchemaString>(1)) } else { None },
            worker_type: if input.count::<SchemaString>(2) > 0 { Some(input.get::<SchemaString>(2)) } else { None },
            connection: if input.object_count(3) > 0 { Some(input.get::<generated::improbable::restricted::Connection>(3)) } else { None },
        }
    }
    fn into_object(&self, output: &mut SchemaObject) {
        if let Some(value) = &self.worker_id {
            output.add::<SchemaString>(1, value);
        }
        if let Some(value) = &self.worker_type {
            output.add::<SchemaString>(2, value);
        }
        if let Some(value) = &self.connection {
            output.add::<generated::improbable::restricted::Connection>(3, value);
        }
    }
}
impl ComponentUpdate<Worker> for WorkerUpdate {
    fn merge(&mut self, update: WorkerUpdate) {
        if update.worker_id.is_some() { self.worker_id = update.worker_id; }
        if update.worker_type.is_some() { self.worker_type = update.worker_type; }
        if update.connection.is_some() { self.connection = update.connection; }
    }
}

#[derive(Debug, Clone)]
pub enum WorkerCommandRequest {
    Disconnect(generated::improbable::restricted::DisconnectRequest),
}

#[derive(Debug, Clone)]
pub enum WorkerCommandResponse {
    Disconnect(generated::improbable::restricted::DisconnectResponse),
}

impl Component for Worker {
    type Update = generated::improbable::restricted::WorkerUpdate;
    type CommandRequest = generated::improbable::restricted::WorkerCommandRequest;
    type CommandResponse = generated::improbable::restricted::WorkerCommandResponse;

    const ID: ComponentId = 60;

    fn from_data(data: &SchemaComponentData) -> Result<generated::improbable::restricted::Worker, String> {
        Ok(<generated::improbable::restricted::Worker as ObjectField>::from_object(&data.fields()))
    }

    fn from_update(update: &SchemaComponentUpdate) -> Result<generated::improbable::restricted::WorkerUpdate, String> {
        Ok(<generated::improbable::restricted::WorkerUpdate as ObjectField>::from_object(&update.fields()))
    }

    fn from_request(command_index: CommandIndex, request: &SchemaCommandRequest) -> Result<generated::improbable::restricted::WorkerCommandRequest, String> {
        match command_index {
            1 => {
                let deserialized = <generated::improbable::restricted::DisconnectRequest as ObjectField>::from_object(&request.object());
                Ok(WorkerCommandRequest::Disconnect(deserialized))
            },
            _ => Err(format!("Attempted to deserialize an unrecognised command request with index {} in component Worker.", command_index))
        }
    }

    fn from_response(command_index: CommandIndex, response: &SchemaCommandResponse) -> Result<generated::improbable::restricted::WorkerCommandResponse, String> {
        match command_index {
            1 => {
                let deserialized = <generated::improbable::restricted::DisconnectResponse as ObjectField>::from_object(&response.object());
                Ok(WorkerCommandResponse::Disconnect(deserialized))
            },
            _ => Err(format!("Attempted to deserialize an unrecognised command response with index {} in component Worker.", command_index))
        }
    }

    fn to_data(data: &generated::improbable::restricted::Worker) -> Result<Owned<SchemaComponentData>, String> {
        let mut serialized_data = SchemaComponentData::new();
        <generated::improbable::restricted::Worker as ObjectField>::into_object(data, &mut serialized_data.fields_mut());
        Ok(serialized_data)
    }

    fn to_update(update: &generated::improbable::restricted::WorkerUpdate) -> Result<Owned<SchemaComponentUpdate>, String> {
        let mut serialized_update = SchemaComponentUpdate::new();
        <generated::improbable::restricted::WorkerUpdate as ObjectField>::into_object(update, &mut serialized_update.fields_mut());
        Ok(serialized_update)
    }

    fn to_request(request: &generated::improbable::restricted::WorkerCommandRequest) -> Result<Owned<SchemaCommandRequest>, String> {
        let mut serialized_request = SchemaCommandRequest::new();
        match request {
            WorkerCommandRequest::Disconnect(ref data) => {
                <generated::improbable::restricted::DisconnectRequest as ObjectField>::into_object(data, &mut serialized_request.object_mut());
            },
            _ => unreachable!()
        }
        Ok(serialized_request)
    }

    fn to_response(response: &generated::improbable::restricted::WorkerCommandResponse) -> Result<Owned<SchemaCommandResponse>, String> {
        let mut serialized_response = SchemaCommandResponse::new();
        match response {
            WorkerCommandResponse::Disconnect(ref data) => {
                <generated::improbable::restricted::DisconnectResponse as ObjectField>::into_object(data, &mut serialized_response.object_mut());
            },
            _ => unreachable!()
        }
        Ok(serialized_response)
    }

    fn get_request_command_index(request: &generated::improbable::restricted::WorkerCommandRequest) -> u32 {
        match request {
            WorkerCommandRequest::Disconnect(_) => 1,
            _ => unreachable!(),
        }
    }

    fn get_response_command_index(response: &generated::improbable::restricted::WorkerCommandResponse) -> u32 {
        match response {
            WorkerCommandResponse::Disconnect(_) => 1,
            _ => unreachable!(),
        }
    }
}

inventory::submit!(VTable::new::<Worker>());


}
}
