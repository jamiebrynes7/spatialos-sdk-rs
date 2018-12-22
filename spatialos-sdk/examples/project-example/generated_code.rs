use spatialos_sdk::worker::internal::schema::{self, SchemaField};
use spatialos_sdk::worker::{ComponentMetaclass, ComponentUpdate, ComponentData, ComponentVtable, ComponentId};

use self as generated_code;

/* Enums. */
/* Types. */
/* Components. */ 


mod example {
use spatialos_sdk::worker::internal::schema::{self, SchemaField};
use spatialos_sdk::worker::{ComponentMetaclass, ComponentUpdate, ComponentData, ComponentVtable, ComponentId};

use super as generated_code;

/* Enums. */
/* Types. */
#[derive(Debug)]
pub struct CommandData {
    value: i32,
}
impl TypeSerializer<CommandData> for CommandData {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        output.field::<i32>.add(input.value);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            value: input.field::<i32>.get_or_default(),
        }
    }
}

#[derive(Debug)]
pub struct ExampleData {
    x: f32,
}
impl TypeSerializer<ExampleData> for ExampleData {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        output.field::<f32>.add(input.x);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            x: input.field::<f32>.get_or_default(),
        }
    }
}

/* Components. */ 
#[derive(Debug)]
pub struct Example {
    x: f32,
}
impl TypeSerializer<Example> for Example {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        output.field::<f32>.add(input.x);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            x: input.field::<f32>.get_or_default(),
        }
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
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        UNKNOWN (UpdateField);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            x: input.field::<f32>.get(),
        }
    }
}
impl ComponentUpdate<Example> for  generated_code::example::ExampleUpdate {
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
    fn serialize_data(data: &Self::Data) -> schema::SchemaComponentData {
        let mut serialized_data = schema::SchemaComponentData::new(Self::component_id());
        TypeSerializer<Self::Data>::serialize(data, serialized_data.fields_mut());
        serialized_data
    }

    fn deserialize_data(data: &schema::SchemaComponentData) -> Option<Self::Data> {
        TypeSerializer<Self::Data>::deserialize(data.fields())
    }

    fn serialize_update(update: &Self::Update) -> schema::SchemaComponentUpdate {
        let mut serialized_update = schema::SchemaComponentUpdate::new(Self::component_id());
        TypeSerializer<Self::Update>::serialize(update, serialized_update.fields_mut());
        serialized_update
    }

    fn deserialize_update(update: &schema::SchemaComponentUpdate) -> Option<Self::Update> {
        TypeSerializer<Self::Update>::deserialize(update.fields())
    }

    fn serialize_command_request(request: &Self::CommandRequest) -> schema::SchemaCommandRequest {
        let command_index = match request {
            ExampleCommandRequest::test_command(_) => 1,
        };
        let mut serialized_request = schema::SchemaCommandRequest::new(Self::component_id(), command_index);
        match request {
            ExampleCommandRequest::test_command(ref data) => {
                TypeSerializer<generated_code::example::CommandData>::serialize(data, serialized_request.object_mut());
            },
        }
        serialized_request
    }

    fn deserialize_command_request(request: &schema::SchemaCommandRequest) -> Option<Self::CommandRequest> {
        match request.command_index() {
            1 => {
                Some(ExampleCommandRequest::test_command(
                    TypeSerializer<generated_code::example::CommandData>::deserialize(request.object());
                ))
            },
            _ => None
        }
    }

    fn serialize_command_response(response: &Self::CommandResponse) -> schema::SchemaCommandResponse {
        let command_index = match response {
            ExampleCommandResponse::test_command(_) => 1,
        };
        let mut serialized_response = schema::SchemaCommandResponse::new(Self::component_id(), command_index);
        match response {
            ExampleCommandResponse::test_command(ref data) => {
                TypeSerializer<generated_code::example::CommandData>::serialize(data, serialized_response.object_mut());
            },
        }
        serialized_response
    }

    fn deserialize_command_response(response: &schema::SchemaCommandResponse) -> Option<Self::CommandResponse> {
        match response.command_index() {
            1 => {
                Some(ExampleCommandResponse::test_command(
                    TypeSerializer<generated_code::example::CommandData>::deserialize(response.object());
                ))
            },
            _ => None
        }
    }
}



}

mod improbable {
use spatialos_sdk::worker::internal::schema::{self, SchemaField};
use spatialos_sdk::worker::{ComponentMetaclass, ComponentUpdate, ComponentData, ComponentVtable, ComponentId};

use super as generated_code;

/* Enums. */
/* Types. */
#[derive(Debug)]
pub struct PersistenceData {
}
impl TypeSerializer<PersistenceData> for PersistenceData {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
        }
    }
}

#[derive(Debug)]
pub struct WorkerAttributeSet {
    attribute: Vec<String>,
}
impl TypeSerializer<WorkerAttributeSet> for WorkerAttributeSet {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        UNKNOWN (ListType);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            attribute: UNKNOWN (ListType),
        }
    }
}

#[derive(Debug)]
pub struct Coordinates {
    x: f64,
    y: f64,
    z: f64,
}
impl TypeSerializer<Coordinates> for Coordinates {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        output.field::<f64>.add(input.x);
        output.field::<f64>.add(input.y);
        output.field::<f64>.add(input.z);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            x: input.field::<f64>.get_or_default(),
            y: input.field::<f64>.get_or_default(),
            z: input.field::<f64>.get_or_default(),
        }
    }
}

#[derive(Debug)]
pub struct WorkerRequirementSet {
    attribute_set: Vec<generated_code::improbable::WorkerAttributeSet>,
}
impl TypeSerializer<WorkerRequirementSet> for WorkerRequirementSet {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        UNKNOWN (ListType);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            attribute_set: UNKNOWN (ListType),
        }
    }
}

#[derive(Debug)]
pub struct EntityAclData {
    read_acl: generated_code::improbable::WorkerRequirementSet,
    component_write_acl: HashMap<u32, generated_code::improbable::WorkerRequirementSet>,
}
impl TypeSerializer<EntityAclData> for EntityAclData {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        UNKNOWN (TypeRef);
        UNKNOWN (MapType);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            read_acl: UNKNOWN (TypeRef),
            component_write_acl: UNKNOWN (MapType),
        }
    }
}

#[derive(Debug)]
pub struct Vector3d {
    x: f64,
    y: f64,
    z: f64,
}
impl TypeSerializer<Vector3d> for Vector3d {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        output.field::<f64>.add(input.x);
        output.field::<f64>.add(input.y);
        output.field::<f64>.add(input.z);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            x: input.field::<f64>.get_or_default(),
            y: input.field::<f64>.get_or_default(),
            z: input.field::<f64>.get_or_default(),
        }
    }
}

#[derive(Debug)]
pub struct Vector3f {
    x: f32,
    y: f32,
    z: f32,
}
impl TypeSerializer<Vector3f> for Vector3f {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        output.field::<f32>.add(input.x);
        output.field::<f32>.add(input.y);
        output.field::<f32>.add(input.z);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            x: input.field::<f32>.get_or_default(),
            y: input.field::<f32>.get_or_default(),
            z: input.field::<f32>.get_or_default(),
        }
    }
}

#[derive(Debug)]
pub struct InterestData {
    component_interest: HashMap<u32, generated_code::improbable::ComponentInterest>,
}
impl TypeSerializer<InterestData> for InterestData {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        UNKNOWN (MapType);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            component_interest: UNKNOWN (MapType),
        }
    }
}

#[derive(Debug)]
pub struct EdgeLength {
    x: f64,
    y: f64,
    z: f64,
}
impl TypeSerializer<EdgeLength> for EdgeLength {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        output.field::<f64>.add(input.x);
        output.field::<f64>.add(input.y);
        output.field::<f64>.add(input.z);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            x: input.field::<f64>.get_or_default(),
            y: input.field::<f64>.get_or_default(),
            z: input.field::<f64>.get_or_default(),
        }
    }
}

#[derive(Debug)]
pub struct MetadataData {
    entity_type: String,
}
impl TypeSerializer<MetadataData> for MetadataData {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        output.field::<String>.add(input.entity_type);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            entity_type: input.field::<String>.get_or_default(),
        }
    }
}

#[derive(Debug)]
pub struct PositionData {
    coords: generated_code::improbable::Coordinates,
}
impl TypeSerializer<PositionData> for PositionData {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        UNKNOWN (TypeRef);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            coords: UNKNOWN (TypeRef),
        }
    }
}

#[derive(Debug)]
pub struct ComponentInterest {
    queries: Vec<generated_code::improbable::ComponentInterest::Query>,
}
impl TypeSerializer<ComponentInterest> for ComponentInterest {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        UNKNOWN (ListType);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            queries: UNKNOWN (ListType),
        }
    }
}

/* Components. */ 
#[derive(Debug)]
pub struct Persistence {
}
impl TypeSerializer<Persistence> for Persistence {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
        }
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
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
        }
    }
}
impl ComponentUpdate<Persistence> for  generated_code::improbable::PersistenceUpdate {
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
    fn serialize_data(data: &Self::Data) -> schema::SchemaComponentData {
        let mut serialized_data = schema::SchemaComponentData::new(Self::component_id());
        TypeSerializer<Self::Data>::serialize(data, serialized_data.fields_mut());
        serialized_data
    }

    fn deserialize_data(data: &schema::SchemaComponentData) -> Option<Self::Data> {
        TypeSerializer<Self::Data>::deserialize(data.fields())
    }

    fn serialize_update(update: &Self::Update) -> schema::SchemaComponentUpdate {
        let mut serialized_update = schema::SchemaComponentUpdate::new(Self::component_id());
        TypeSerializer<Self::Update>::serialize(update, serialized_update.fields_mut());
        serialized_update
    }

    fn deserialize_update(update: &schema::SchemaComponentUpdate) -> Option<Self::Update> {
        TypeSerializer<Self::Update>::deserialize(update.fields())
    }

    fn serialize_command_request(request: &Self::CommandRequest) -> schema::SchemaCommandRequest {
        let command_index = match request {
        };
        let mut serialized_request = schema::SchemaCommandRequest::new(Self::component_id(), command_index);
        match request {
        }
        serialized_request
    }

    fn deserialize_command_request(request: &schema::SchemaCommandRequest) -> Option<Self::CommandRequest> {
        match request.command_index() {
            _ => None
        }
    }

    fn serialize_command_response(response: &Self::CommandResponse) -> schema::SchemaCommandResponse {
        let command_index = match response {
        };
        let mut serialized_response = schema::SchemaCommandResponse::new(Self::component_id(), command_index);
        match response {
        }
        serialized_response
    }

    fn deserialize_command_response(response: &schema::SchemaCommandResponse) -> Option<Self::CommandResponse> {
        match response.command_index() {
            _ => None
        }
    }
}

#[derive(Debug)]
pub struct Metadata {
    entity_type: String,
}
impl TypeSerializer<Metadata> for Metadata {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        output.field::<String>.add(input.entity_type);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            entity_type: input.field::<String>.get_or_default(),
        }
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
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        UNKNOWN (UpdateField);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            entity_type: input.field::<String>.get(),
        }
    }
}
impl ComponentUpdate<Metadata> for  generated_code::improbable::MetadataUpdate {
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
    fn serialize_data(data: &Self::Data) -> schema::SchemaComponentData {
        let mut serialized_data = schema::SchemaComponentData::new(Self::component_id());
        TypeSerializer<Self::Data>::serialize(data, serialized_data.fields_mut());
        serialized_data
    }

    fn deserialize_data(data: &schema::SchemaComponentData) -> Option<Self::Data> {
        TypeSerializer<Self::Data>::deserialize(data.fields())
    }

    fn serialize_update(update: &Self::Update) -> schema::SchemaComponentUpdate {
        let mut serialized_update = schema::SchemaComponentUpdate::new(Self::component_id());
        TypeSerializer<Self::Update>::serialize(update, serialized_update.fields_mut());
        serialized_update
    }

    fn deserialize_update(update: &schema::SchemaComponentUpdate) -> Option<Self::Update> {
        TypeSerializer<Self::Update>::deserialize(update.fields())
    }

    fn serialize_command_request(request: &Self::CommandRequest) -> schema::SchemaCommandRequest {
        let command_index = match request {
        };
        let mut serialized_request = schema::SchemaCommandRequest::new(Self::component_id(), command_index);
        match request {
        }
        serialized_request
    }

    fn deserialize_command_request(request: &schema::SchemaCommandRequest) -> Option<Self::CommandRequest> {
        match request.command_index() {
            _ => None
        }
    }

    fn serialize_command_response(response: &Self::CommandResponse) -> schema::SchemaCommandResponse {
        let command_index = match response {
        };
        let mut serialized_response = schema::SchemaCommandResponse::new(Self::component_id(), command_index);
        match response {
        }
        serialized_response
    }

    fn deserialize_command_response(response: &schema::SchemaCommandResponse) -> Option<Self::CommandResponse> {
        match response.command_index() {
            _ => None
        }
    }
}

#[derive(Debug)]
pub struct Interest {
    component_interest: HashMap<u32, generated_code::improbable::ComponentInterest>,
}
impl TypeSerializer<Interest> for Interest {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        UNKNOWN (MapType);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            component_interest: UNKNOWN (MapType),
        }
    }
}
impl ComponentData<Interest> for generated_code::improbable::Interest {
    fn merge(&mut self, update: generated_code::improbable::InterestUpdate) {
        if let Some(value) = update.component_interest { self.component_interest = Some(value); }
    }
}

#[derive(Debug)]
pub struct InterestUpdate {
    component_interest: Option<HashMap<u32, generated_code::improbable::ComponentInterest>>,
}
impl TypeSerializer<InterestUpdate> for InterestUpdate {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        UNKNOWN (UpdateField);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            component_interest: UNKNOWN (MapType Update),
        }
    }
}
impl ComponentUpdate<Interest> for  generated_code::improbable::InterestUpdate {
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
    fn serialize_data(data: &Self::Data) -> schema::SchemaComponentData {
        let mut serialized_data = schema::SchemaComponentData::new(Self::component_id());
        TypeSerializer<Self::Data>::serialize(data, serialized_data.fields_mut());
        serialized_data
    }

    fn deserialize_data(data: &schema::SchemaComponentData) -> Option<Self::Data> {
        TypeSerializer<Self::Data>::deserialize(data.fields())
    }

    fn serialize_update(update: &Self::Update) -> schema::SchemaComponentUpdate {
        let mut serialized_update = schema::SchemaComponentUpdate::new(Self::component_id());
        TypeSerializer<Self::Update>::serialize(update, serialized_update.fields_mut());
        serialized_update
    }

    fn deserialize_update(update: &schema::SchemaComponentUpdate) -> Option<Self::Update> {
        TypeSerializer<Self::Update>::deserialize(update.fields())
    }

    fn serialize_command_request(request: &Self::CommandRequest) -> schema::SchemaCommandRequest {
        let command_index = match request {
        };
        let mut serialized_request = schema::SchemaCommandRequest::new(Self::component_id(), command_index);
        match request {
        }
        serialized_request
    }

    fn deserialize_command_request(request: &schema::SchemaCommandRequest) -> Option<Self::CommandRequest> {
        match request.command_index() {
            _ => None
        }
    }

    fn serialize_command_response(response: &Self::CommandResponse) -> schema::SchemaCommandResponse {
        let command_index = match response {
        };
        let mut serialized_response = schema::SchemaCommandResponse::new(Self::component_id(), command_index);
        match response {
        }
        serialized_response
    }

    fn deserialize_command_response(response: &schema::SchemaCommandResponse) -> Option<Self::CommandResponse> {
        match response.command_index() {
            _ => None
        }
    }
}

#[derive(Debug)]
pub struct Position {
    coords: generated_code::improbable::Coordinates,
}
impl TypeSerializer<Position> for Position {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        UNKNOWN (TypeRef);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            coords: UNKNOWN (TypeRef),
        }
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
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        UNKNOWN (UpdateField);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            coords: UNKNOWN (TypeRef),
        }
    }
}
impl ComponentUpdate<Position> for  generated_code::improbable::PositionUpdate {
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
    fn serialize_data(data: &Self::Data) -> schema::SchemaComponentData {
        let mut serialized_data = schema::SchemaComponentData::new(Self::component_id());
        TypeSerializer<Self::Data>::serialize(data, serialized_data.fields_mut());
        serialized_data
    }

    fn deserialize_data(data: &schema::SchemaComponentData) -> Option<Self::Data> {
        TypeSerializer<Self::Data>::deserialize(data.fields())
    }

    fn serialize_update(update: &Self::Update) -> schema::SchemaComponentUpdate {
        let mut serialized_update = schema::SchemaComponentUpdate::new(Self::component_id());
        TypeSerializer<Self::Update>::serialize(update, serialized_update.fields_mut());
        serialized_update
    }

    fn deserialize_update(update: &schema::SchemaComponentUpdate) -> Option<Self::Update> {
        TypeSerializer<Self::Update>::deserialize(update.fields())
    }

    fn serialize_command_request(request: &Self::CommandRequest) -> schema::SchemaCommandRequest {
        let command_index = match request {
        };
        let mut serialized_request = schema::SchemaCommandRequest::new(Self::component_id(), command_index);
        match request {
        }
        serialized_request
    }

    fn deserialize_command_request(request: &schema::SchemaCommandRequest) -> Option<Self::CommandRequest> {
        match request.command_index() {
            _ => None
        }
    }

    fn serialize_command_response(response: &Self::CommandResponse) -> schema::SchemaCommandResponse {
        let command_index = match response {
        };
        let mut serialized_response = schema::SchemaCommandResponse::new(Self::component_id(), command_index);
        match response {
        }
        serialized_response
    }

    fn deserialize_command_response(response: &schema::SchemaCommandResponse) -> Option<Self::CommandResponse> {
        match response.command_index() {
            _ => None
        }
    }
}

#[derive(Debug)]
pub struct EntityAcl {
    read_acl: generated_code::improbable::WorkerRequirementSet,
    component_write_acl: HashMap<u32, generated_code::improbable::WorkerRequirementSet>,
}
impl TypeSerializer<EntityAcl> for EntityAcl {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        UNKNOWN (TypeRef);
        UNKNOWN (MapType);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            read_acl: UNKNOWN (TypeRef),
            component_write_acl: UNKNOWN (MapType),
        }
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
    component_write_acl: Option<HashMap<u32, generated_code::improbable::WorkerRequirementSet>>,
}
impl TypeSerializer<EntityAclUpdate> for EntityAclUpdate {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        UNKNOWN (UpdateField);
        UNKNOWN (UpdateField);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            read_acl: UNKNOWN (TypeRef),
            component_write_acl: UNKNOWN (MapType Update),
        }
    }
}
impl ComponentUpdate<EntityAcl> for  generated_code::improbable::EntityAclUpdate {
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
    fn serialize_data(data: &Self::Data) -> schema::SchemaComponentData {
        let mut serialized_data = schema::SchemaComponentData::new(Self::component_id());
        TypeSerializer<Self::Data>::serialize(data, serialized_data.fields_mut());
        serialized_data
    }

    fn deserialize_data(data: &schema::SchemaComponentData) -> Option<Self::Data> {
        TypeSerializer<Self::Data>::deserialize(data.fields())
    }

    fn serialize_update(update: &Self::Update) -> schema::SchemaComponentUpdate {
        let mut serialized_update = schema::SchemaComponentUpdate::new(Self::component_id());
        TypeSerializer<Self::Update>::serialize(update, serialized_update.fields_mut());
        serialized_update
    }

    fn deserialize_update(update: &schema::SchemaComponentUpdate) -> Option<Self::Update> {
        TypeSerializer<Self::Update>::deserialize(update.fields())
    }

    fn serialize_command_request(request: &Self::CommandRequest) -> schema::SchemaCommandRequest {
        let command_index = match request {
        };
        let mut serialized_request = schema::SchemaCommandRequest::new(Self::component_id(), command_index);
        match request {
        }
        serialized_request
    }

    fn deserialize_command_request(request: &schema::SchemaCommandRequest) -> Option<Self::CommandRequest> {
        match request.command_index() {
            _ => None
        }
    }

    fn serialize_command_response(response: &Self::CommandResponse) -> schema::SchemaCommandResponse {
        let command_index = match response {
        };
        let mut serialized_response = schema::SchemaCommandResponse::new(Self::component_id(), command_index);
        match response {
        }
        serialized_response
    }

    fn deserialize_command_response(response: &schema::SchemaCommandResponse) -> Option<Self::CommandResponse> {
        match response.command_index() {
            _ => None
        }
    }
}



mod ComponentInterest {
use spatialos_sdk::worker::internal::schema::{self, SchemaField};
use spatialos_sdk::worker::{ComponentMetaclass, ComponentUpdate, ComponentData, ComponentVtable, ComponentId};

use super::super as generated_code;

/* Enums. */
/* Types. */
#[derive(Debug)]
pub struct RelativeCylinderConstraint {
    radius: f64,
}
impl TypeSerializer<RelativeCylinderConstraint> for RelativeCylinderConstraint {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        output.field::<f64>.add(input.radius);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            radius: input.field::<f64>.get_or_default(),
        }
    }
}

#[derive(Debug)]
pub struct CylinderConstraint {
    center: generated_code::improbable::Coordinates,
    radius: f64,
}
impl TypeSerializer<CylinderConstraint> for CylinderConstraint {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        UNKNOWN (TypeRef);
        output.field::<f64>.add(input.radius);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            center: UNKNOWN (TypeRef),
            radius: input.field::<f64>.get_or_default(),
        }
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
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        UNKNOWN (TypeRef);
        UNKNOWN (OptionType);
        UNKNOWN (ListType);
        UNKNOWN (OptionType);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            constraint: UNKNOWN (TypeRef),
            full_snapshot_result: UNKNOWN (OptionType),
            result_component_id: UNKNOWN (ListType),
            frequency: UNKNOWN (OptionType),
        }
    }
}

#[derive(Debug)]
pub struct SphereConstraint {
    center: generated_code::improbable::Coordinates,
    radius: f64,
}
impl TypeSerializer<SphereConstraint> for SphereConstraint {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        UNKNOWN (TypeRef);
        output.field::<f64>.add(input.radius);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            center: UNKNOWN (TypeRef),
            radius: input.field::<f64>.get_or_default(),
        }
    }
}

#[derive(Debug)]
pub struct BoxConstraint {
    center: generated_code::improbable::Coordinates,
    edge_length: generated_code::improbable::EdgeLength,
}
impl TypeSerializer<BoxConstraint> for BoxConstraint {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        UNKNOWN (TypeRef);
        UNKNOWN (TypeRef);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            center: UNKNOWN (TypeRef),
            edge_length: UNKNOWN (TypeRef),
        }
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
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        UNKNOWN (OptionType);
        UNKNOWN (OptionType);
        UNKNOWN (OptionType);
        UNKNOWN (OptionType);
        UNKNOWN (OptionType);
        UNKNOWN (OptionType);
        UNKNOWN (OptionType);
        UNKNOWN (OptionType);
        UNKNOWN (ListType);
        UNKNOWN (ListType);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            sphere_constraint: UNKNOWN (OptionType),
            cylinder_constraint: UNKNOWN (OptionType),
            box_constraint: UNKNOWN (OptionType),
            relative_sphere_constraint: UNKNOWN (OptionType),
            relative_cylinder_constraint: UNKNOWN (OptionType),
            relative_box_constraint: UNKNOWN (OptionType),
            entity_id_constraint: UNKNOWN (OptionType),
            component_constraint: UNKNOWN (OptionType),
            and_constraint: UNKNOWN (ListType),
            or_constraint: UNKNOWN (ListType),
        }
    }
}

#[derive(Debug)]
pub struct RelativeSphereConstraint {
    radius: f64,
}
impl TypeSerializer<RelativeSphereConstraint> for RelativeSphereConstraint {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        output.field::<f64>.add(input.radius);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            radius: input.field::<f64>.get_or_default(),
        }
    }
}

#[derive(Debug)]
pub struct RelativeBoxConstraint {
    edge_length: generated_code::improbable::EdgeLength,
}
impl TypeSerializer<RelativeBoxConstraint> for RelativeBoxConstraint {
    fn serialize(input: &Self, output: &mut schema::SchemaObject) {
        UNKNOWN (TypeRef);
    }
    fn deserialize(input: &schema::SchemaObject) -> Self {
        Self {
            edge_length: UNKNOWN (TypeRef),
        }
    }
}

/* Components. */ 


}

}
