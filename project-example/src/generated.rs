#![allow(unused_imports)]
#![allow(unreachable_code)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_mut)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::option_option)]

use spatialos_sdk::worker::schema::*;
use spatialos_sdk::worker::component::*;
use spatialos_sdk::worker::commands::*;
use std::{collections::BTreeMap, convert::TryFrom};

use super::generated as generated;

/* Enums. */
/* Types. */
/* Components. */ 


pub mod example {
use spatialos_sdk::worker::schema::*;
use spatialos_sdk::worker::component::*;
use spatialos_sdk::worker::commands::*;
use std::{collections::BTreeMap, convert::TryFrom};

use super::super::generated as generated;

/* Enums. */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SomeEnum {

    FIRST,
    SECOND,
}

impl EnumField for SomeEnum {}

impl Default for SomeEnum {
    fn default() -> Self {
        SomeEnum::FIRST
    }
}

impl TryFrom<u32> for SomeEnum {
    type Error = UnknownDiscriminantError;

    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            
            0 => Ok(SomeEnum::FIRST), 
            1 => Ok(SomeEnum::SECOND), 
            _ => Err(UnknownDiscriminantError {
                type_name: std::any::type_name::<Self>(),
                value,
            }),
        }
    }
}

impl Into<u32> for SomeEnum {
    fn into(self) -> u32 {
        match self {
            
            SomeEnum::FIRST => 0, 
            SomeEnum::SECOND => 1, 
        }
    }
}

impl_field_for_enum_field!(SomeEnum);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
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
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommandData {
    pub value: i32,
}
impl ObjectField for CommandData {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            value: input.get::<SchemaInt32>(1).map_err(Error::at_field::<Self>(1))?,
        })
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaInt32>(1, &self.value);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MapTypes {
    pub first: BTreeMap<generated::example::SomeEnum, i32>,
    pub second: BTreeMap<spatialos_sdk::worker::entity::Entity, i32>,
}
impl ObjectField for MapTypes {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            first: input.get::<Map<generated::example::SomeEnum, SchemaInt32>>(1).map_err(Error::at_field::<Self>(1))?,
            second: input.get::<Map<SchemaEntity, SchemaInt32>>(2).map_err(Error::at_field::<Self>(2))?,
        })
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<Map<generated::example::SomeEnum, SchemaInt32>>(1, &self.first);
        output.add::<Map<SchemaEntity, SchemaInt32>>(2, &self.second);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Recursive {
    pub opt: Option<Box<generated::example::Recursive>>,
}
impl ObjectField for Recursive {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            opt: input.get::<RecursiveOptional<generated::example::Recursive>>(1).map_err(Error::at_field::<Self>(1))?,
        })
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<RecursiveOptional<generated::example::Recursive>>(1, &self.opt);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TestType {
    pub value: i32,
}
impl ObjectField for TestType {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            value: input.get::<SchemaInt32>(1).map_err(Error::at_field::<Self>(1))?,
        })
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaInt32>(1, &self.value);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TestType_Inner {
    pub number: FloatOrd<f32>,
}
impl ObjectField for TestType_Inner {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            number: input.get::<SchemaFloat>(2).map_err(Error::at_field::<Self>(2))?,
        })
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaFloat>(2, &self.number);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector3d {
    pub x: FloatOrd<f64>,
    pub y: FloatOrd<f64>,
    pub z: FloatOrd<f64>,
}
impl ObjectField for Vector3d {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            x: input.get::<SchemaDouble>(1).map_err(Error::at_field::<Self>(1))?,
            y: input.get::<SchemaDouble>(2).map_err(Error::at_field::<Self>(2))?,
            z: input.get::<SchemaDouble>(3).map_err(Error::at_field::<Self>(3))?,
        })
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaDouble>(1, &self.x);
        output.add::<SchemaDouble>(2, &self.y);
        output.add::<SchemaDouble>(3, &self.z);
    }
}

/* Components. */ 
#[derive(Debug, Clone, Default)]
pub struct EntityIdTest {
    pub eid: spatialos_sdk::worker::EntityId,
}

impl ObjectField for EntityIdTest {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            eid: input.get::<SchemaEntityId>(1).map_err(Error::at_field::<Self>(1))?,
        })
    }

    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaEntityId>(1, &self.eid);
    }
}

#[derive(Debug, Clone, Default)]
pub struct EntityIdTestUpdate {
    pub eid: Option<spatialos_sdk::worker::EntityId>,
}

impl Update for EntityIdTestUpdate {
    type Component = EntityIdTest;

    fn from_schema(update: &SchemaComponentUpdate) -> Result<Self> {
        Ok(Self {
            eid: update.get_field::<SchemaEntityId>(1).map_err(Error::at_field::<Self>(1))?,
        })
    }

    fn into_schema(&self, update: &mut SchemaComponentUpdate) {
        update.add_field::<SchemaEntityId>(1, &self.eid);
    }

    fn merge(&mut self, mut update: Self) {
        if update.eid.is_some() { self.eid = update.eid; }
    }
}



impl Component for EntityIdTest {
    type Update = EntityIdTestUpdate;

    const ID: ComponentId = 2001;

    fn merge_update(&mut self, update: Self::Update) {
        if let Some(value) = update.eid { self.eid = value; }
    }

    fn merge_update_ref(&mut self, update: &Self::Update) {
        let copy = EntityIdTestUpdate {
            eid: update.eid,
        };

        self.merge_update(copy);
    }
}

#[derive(Debug, Clone, Default)]
pub struct EntityTest {
    pub entity: spatialos_sdk::worker::entity::Entity,
}

impl ObjectField for EntityTest {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            entity: input.get::<SchemaEntity>(1).map_err(Error::at_field::<Self>(1))?,
        })
    }

    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaEntity>(1, &self.entity);
    }
}

#[derive(Debug, Clone, Default)]
pub struct EntityTestUpdate {
    pub entity: Option<spatialos_sdk::worker::entity::Entity>,
}

impl Update for EntityTestUpdate {
    type Component = EntityTest;

    fn from_schema(update: &SchemaComponentUpdate) -> Result<Self> {
        Ok(Self {
            entity: update.get_field::<SchemaEntity>(1).map_err(Error::at_field::<Self>(1))?,
        })
    }

    fn into_schema(&self, update: &mut SchemaComponentUpdate) {
        update.add_field::<SchemaEntity>(1, &self.entity);
    }

    fn merge(&mut self, mut update: Self) {
        if update.entity.is_some() { self.entity = update.entity; }
    }
}



impl Component for EntityTest {
    type Update = EntityTestUpdate;

    const ID: ComponentId = 2003;

    fn merge_update(&mut self, update: Self::Update) {
        if let Some(value) = update.entity { self.entity = value; }
    }

    fn merge_update_ref(&mut self, update: &Self::Update) {
        let copy = EntityTestUpdate {
            entity: update.entity.clone(),
        };

        self.merge_update(copy);
    }
}

#[derive(Debug, Clone, Default)]
pub struct EnumTestComponent {
    pub test: generated::example::TestEnum,
}

impl ObjectField for EnumTestComponent {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            test: input.get::<generated::example::TestEnum>(1).map_err(Error::at_field::<Self>(1))?,
        })
    }

    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<generated::example::TestEnum>(1, &self.test);
    }
}

#[derive(Debug, Clone, Default)]
pub struct EnumTestComponentUpdate {
    pub test: Option<generated::example::TestEnum>,
}

impl Update for EnumTestComponentUpdate {
    type Component = EnumTestComponent;

    fn from_schema(update: &SchemaComponentUpdate) -> Result<Self> {
        Ok(Self {
            test: update.get_field::<generated::example::TestEnum>(1).map_err(Error::at_field::<Self>(1))?,
        })
    }

    fn into_schema(&self, update: &mut SchemaComponentUpdate) {
        update.add_field::<generated::example::TestEnum>(1, &self.test);
    }

    fn merge(&mut self, mut update: Self) {
        if update.test.is_some() { self.test = update.test; }
    }
}



impl Component for EnumTestComponent {
    type Update = EnumTestComponentUpdate;

    const ID: ComponentId = 2002;

    fn merge_update(&mut self, update: Self::Update) {
        if let Some(value) = update.test { self.test = value; }
    }

    fn merge_update_ref(&mut self, update: &Self::Update) {
        let copy = EnumTestComponentUpdate {
            test: update.test,
        };

        self.merge_update(copy);
    }
}

#[derive(Debug, Clone, Default)]
pub struct Example {
    pub x: FloatOrd<f32>,
}

impl ObjectField for Example {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            x: input.get::<SchemaFloat>(1).map_err(Error::at_field::<Self>(1))?,
        })
    }

    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaFloat>(1, &self.x);
    }
}

#[derive(Debug, Clone, Default)]
pub struct ExampleUpdate {
    pub x: Option<FloatOrd<f32>>,
}

impl Update for ExampleUpdate {
    type Component = Example;

    fn from_schema(update: &SchemaComponentUpdate) -> Result<Self> {
        Ok(Self {
            x: update.get_field::<SchemaFloat>(1).map_err(Error::at_field::<Self>(1))?,
        })
    }

    fn into_schema(&self, update: &mut SchemaComponentUpdate) {
        update.add_field::<SchemaFloat>(1, &self.x);
    }

    fn merge(&mut self, mut update: Self) {
        if update.x.is_some() { self.x = update.x; }
    }
}



#[derive(Debug, Clone)]
pub enum ExampleCommandRequest {
    TestCommand(generated::example::CommandData),
}

impl Request for ExampleCommandRequest {
    type Commands = Example;

    fn from_schema(command_index: CommandIndex, request: &SchemaCommandRequest) -> Result<Self> {
        match command_index {
            1 => {
                <generated::example::CommandData as ObjectField>::from_object(&request.object())
                    .map(Self::TestCommand)
            },
            _ => Err(Error::unknown_command::<Self>(command_index))
        }
    }

    fn into_schema(&self, request: &mut SchemaCommandRequest) -> CommandIndex {
        match self {
            Self::TestCommand(ref inner) => {
                <generated::example::CommandData as ObjectField>::into_object(inner, &mut request.object_mut());
                1
            }, 
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExampleCommandResponse {
    TestCommand(generated::example::CommandData),
}

impl Response for ExampleCommandResponse {
    type Commands = Example;

    fn from_schema(command_index: CommandIndex, response: &SchemaCommandResponse) -> Result<Self> {
        match command_index { 
            1 => {
                <generated::example::CommandData as ObjectField>::from_object(&response.object())
                    .map(ExampleCommandResponse::TestCommand)
            }, 
            _ => Err(Error::unknown_command::<Self>(command_index))
        }
    }

    fn into_schema(&self, response: &mut SchemaCommandResponse) -> CommandIndex {
        match self {
            Self::TestCommand(ref inner) => {
                <generated::example::CommandData as ObjectField>::into_object(inner, &mut response.object_mut());
                1
            },
        }
    }
}

impl Commands for Example {
    type Component = Example;
    type Request = ExampleCommandRequest;
    type Response = ExampleCommandResponse;
}



impl Component for Example {
    type Update = ExampleUpdate;

    const ID: ComponentId = 1000;

    fn merge_update(&mut self, update: Self::Update) {
        if let Some(value) = update.x { self.x = value; }
    }

    fn merge_update_ref(&mut self, update: &Self::Update) {
        let copy = ExampleUpdate {
            x: update.x,
        };

        self.merge_update(copy);
    }
}

#[derive(Debug, Clone, Default)]
pub struct Rotate {
    pub angle: FloatOrd<f64>,
    pub center: generated::example::Vector3d,
    pub radius: FloatOrd<f64>,
}

impl ObjectField for Rotate {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            angle: input.get::<SchemaDouble>(1).map_err(Error::at_field::<Self>(1))?,
            center: input.get::<generated::example::Vector3d>(2).map_err(Error::at_field::<Self>(2))?,
            radius: input.get::<SchemaDouble>(3).map_err(Error::at_field::<Self>(3))?,
        })
    }

    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaDouble>(1, &self.angle);
        output.add::<generated::example::Vector3d>(2, &self.center);
        output.add::<SchemaDouble>(3, &self.radius);
    }
}

#[derive(Debug, Clone, Default)]
pub struct RotateUpdate {
    pub angle: Option<FloatOrd<f64>>,
    pub center: Option<generated::example::Vector3d>,
    pub radius: Option<FloatOrd<f64>>,
}

impl Update for RotateUpdate {
    type Component = Rotate;

    fn from_schema(update: &SchemaComponentUpdate) -> Result<Self> {
        Ok(Self {
            angle: update.get_field::<SchemaDouble>(1).map_err(Error::at_field::<Self>(1))?,
            center: update.get_field::<generated::example::Vector3d>(2).map_err(Error::at_field::<Self>(2))?,
            radius: update.get_field::<SchemaDouble>(3).map_err(Error::at_field::<Self>(3))?,
        })
    }

    fn into_schema(&self, update: &mut SchemaComponentUpdate) {
        update.add_field::<SchemaDouble>(1, &self.angle);
        update.add_field::<generated::example::Vector3d>(2, &self.center);
        update.add_field::<SchemaDouble>(3, &self.radius);
    }

    fn merge(&mut self, mut update: Self) {
        if update.angle.is_some() { self.angle = update.angle; }
        if update.center.is_some() { self.center = update.center; }
        if update.radius.is_some() { self.radius = update.radius; }
    }
}



impl Component for Rotate {
    type Update = RotateUpdate;

    const ID: ComponentId = 1001;

    fn merge_update(&mut self, update: Self::Update) {
        if let Some(value) = update.angle { self.angle = value; }
        if let Some(value) = update.center { self.center = value; }
        if let Some(value) = update.radius { self.radius = value; }
    }

    fn merge_update_ref(&mut self, update: &Self::Update) {
        let copy = RotateUpdate {
            angle: update.angle,
            center: update.center.clone(),
            radius: update.radius,
        };

        self.merge_update(copy);
    }
}


}

pub mod improbable {
use spatialos_sdk::worker::schema::*;
use spatialos_sdk::worker::component::*;
use spatialos_sdk::worker::commands::*;
use std::{collections::BTreeMap, convert::TryFrom};

use super::super::generated as generated;

/* Enums. */
/* Types. */
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ComponentInterest {
    pub queries: Vec<generated::improbable::ComponentInterest_Query>,
}
impl ObjectField for ComponentInterest {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            queries: input.get::<List<generated::improbable::ComponentInterest_Query>>(1).map_err(Error::at_field::<Self>(1))?,
        })
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<List<generated::improbable::ComponentInterest_Query>>(1, &self.queries);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ComponentInterest_BoxConstraint {
    pub center: generated::improbable::Coordinates,
    pub edge_length: generated::improbable::EdgeLength,
}
impl ObjectField for ComponentInterest_BoxConstraint {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            center: input.get::<generated::improbable::Coordinates>(1).map_err(Error::at_field::<Self>(1))?,
            edge_length: input.get::<generated::improbable::EdgeLength>(2).map_err(Error::at_field::<Self>(2))?,
        })
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<generated::improbable::Coordinates>(1, &self.center);
        output.add::<generated::improbable::EdgeLength>(2, &self.edge_length);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ComponentInterest_CylinderConstraint {
    pub center: generated::improbable::Coordinates,
    pub radius: FloatOrd<f64>,
}
impl ObjectField for ComponentInterest_CylinderConstraint {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            center: input.get::<generated::improbable::Coordinates>(1).map_err(Error::at_field::<Self>(1))?,
            radius: input.get::<SchemaDouble>(2).map_err(Error::at_field::<Self>(2))?,
        })
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<generated::improbable::Coordinates>(1, &self.center);
        output.add::<SchemaDouble>(2, &self.radius);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ComponentInterest_Query {
    pub constraint: generated::improbable::ComponentInterest_QueryConstraint,
    pub full_snapshot_result: Option<bool>,
    pub result_component_id: Vec<u32>,
    pub frequency: Option<FloatOrd<f32>>,
}
impl ObjectField for ComponentInterest_Query {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            constraint: input.get::<generated::improbable::ComponentInterest_QueryConstraint>(1).map_err(Error::at_field::<Self>(1))?,
            full_snapshot_result: input.get::<Optional<SchemaBool>>(2).map_err(Error::at_field::<Self>(2))?,
            result_component_id: input.get::<List<SchemaUint32>>(3).map_err(Error::at_field::<Self>(3))?,
            frequency: input.get::<Optional<SchemaFloat>>(4).map_err(Error::at_field::<Self>(4))?,
        })
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<generated::improbable::ComponentInterest_QueryConstraint>(1, &self.constraint);
        output.add::<Optional<SchemaBool>>(2, &self.full_snapshot_result);
        output.add::<List<SchemaUint32>>(3, &self.result_component_id);
        output.add::<Optional<SchemaFloat>>(4, &self.frequency);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            sphere_constraint: input.get::<Optional<generated::improbable::ComponentInterest_SphereConstraint>>(1).map_err(Error::at_field::<Self>(1))?,
            cylinder_constraint: input.get::<Optional<generated::improbable::ComponentInterest_CylinderConstraint>>(2).map_err(Error::at_field::<Self>(2))?,
            box_constraint: input.get::<Optional<generated::improbable::ComponentInterest_BoxConstraint>>(3).map_err(Error::at_field::<Self>(3))?,
            relative_sphere_constraint: input.get::<Optional<generated::improbable::ComponentInterest_RelativeSphereConstraint>>(4).map_err(Error::at_field::<Self>(4))?,
            relative_cylinder_constraint: input.get::<Optional<generated::improbable::ComponentInterest_RelativeCylinderConstraint>>(5).map_err(Error::at_field::<Self>(5))?,
            relative_box_constraint: input.get::<Optional<generated::improbable::ComponentInterest_RelativeBoxConstraint>>(6).map_err(Error::at_field::<Self>(6))?,
            entity_id_constraint: input.get::<Optional<SchemaInt64>>(7).map_err(Error::at_field::<Self>(7))?,
            component_constraint: input.get::<Optional<SchemaUint32>>(8).map_err(Error::at_field::<Self>(8))?,
            and_constraint: input.get::<List<generated::improbable::ComponentInterest_QueryConstraint>>(9).map_err(Error::at_field::<Self>(9))?,
            or_constraint: input.get::<List<generated::improbable::ComponentInterest_QueryConstraint>>(10).map_err(Error::at_field::<Self>(10))?,
        })
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

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ComponentInterest_RelativeBoxConstraint {
    pub edge_length: generated::improbable::EdgeLength,
}
impl ObjectField for ComponentInterest_RelativeBoxConstraint {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            edge_length: input.get::<generated::improbable::EdgeLength>(1).map_err(Error::at_field::<Self>(1))?,
        })
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<generated::improbable::EdgeLength>(1, &self.edge_length);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ComponentInterest_RelativeCylinderConstraint {
    pub radius: FloatOrd<f64>,
}
impl ObjectField for ComponentInterest_RelativeCylinderConstraint {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            radius: input.get::<SchemaDouble>(1).map_err(Error::at_field::<Self>(1))?,
        })
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaDouble>(1, &self.radius);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ComponentInterest_RelativeSphereConstraint {
    pub radius: FloatOrd<f64>,
}
impl ObjectField for ComponentInterest_RelativeSphereConstraint {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            radius: input.get::<SchemaDouble>(1).map_err(Error::at_field::<Self>(1))?,
        })
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaDouble>(1, &self.radius);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ComponentInterest_SphereConstraint {
    pub center: generated::improbable::Coordinates,
    pub radius: FloatOrd<f64>,
}
impl ObjectField for ComponentInterest_SphereConstraint {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            center: input.get::<generated::improbable::Coordinates>(1).map_err(Error::at_field::<Self>(1))?,
            radius: input.get::<SchemaDouble>(2).map_err(Error::at_field::<Self>(2))?,
        })
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<generated::improbable::Coordinates>(1, &self.center);
        output.add::<SchemaDouble>(2, &self.radius);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coordinates {
    pub x: FloatOrd<f64>,
    pub y: FloatOrd<f64>,
    pub z: FloatOrd<f64>,
}
impl ObjectField for Coordinates {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            x: input.get::<SchemaDouble>(1).map_err(Error::at_field::<Self>(1))?,
            y: input.get::<SchemaDouble>(2).map_err(Error::at_field::<Self>(2))?,
            z: input.get::<SchemaDouble>(3).map_err(Error::at_field::<Self>(3))?,
        })
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaDouble>(1, &self.x);
        output.add::<SchemaDouble>(2, &self.y);
        output.add::<SchemaDouble>(3, &self.z);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EdgeLength {
    pub x: FloatOrd<f64>,
    pub y: FloatOrd<f64>,
    pub z: FloatOrd<f64>,
}
impl ObjectField for EdgeLength {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            x: input.get::<SchemaDouble>(1).map_err(Error::at_field::<Self>(1))?,
            y: input.get::<SchemaDouble>(2).map_err(Error::at_field::<Self>(2))?,
            z: input.get::<SchemaDouble>(3).map_err(Error::at_field::<Self>(3))?,
        })
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaDouble>(1, &self.x);
        output.add::<SchemaDouble>(2, &self.y);
        output.add::<SchemaDouble>(3, &self.z);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ShardedMap {
}
impl ObjectField for ShardedMap {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
        })
    }
    fn into_object(&self, output: &mut SchemaObject) {
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorkerAttributeSet {
    pub attribute: Vec<String>,
}
impl ObjectField for WorkerAttributeSet {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            attribute: input.get::<List<SchemaString>>(1).map_err(Error::at_field::<Self>(1))?,
        })
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<List<SchemaString>>(1, &self.attribute);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorkerRequirementSet {
    pub attribute_set: Vec<generated::improbable::WorkerAttributeSet>,
}
impl ObjectField for WorkerRequirementSet {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            attribute_set: input.get::<List<generated::improbable::WorkerAttributeSet>>(1).map_err(Error::at_field::<Self>(1))?,
        })
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<List<generated::improbable::WorkerAttributeSet>>(1, &self.attribute_set);
    }
}

/* Components. */ 
#[derive(Debug, Clone, Default)]
pub struct EntityAcl {
    pub read_acl: generated::improbable::WorkerRequirementSet,
    pub component_write_acl: BTreeMap<u32, generated::improbable::WorkerRequirementSet>,
}

impl ObjectField for EntityAcl {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            read_acl: input.get::<generated::improbable::WorkerRequirementSet>(1).map_err(Error::at_field::<Self>(1))?,
            component_write_acl: input.get::<Map<SchemaUint32, generated::improbable::WorkerRequirementSet>>(2).map_err(Error::at_field::<Self>(2))?,
        })
    }

    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<generated::improbable::WorkerRequirementSet>(1, &self.read_acl);
        output.add::<Map<SchemaUint32, generated::improbable::WorkerRequirementSet>>(2, &self.component_write_acl);
    }
}

#[derive(Debug, Clone, Default)]
pub struct EntityAclUpdate {
    pub read_acl: Option<generated::improbable::WorkerRequirementSet>,
    pub component_write_acl: Option<BTreeMap<u32, generated::improbable::WorkerRequirementSet>>,
}

impl Update for EntityAclUpdate {
    type Component = EntityAcl;

    fn from_schema(update: &SchemaComponentUpdate) -> Result<Self> {
        Ok(Self {
            read_acl: update.get_field::<generated::improbable::WorkerRequirementSet>(1).map_err(Error::at_field::<Self>(1))?,
            component_write_acl: update.get_field::<Map<SchemaUint32, generated::improbable::WorkerRequirementSet>>(2).map_err(Error::at_field::<Self>(2))?,
        })
    }

    fn into_schema(&self, update: &mut SchemaComponentUpdate) {
        update.add_field::<generated::improbable::WorkerRequirementSet>(1, &self.read_acl);
        update.add_field::<Map<SchemaUint32, generated::improbable::WorkerRequirementSet>>(2, &self.component_write_acl);
    }

    fn merge(&mut self, mut update: Self) {
        if update.read_acl.is_some() { self.read_acl = update.read_acl; }
        if update.component_write_acl.is_some() { self.component_write_acl = update.component_write_acl; }
    }
}



impl Component for EntityAcl {
    type Update = EntityAclUpdate;

    const ID: ComponentId = 50;

    fn merge_update(&mut self, update: Self::Update) {
        if let Some(value) = update.read_acl { self.read_acl = value; }
        if let Some(value) = update.component_write_acl { self.component_write_acl = value; }
    }

    fn merge_update_ref(&mut self, update: &Self::Update) {
        let copy = EntityAclUpdate {
            read_acl: update.read_acl.clone(),
            component_write_acl: update.component_write_acl.clone(),
        };

        self.merge_update(copy);
    }
}

#[derive(Debug, Clone, Default)]
pub struct Interest {
    pub component_interest: BTreeMap<u32, generated::improbable::ComponentInterest>,
}

impl ObjectField for Interest {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            component_interest: input.get::<Map<SchemaUint32, generated::improbable::ComponentInterest>>(1).map_err(Error::at_field::<Self>(1))?,
        })
    }

    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<Map<SchemaUint32, generated::improbable::ComponentInterest>>(1, &self.component_interest);
    }
}

#[derive(Debug, Clone, Default)]
pub struct InterestUpdate {
    pub component_interest: Option<BTreeMap<u32, generated::improbable::ComponentInterest>>,
}

impl Update for InterestUpdate {
    type Component = Interest;

    fn from_schema(update: &SchemaComponentUpdate) -> Result<Self> {
        Ok(Self {
            component_interest: update.get_field::<Map<SchemaUint32, generated::improbable::ComponentInterest>>(1).map_err(Error::at_field::<Self>(1))?,
        })
    }

    fn into_schema(&self, update: &mut SchemaComponentUpdate) {
        update.add_field::<Map<SchemaUint32, generated::improbable::ComponentInterest>>(1, &self.component_interest);
    }

    fn merge(&mut self, mut update: Self) {
        if update.component_interest.is_some() { self.component_interest = update.component_interest; }
    }
}



impl Component for Interest {
    type Update = InterestUpdate;

    const ID: ComponentId = 58;

    fn merge_update(&mut self, update: Self::Update) {
        if let Some(value) = update.component_interest { self.component_interest = value; }
    }

    fn merge_update_ref(&mut self, update: &Self::Update) {
        let copy = InterestUpdate {
            component_interest: update.component_interest.clone(),
        };

        self.merge_update(copy);
    }
}

#[derive(Debug, Clone, Default)]
pub struct Metadata {
    pub entity_type: String,
}

impl ObjectField for Metadata {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            entity_type: input.get::<SchemaString>(1).map_err(Error::at_field::<Self>(1))?,
        })
    }

    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaString>(1, &self.entity_type);
    }
}

#[derive(Debug, Clone, Default)]
pub struct MetadataUpdate {
    pub entity_type: Option<String>,
}

impl Update for MetadataUpdate {
    type Component = Metadata;

    fn from_schema(update: &SchemaComponentUpdate) -> Result<Self> {
        Ok(Self {
            entity_type: update.get_field::<SchemaString>(1).map_err(Error::at_field::<Self>(1))?,
        })
    }

    fn into_schema(&self, update: &mut SchemaComponentUpdate) {
        update.add_field::<SchemaString>(1, &self.entity_type);
    }

    fn merge(&mut self, mut update: Self) {
        if update.entity_type.is_some() { self.entity_type = update.entity_type; }
    }
}



impl Component for Metadata {
    type Update = MetadataUpdate;

    const ID: ComponentId = 53;

    fn merge_update(&mut self, update: Self::Update) {
        if let Some(value) = update.entity_type { self.entity_type = value; }
    }

    fn merge_update_ref(&mut self, update: &Self::Update) {
        let copy = MetadataUpdate {
            entity_type: update.entity_type.clone(),
        };

        self.merge_update(copy);
    }
}

#[derive(Debug, Clone, Default)]
pub struct Persistence {
}

impl ObjectField for Persistence {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
        })
    }

    fn into_object(&self, output: &mut SchemaObject) {
    }
}

#[derive(Debug, Clone, Default)]
pub struct PersistenceUpdate {
}

impl Update for PersistenceUpdate {
    type Component = Persistence;

    fn from_schema(update: &SchemaComponentUpdate) -> Result<Self> {
        Ok(Self {
        })
    }

    fn into_schema(&self, update: &mut SchemaComponentUpdate) {
    }

    fn merge(&mut self, mut update: Self) {
    }
}



impl Component for Persistence {
    type Update = PersistenceUpdate;

    const ID: ComponentId = 55;

    fn merge_update(&mut self, update: Self::Update) {
    }

    fn merge_update_ref(&mut self, update: &Self::Update) {
        let copy = PersistenceUpdate {
        };

        self.merge_update(copy);
    }
}

#[derive(Debug, Clone, Default)]
pub struct Position {
    pub coords: generated::improbable::Coordinates,
}

impl ObjectField for Position {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            coords: input.get::<generated::improbable::Coordinates>(1).map_err(Error::at_field::<Self>(1))?,
        })
    }

    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<generated::improbable::Coordinates>(1, &self.coords);
    }
}

#[derive(Debug, Clone, Default)]
pub struct PositionUpdate {
    pub coords: Option<generated::improbable::Coordinates>,
}

impl Update for PositionUpdate {
    type Component = Position;

    fn from_schema(update: &SchemaComponentUpdate) -> Result<Self> {
        Ok(Self {
            coords: update.get_field::<generated::improbable::Coordinates>(1).map_err(Error::at_field::<Self>(1))?,
        })
    }

    fn into_schema(&self, update: &mut SchemaComponentUpdate) {
        update.add_field::<generated::improbable::Coordinates>(1, &self.coords);
    }

    fn merge(&mut self, mut update: Self) {
        if update.coords.is_some() { self.coords = update.coords; }
    }
}



impl Component for Position {
    type Update = PositionUpdate;

    const ID: ComponentId = 54;

    fn merge_update(&mut self, update: Self::Update) {
        if let Some(value) = update.coords { self.coords = value; }
    }

    fn merge_update_ref(&mut self, update: &Self::Update) {
        let copy = PositionUpdate {
            coords: update.coords.clone(),
        };

        self.merge_update(copy);
    }
}



pub mod restricted {
use spatialos_sdk::worker::schema::*;
use spatialos_sdk::worker::component::*;
use spatialos_sdk::worker::commands::*;
use std::{collections::BTreeMap, convert::TryFrom};

use super::super::super::generated as generated;

/* Enums. */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
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
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Connection {
    pub status: generated::improbable::restricted::Connection_ConnectionStatus,
    pub data_latency_ms: u32,
    pub connected_since_utc: u64,
}
impl ObjectField for Connection {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            status: input.get::<generated::improbable::restricted::Connection_ConnectionStatus>(1).map_err(Error::at_field::<Self>(1))?,
            data_latency_ms: input.get::<SchemaUint32>(2).map_err(Error::at_field::<Self>(2))?,
            connected_since_utc: input.get::<SchemaUint64>(3).map_err(Error::at_field::<Self>(3))?,
        })
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<generated::improbable::restricted::Connection_ConnectionStatus>(1, &self.status);
        output.add::<SchemaUint32>(2, &self.data_latency_ms);
        output.add::<SchemaUint64>(3, &self.connected_since_utc);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DisconnectRequest {
}
impl ObjectField for DisconnectRequest {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
        })
    }
    fn into_object(&self, output: &mut SchemaObject) {
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DisconnectResponse {
}
impl ObjectField for DisconnectResponse {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
        })
    }
    fn into_object(&self, output: &mut SchemaObject) {
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlayerIdentity {
    pub player_identifier: String,
    pub provider: String,
    pub metadata: Vec<u8>,
}
impl ObjectField for PlayerIdentity {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            player_identifier: input.get::<SchemaString>(1).map_err(Error::at_field::<Self>(1))?,
            provider: input.get::<SchemaString>(2).map_err(Error::at_field::<Self>(2))?,
            metadata: input.get::<SchemaBytes>(3).map_err(Error::at_field::<Self>(3))?,
        })
    }
    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaString>(1, &self.player_identifier);
        output.add::<SchemaString>(2, &self.provider);
        output.add::<SchemaBytes>(3, &self.metadata);
    }
}

/* Components. */ 
#[derive(Debug, Clone, Default)]
pub struct PlayerClient {
    pub player_identity: generated::improbable::restricted::PlayerIdentity,
}

impl ObjectField for PlayerClient {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            player_identity: input.get::<generated::improbable::restricted::PlayerIdentity>(1).map_err(Error::at_field::<Self>(1))?,
        })
    }

    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<generated::improbable::restricted::PlayerIdentity>(1, &self.player_identity);
    }
}

#[derive(Debug, Clone, Default)]
pub struct PlayerClientUpdate {
    pub player_identity: Option<generated::improbable::restricted::PlayerIdentity>,
}

impl Update for PlayerClientUpdate {
    type Component = PlayerClient;

    fn from_schema(update: &SchemaComponentUpdate) -> Result<Self> {
        Ok(Self {
            player_identity: update.get_field::<generated::improbable::restricted::PlayerIdentity>(1).map_err(Error::at_field::<Self>(1))?,
        })
    }

    fn into_schema(&self, update: &mut SchemaComponentUpdate) {
        update.add_field::<generated::improbable::restricted::PlayerIdentity>(1, &self.player_identity);
    }

    fn merge(&mut self, mut update: Self) {
        if update.player_identity.is_some() { self.player_identity = update.player_identity; }
    }
}



impl Component for PlayerClient {
    type Update = PlayerClientUpdate;

    const ID: ComponentId = 61;

    fn merge_update(&mut self, update: Self::Update) {
        if let Some(value) = update.player_identity { self.player_identity = value; }
    }

    fn merge_update_ref(&mut self, update: &Self::Update) {
        let copy = PlayerClientUpdate {
            player_identity: update.player_identity.clone(),
        };

        self.merge_update(copy);
    }
}

#[derive(Debug, Clone, Default)]
pub struct System {
}

impl ObjectField for System {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
        })
    }

    fn into_object(&self, output: &mut SchemaObject) {
    }
}

#[derive(Debug, Clone, Default)]
pub struct SystemUpdate {
}

impl Update for SystemUpdate {
    type Component = System;

    fn from_schema(update: &SchemaComponentUpdate) -> Result<Self> {
        Ok(Self {
        })
    }

    fn into_schema(&self, update: &mut SchemaComponentUpdate) {
    }

    fn merge(&mut self, mut update: Self) {
    }
}



impl Component for System {
    type Update = SystemUpdate;

    const ID: ComponentId = 59;

    fn merge_update(&mut self, update: Self::Update) {
    }

    fn merge_update_ref(&mut self, update: &Self::Update) {
        let copy = SystemUpdate {
        };

        self.merge_update(copy);
    }
}

#[derive(Debug, Clone, Default)]
pub struct Worker {
    pub worker_id: String,
    pub worker_type: String,
    pub connection: generated::improbable::restricted::Connection,
}

impl ObjectField for Worker {
    fn from_object(input: &SchemaObject) -> Result<Self> {
        Ok(Self {
            worker_id: input.get::<SchemaString>(1).map_err(Error::at_field::<Self>(1))?,
            worker_type: input.get::<SchemaString>(2).map_err(Error::at_field::<Self>(2))?,
            connection: input.get::<generated::improbable::restricted::Connection>(3).map_err(Error::at_field::<Self>(3))?,
        })
    }

    fn into_object(&self, output: &mut SchemaObject) {
        output.add::<SchemaString>(1, &self.worker_id);
        output.add::<SchemaString>(2, &self.worker_type);
        output.add::<generated::improbable::restricted::Connection>(3, &self.connection);
    }
}

#[derive(Debug, Clone, Default)]
pub struct WorkerUpdate {
    pub worker_id: Option<String>,
    pub worker_type: Option<String>,
    pub connection: Option<generated::improbable::restricted::Connection>,
}

impl Update for WorkerUpdate {
    type Component = Worker;

    fn from_schema(update: &SchemaComponentUpdate) -> Result<Self> {
        Ok(Self {
            worker_id: update.get_field::<SchemaString>(1).map_err(Error::at_field::<Self>(1))?,
            worker_type: update.get_field::<SchemaString>(2).map_err(Error::at_field::<Self>(2))?,
            connection: update.get_field::<generated::improbable::restricted::Connection>(3).map_err(Error::at_field::<Self>(3))?,
        })
    }

    fn into_schema(&self, update: &mut SchemaComponentUpdate) {
        update.add_field::<SchemaString>(1, &self.worker_id);
        update.add_field::<SchemaString>(2, &self.worker_type);
        update.add_field::<generated::improbable::restricted::Connection>(3, &self.connection);
    }

    fn merge(&mut self, mut update: Self) {
        if update.worker_id.is_some() { self.worker_id = update.worker_id; }
        if update.worker_type.is_some() { self.worker_type = update.worker_type; }
        if update.connection.is_some() { self.connection = update.connection; }
    }
}



#[derive(Debug, Clone)]
pub enum WorkerCommandRequest {
    Disconnect(generated::improbable::restricted::DisconnectRequest),
}

impl Request for WorkerCommandRequest {
    type Commands = Worker;

    fn from_schema(command_index: CommandIndex, request: &SchemaCommandRequest) -> Result<Self> {
        match command_index {
            1 => {
                <generated::improbable::restricted::DisconnectRequest as ObjectField>::from_object(&request.object())
                    .map(Self::Disconnect)
            },
            _ => Err(Error::unknown_command::<Self>(command_index))
        }
    }

    fn into_schema(&self, request: &mut SchemaCommandRequest) -> CommandIndex {
        match self {
            Self::Disconnect(ref inner) => {
                <generated::improbable::restricted::DisconnectRequest as ObjectField>::into_object(inner, &mut request.object_mut());
                1
            }, 
        }
    }
}

#[derive(Debug, Clone)]
pub enum WorkerCommandResponse {
    Disconnect(generated::improbable::restricted::DisconnectResponse),
}

impl Response for WorkerCommandResponse {
    type Commands = Worker;

    fn from_schema(command_index: CommandIndex, response: &SchemaCommandResponse) -> Result<Self> {
        match command_index { 
            1 => {
                <generated::improbable::restricted::DisconnectResponse as ObjectField>::from_object(&response.object())
                    .map(WorkerCommandResponse::Disconnect)
            }, 
            _ => Err(Error::unknown_command::<Self>(command_index))
        }
    }

    fn into_schema(&self, response: &mut SchemaCommandResponse) -> CommandIndex {
        match self {
            Self::Disconnect(ref inner) => {
                <generated::improbable::restricted::DisconnectResponse as ObjectField>::into_object(inner, &mut response.object_mut());
                1
            },
        }
    }
}

impl Commands for Worker {
    type Component = Worker;
    type Request = WorkerCommandRequest;
    type Response = WorkerCommandResponse;
}



impl Component for Worker {
    type Update = WorkerUpdate;

    const ID: ComponentId = 60;

    fn merge_update(&mut self, update: Self::Update) {
        if let Some(value) = update.worker_id { self.worker_id = value; }
        if let Some(value) = update.worker_type { self.worker_type = value; }
        if let Some(value) = update.connection { self.connection = value; }
    }

    fn merge_update_ref(&mut self, update: &Self::Update) {
        let copy = WorkerUpdate {
            worker_id: update.worker_id.clone(),
            worker_type: update.worker_type.clone(),
            connection: update.connection.clone(),
        };

        self.merge_update(copy);
    }
}


}
}
