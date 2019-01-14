use spatialos_sdk_sys::worker::*;
use std::marker::PhantomData;
use crate::worker::component::ComponentId;

pub type FieldId = u32;

pub struct SchemaComponentUpdate {
    pub component_id: ComponentId,
    pub internal: *mut Schema_ComponentUpdate,
}

pub struct SchemaComponentData {
    pub component_id: ComponentId,
    pub internal: *mut Schema_ComponentData,
}

pub struct SchemaCommandRequest {
    pub component_id: ComponentId,
    pub internal: *mut Schema_CommandRequest,
}

pub struct SchemaCommandResponse {
    pub component_id: ComponentId,
    pub internal: *mut Schema_CommandResponse,
}

pub struct SchemaObject {
    internal: *mut Schema_Object,
}

pub struct SchemaFieldContainer<'a, T> {
    field_id: FieldId,
    container: &'a SchemaObject,
    _phantom: PhantomData<T>,
}

pub trait SchemaField<T> {
    fn get(&self) -> Option<T> {
        if self.count() == 0 { None } else { Some(self.get_or_default()) }
    }

    fn get_or_default(&self) -> T;
    fn index(&self, index: usize) -> T;
    fn count(&self) -> usize;

    fn add(&mut self, value: T);
    fn add_list(&mut self, value: &[T]);
}

pub trait SchemaObjectField {
    fn get(&self) -> Option<SchemaObject> {
        if self.count() == 0 { None } else { Some(self.get_or_add()) }
    }

    fn get_or_add(&self) -> SchemaObject;
    fn index(&self, index: usize) -> SchemaObject;
    fn count(&self) -> usize;

    fn add(&mut self) -> SchemaObject;
}

impl SchemaComponentUpdate {
    pub(crate) fn from_worker_sdk(component_id: ComponentId, component_update: *mut Schema_ComponentUpdate) -> SchemaComponentUpdate {
        SchemaComponentUpdate {
            component_id: component_id,
            internal: component_update
        }
    }

    pub fn new(component_id: ComponentId) -> SchemaComponentUpdate {
        SchemaComponentUpdate {
            component_id: component_id,
            internal: unsafe { Schema_CreateComponentUpdate(component_id) }
        }
    }

    pub fn component_id(&self) -> ComponentId {
        unsafe { Schema_GetComponentUpdateComponentId(self.internal) }
    }

    pub fn fields(&self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetComponentUpdateFields(self.internal) }
        }
    }

    pub fn fields_mut(&mut self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetComponentUpdateFields(self.internal) }
        }
    }

    pub fn events(&self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetComponentUpdateEvents(self.internal) }
        }
    }

    pub fn events_mut(&mut self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetComponentUpdateEvents(self.internal) }
        }
    }

    // TODO: Cleared fields.
}

impl SchemaComponentData {
    pub(crate) fn from_worker_sdk(component_id: ComponentId, component_data: *mut Schema_ComponentData) -> SchemaComponentData {
        SchemaComponentData {
            component_id: component_id,
            internal: component_data
        }
    }

    pub fn new(component_id: ComponentId) -> SchemaComponentData {
        SchemaComponentData {
            component_id: component_id,
            internal: unsafe { Schema_CreateComponentData(component_id) }
        }
    }

    pub fn component_id(&self) -> ComponentId {
        unsafe { Schema_GetComponentDataComponentId(self.internal) }
    }

    pub fn fields(&self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetComponentDataFields(self.internal) }
        }
    }

    pub fn fields_mut(&mut self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetComponentDataFields(self.internal) }
        }
    }
}

impl SchemaCommandRequest {
    pub(crate) fn from_worker_sdk(component_id: ComponentId, command_request: *mut Schema_CommandRequest) -> SchemaCommandRequest {
        SchemaCommandRequest {
            component_id: component_id,
            internal: command_request
        }
    }

    pub fn new(component_id: ComponentId, command_index: FieldId) -> SchemaCommandRequest {
        SchemaCommandRequest {
            component_id: component_id,
            internal: unsafe { Schema_CreateCommandRequest(component_id, command_index) }
        }
    }

    pub fn component_id(&self) -> ComponentId {
        unsafe { Schema_GetCommandRequestComponentId(self.internal) }
    }

    pub fn command_index(&self) -> FieldId {
        unsafe { Schema_GetCommandRequestCommandIndex(self.internal) }
    }

    pub fn object(&self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetCommandRequestObject(self.internal) }
        }
    }

    pub fn object_mut(&mut self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetCommandRequestObject(self.internal) }
        }
    }
}

impl SchemaCommandResponse {
    pub(crate) fn from_worker_sdk(component_id: u32, command_response: *mut Schema_CommandResponse) -> SchemaCommandResponse {
        SchemaCommandResponse {
            component_id: component_id,
            internal: command_response
        }
    }

    pub fn new(component_id: u32, command_index: u32) -> SchemaCommandResponse {
        SchemaCommandResponse {
            component_id: component_id,
            internal: unsafe { Schema_CreateCommandResponse(component_id, command_index) }
        }
    }

    pub fn component_id(&self) -> ComponentId {
        unsafe { Schema_GetCommandResponseComponentId(self.internal) }
    }

    pub fn command_index(&self) -> FieldId {
        unsafe { Schema_GetCommandResponseCommandIndex(self.internal) }
    }

    pub fn object(&self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetCommandResponseObject(self.internal) }
        }
    }

    pub fn object_mut(&mut self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetCommandResponseObject(self.internal) }
        }
    }
}

impl SchemaObject {
    pub fn field<T>(&self, field_id: ComponentId) -> SchemaFieldContainer<T> {
        SchemaFieldContainer { field_id: field_id, container: self, _phantom: PhantomData }
    }
}

macro_rules! impl_primitive_field {
    ($rust_type:ty, $schema_get:ident, $schema_index:ident, $schema_count:ident, $schema_add:ident, $schema_add_list:ident) => (
        impl<'a> SchemaField<$rust_type> for SchemaFieldContainer<'a, $rust_type> {
            fn get_or_default(&self) -> $rust_type {
                unsafe { $schema_get(self.container.internal, self.field_id) }
            }
            fn index(&self, index: usize) -> $rust_type {
                unsafe { $schema_index(self.container.internal, self.field_id, index as u32) }
            }
            fn count(&self) -> usize {
                unsafe { $schema_count(self.container.internal, self.field_id) as usize }
            }

            fn add(&mut self, value: $rust_type) {
                unsafe { $schema_add(self.container.internal, self.field_id, value); }
            }
            fn add_list(&mut self, value: &[$rust_type]) {
                unsafe {
                    let ptr = value.as_ptr();
                    $schema_add_list(self.container.internal, self.field_id, ptr, value.len() as u32);
                }
            }
        }
    )
}

/*

Better API design:

struct SchemaObject {
    fn float_field(field_id) -> SchemaFieldContainer<SchemaType::Float> {
        ...
    }
    fn double_field(field_id) -> SchemaFieldContainer<SchemaType::Double> {
        ...
    }
    ...
    fn sint32_field(field_id) -> SchemaFieldContainer<SchemaType::Sint32> {
        ...
    }
}

*/

/*
pub fn Schema_GetFloatCount(object: *const Schema_Object, field_id: Schema_FieldId) -> u32;
pub fn Schema_GetDoubleCount(object: *const Schema_Object, field_id: Schema_FieldId) -> u32;
pub fn Schema_GetBoolCount(object: *const Schema_Object, field_id: Schema_FieldId) -> u32;
pub fn Schema_GetInt32Count(object: *const Schema_Object, field_id: Schema_FieldId) -> u32;
pub fn Schema_GetInt64Count(object: *const Schema_Object, field_id: Schema_FieldId) -> u32;
pub fn Schema_GetUint32Count(object: *const Schema_Object, field_id: Schema_FieldId) -> u32;
pub fn Schema_GetUint64Count(object: *const Schema_Object, field_id: Schema_FieldId) -> u32;
pub fn Schema_GetSint32Count(object: *const Schema_Object, field_id: Schema_FieldId) -> u32;
pub fn Schema_GetSint64Count(object: *const Schema_Object, field_id: Schema_FieldId) -> u32;
pub fn Schema_GetFixed32Count(object: *const Schema_Object, field_id: Schema_FieldId) -> u32;
pub fn Schema_GetFixed64Count(object: *const Schema_Object, field_id: Schema_FieldId) -> u32;
pub fn Schema_GetSfixed32Count(object: *const Schema_Object, field_id: Schema_FieldId) -> u32;
pub fn Schema_GetSfixed64Count(object: *const Schema_Object, field_id: Schema_FieldId) -> u32;
pub fn Schema_GetEntityIdCount(object: *const Schema_Object, field_id: Schema_FieldId) -> u32;
pub fn Schema_GetEnumCount(object: *const Schema_Object, field_id: Schema_FieldId) -> u32;
pub fn Schema_GetBytesCount(object: *const Schema_Object, field_id: Schema_FieldId) -> u32;
pub fn Schema_GetObjectCount(object: *const Schema_Object, field_id: Schema_FieldId) -> u32;
*/

impl_primitive_field!(f32, Schema_GetFloat, Schema_IndexFloat, Schema_GetFloatCount, Schema_AddFloat, Schema_AddFloatList);
impl_primitive_field!(f64, Schema_GetDouble, Schema_IndexDouble, Schema_GetDoubleCount, Schema_AddDouble, Schema_AddDoubleList);
impl_primitive_field!(bool, Schema_GetBool, Schema_IndexBool, Schema_GetBoolCount, Schema_AddBool, Schema_AddBoolList);
impl_primitive_field!(i32, Schema_GetInt32, Schema_IndexInt32, Schema_GetInt32Count, Schema_AddInt32, Schema_AddInt32List);
impl_primitive_field!(i64, Schema_GetInt64, Schema_IndexInt64, Schema_GetInt64Count, Schema_AddInt64, Schema_AddInt64List);
impl_primitive_field!(u32, Schema_GetUint32, Schema_IndexUint32, Schema_GetUint32Count, Schema_AddUint32, Schema_AddUint32List);
impl_primitive_field!(u64, Schema_GetUint64, Schema_IndexUint64, Schema_GetUint64Count, Schema_AddUint64, Schema_AddUint64List);
// PROBLEM: We can't identifier certain schema types with rust types (cause multiple schema types map to the same rust type).
impl_primitive_field!(f64, Schema_GetDouble, Schema_IndexDouble, Schema_GetDoubleCount, Schema_AddDouble, Schema_AddDoubleList);
impl_primitive_field!(f64, Schema_GetDouble, Schema_IndexDouble, Schema_GetDoubleCount, Schema_AddDouble, Schema_AddDoubleList);
impl_primitive_field!(f64, Schema_GetDouble, Schema_IndexDouble, Schema_GetDoubleCount, Schema_AddDouble, Schema_AddDoubleList);
impl_primitive_field!(f64, Schema_GetDouble, Schema_IndexDouble, Schema_GetDoubleCount, Schema_AddDouble, Schema_AddDoubleList);
impl_primitive_field!(f64, Schema_GetDouble, Schema_IndexDouble, Schema_GetDoubleCount, Schema_AddDouble, Schema_AddDoubleList);
impl_primitive_field!(f64, Schema_GetDouble, Schema_IndexDouble, Schema_GetDoubleCount, Schema_AddDouble, Schema_AddDoubleList);
impl_primitive_field!(f64, Schema_GetDouble, Schema_IndexDouble, Schema_GetDoubleCount, Schema_AddDouble, Schema_AddDoubleList);
impl_primitive_field!(f64, Schema_GetDouble, Schema_IndexDouble, Schema_GetDoubleCount, Schema_AddDouble, Schema_AddDoubleList);
impl_primitive_field!(f64, Schema_GetDouble, Schema_IndexDouble, Schema_GetDoubleCount, Schema_AddDouble, Schema_AddDoubleList);
impl_primitive_field!(f64, Schema_GetDouble, Schema_IndexDouble, Schema_GetDoubleCount, Schema_AddDouble, Schema_AddDoubleList);
impl_primitive_field!(f64, Schema_GetDouble, Schema_IndexDouble, Schema_GetDoubleCount, Schema_AddDouble, Schema_AddDoubleList);

// TODO: Generate this for all primitive types with a macro.
// i.e. impl_field_primitives!([f32, Float], [f64, Double])
/*
impl<'a> SchemaField<f32> for SchemaFieldContainer<'a, f32> {
    fn get_or_default(&self) -> f32 {
        unsafe { Schema_GetFloat(self.container.internal, self.field_id) }
    }
    fn index(&self, index: usize) -> f32 {
        unsafe { Schema_IndexFloat(self.container.internal, self.field_id, index as u32) }
    }
    fn count(&self) -> usize {
        unsafe { Schema_GetFloatCount(self.container.internal, self.field_id) as usize }
    }

    fn add(&mut self, value: f32) {
        unsafe { Schema_AddFloat(self.container.internal, self.field_id, value); }
    }
    fn add_list(&mut self, value: &[f32]) {
        unsafe {
            let ptr = value.as_ptr();
            Schema_AddFloatList(self.container.internal, self.field_id, ptr, value.len() as u32);
        }
    }
}

impl<'a> SchemaField<i32> for SchemaFieldContainer<'a, i32> {
    fn get_or_default(&self) -> i32 {
        unsafe { Schema_GetInt32(self.container.internal, self.field_id) }
    }
    fn index(&self, index: usize) -> i32 {
        unsafe { Schema_IndexInt32(self.container.internal, self.field_id, index as u32) }
    }
    fn count(&self) -> usize {
        unsafe { Schema_GetInt32Count(self.container.internal, self.field_id) as usize }
    }

    fn add(&mut self, value: i32) {
        unsafe { Schema_AddInt32(self.container.internal, self.field_id, value); }
    }
    fn add_list(&mut self, value: &[i32]) {
        unsafe {
            let ptr = value.as_ptr();
            Schema_AddInt32List(self.container.internal, self.field_id, ptr, value.len() as u32);
        }
    }
}
*/

impl<'a> SchemaObjectField for SchemaFieldContainer<'a, SchemaObject> {
    fn get_or_add(&self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetObject(self.container.internal, self.field_id) }
        }
    }
    fn index(&self, index: usize) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_IndexObject(self.container.internal, self.field_id, index as u32) }
        }
    }
    fn count(&self) -> usize {
        unsafe { Schema_GetObjectCount(self.container.internal, self.field_id) as usize }
    }

    fn add(&mut self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_AddObject(self.container.internal, self.field_id) }
        }
    }
}