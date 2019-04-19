use crate::worker::component::ComponentId;
use spatialos_sdk_sys::worker::*;
use std::marker::PhantomData;
use std::slice;

pub type FieldId = u32;

#[derive(Debug)]
pub struct SchemaComponentUpdate {
    pub component_id: ComponentId,
    pub internal: *mut Schema_ComponentUpdate,
}

#[derive(Debug)]
pub struct SchemaComponentData {
    pub component_id: ComponentId,
    pub internal: *mut Schema_ComponentData,
}

#[derive(Debug)]
pub struct SchemaCommandRequest {
    pub component_id: ComponentId,
    pub internal: *mut Schema_CommandRequest,
}

#[derive(Debug)]
pub struct SchemaCommandResponse {
    pub component_id: ComponentId,
    pub internal: *mut Schema_CommandResponse,
}

#[derive(Debug)]
pub struct SchemaObject {
    internal: *mut Schema_Object,
}

impl SchemaComponentUpdate {
    pub fn new(component_id: ComponentId) -> SchemaComponentUpdate {
        SchemaComponentUpdate {
            component_id,
            internal: unsafe { Schema_CreateComponentUpdate(component_id) },
        }
    }

    pub fn component_id(&self) -> ComponentId {
        unsafe { Schema_GetComponentUpdateComponentId(self.internal) }
    }

    pub fn fields(&self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetComponentUpdateFields(self.internal) },
        }
    }

    pub fn fields_mut(&mut self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetComponentUpdateFields(self.internal) },
        }
    }

    pub fn events(&self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetComponentUpdateEvents(self.internal) },
        }
    }

    pub fn events_mut(&mut self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetComponentUpdateEvents(self.internal) },
        }
    }

    pub fn cleared_fields(&self) -> Vec<FieldId> {
        let count = unsafe { Schema_GetComponentUpdateClearedFieldCount(self.internal) };
        let mut cleared_fields = Vec::with_capacity(count as usize);

        unsafe {
            Schema_GetComponentUpdateClearedFieldList(self.internal, cleared_fields.as_mut_ptr())
        }

        cleared_fields
    }

    pub fn clear_field(&mut self, id: FieldId) {
        unsafe { Schema_AddComponentUpdateClearedField(self.internal, id) };
    }
}

impl SchemaComponentData {
    pub fn new(component_id: ComponentId) -> SchemaComponentData {
        SchemaComponentData {
            component_id,
            internal: unsafe { Schema_CreateComponentData(component_id) },
        }
    }

    pub fn component_id(&self) -> ComponentId {
        unsafe { Schema_GetComponentDataComponentId(self.internal) }
    }

    pub fn fields(&self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetComponentDataFields(self.internal) },
        }
    }

    pub fn fields_mut(&mut self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetComponentDataFields(self.internal) },
        }
    }
}

impl SchemaCommandRequest {
    pub fn new(component_id: ComponentId, command_index: FieldId) -> SchemaCommandRequest {
        SchemaCommandRequest {
            component_id,
            internal: unsafe { Schema_CreateCommandRequest(component_id, command_index) },
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
            internal: unsafe { Schema_GetCommandRequestObject(self.internal) },
        }
    }

    pub fn object_mut(&mut self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetCommandRequestObject(self.internal) },
        }
    }
}

impl SchemaCommandResponse {
    pub fn new(component_id: u32, command_index: u32) -> SchemaCommandResponse {
        SchemaCommandResponse {
            component_id,
            internal: unsafe { Schema_CreateCommandResponse(component_id, command_index) },
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
            internal: unsafe { Schema_GetCommandResponseObject(self.internal) },
        }
    }

    pub fn object_mut(&mut self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetCommandResponseObject(self.internal) },
        }
    }
}

// A schema field. T is a schema type tag.
#[derive(Debug)]
pub struct SchemaFieldContainer<'a, T> {
    field_id: FieldId,
    container: &'a SchemaObject,
    _phantom: PhantomData<T>,
}

// Tags to represent schema types.
#[derive(Debug)]
pub struct SchemaFloat;
#[derive(Debug)]
pub struct SchemaDouble;
#[derive(Debug)]
pub struct SchemaBool;
#[derive(Debug)]
pub struct SchemaInt32;
#[derive(Debug)]
pub struct SchemaInt64;
#[derive(Debug)]
pub struct SchemaUint32;
#[derive(Debug)]
pub struct SchemaUint64;
#[derive(Debug)]
pub struct SchemaSint32;
#[derive(Debug)]
pub struct SchemaSint64;
#[derive(Debug)]
pub struct SchemaFixed32;
#[derive(Debug)]
pub struct SchemaFixed64;
#[derive(Debug)]
pub struct SchemaSfixed32;
#[derive(Debug)]
pub struct SchemaSfixed64;
#[derive(Debug)]
pub struct SchemaEntityId;
#[derive(Debug)]
pub struct SchemaEnum;
#[derive(Debug)]
pub struct SchemaBytes;
#[derive(Debug)]
pub struct SchemaString;

// A primitive schema field.
pub trait SchemaPrimitiveField<T> {
    fn get(&self) -> Option<T> {
        if self.count() == 0 {
            None
        } else {
            Some(self.get_or_default())
        }
    }

    fn get_or_default(&self) -> T;
    fn index(&self, index: usize) -> T;
    fn count(&self) -> usize;

    fn add(&mut self, value: T);
    fn add_list(&mut self, value: &[T]);
}

// A bytes schema field.
pub trait SchemaBytesField {
    fn get(&self) -> Option<Vec<u8>> {
        if self.count() == 0 {
            None
        } else {
            Some(self.get_or_default())
        }
    }

    fn get_or_default(&self) -> Vec<u8>;
    fn index(&self, index: usize) -> Vec<u8>;
    fn count(&self) -> usize;
    fn add(&mut self, value: &[u8]);
}

// A string schema field.
#[allow(clippy::ptr_arg)]
pub trait SchemaStringField {
    fn get(&self) -> Option<String> {
        if self.count() == 0 {
            None
        } else {
            Some(self.get_or_default())
        }
    }

    fn get_or_default(&self) -> String;
    fn index(&self, index: usize) -> String;
    fn count(&self) -> usize;

    fn add(&mut self, value: &String);
    fn add_list(&mut self, value: &[String]);
}

// An object schema field.
pub trait SchemaObjectField {
    fn get(&self) -> Option<SchemaObject> {
        if self.count() == 0 {
            None
        } else {
            Some(self.get_or_default())
        }
    }

    fn get_or_default(&self) -> SchemaObject;
    fn index(&self, index: usize) -> SchemaObject;
    fn count(&self) -> usize;

    fn add(&mut self) -> SchemaObject;
}

impl SchemaObject {
    pub fn field<T>(&self, field_id: ComponentId) -> SchemaFieldContainer<T> {
        SchemaFieldContainer {
            field_id,
            container: self,
            _phantom: PhantomData,
        }
    }
}

macro_rules! impl_primitive_field {
    ($rust_type:ty, $schema_type:ty, $schema_get:ident, $schema_index:ident, $schema_count:ident, $schema_add:ident, $schema_add_list:ident) => {
        impl<'a> SchemaPrimitiveField<$rust_type> for SchemaFieldContainer<'a, $schema_type> {
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
                unsafe {
                    $schema_add(self.container.internal, self.field_id, value);
                }
            }
            fn add_list(&mut self, value: &[$rust_type]) {
                unsafe {
                    let ptr = value.as_ptr();
                    $schema_add_list(
                        self.container.internal,
                        self.field_id,
                        ptr,
                        value.len() as u32,
                    );
                }
            }
        }
    };
}

impl_primitive_field!(
    f32,
    SchemaFloat,
    Schema_GetFloat,
    Schema_IndexFloat,
    Schema_GetFloatCount,
    Schema_AddFloat,
    Schema_AddFloatList
);
impl_primitive_field!(
    f64,
    SchemaDouble,
    Schema_GetDouble,
    Schema_IndexDouble,
    Schema_GetDoubleCount,
    Schema_AddDouble,
    Schema_AddDoubleList
);
impl_primitive_field!(
    i32,
    SchemaInt32,
    Schema_GetInt32,
    Schema_IndexInt32,
    Schema_GetInt32Count,
    Schema_AddInt32,
    Schema_AddInt32List
);
impl_primitive_field!(
    i64,
    SchemaInt64,
    Schema_GetInt64,
    Schema_IndexInt64,
    Schema_GetInt64Count,
    Schema_AddInt64,
    Schema_AddInt64List
);
impl_primitive_field!(
    u32,
    SchemaUint32,
    Schema_GetUint32,
    Schema_IndexUint32,
    Schema_GetUint32Count,
    Schema_AddUint32,
    Schema_AddUint32List
);
impl_primitive_field!(
    u64,
    SchemaUint64,
    Schema_GetUint64,
    Schema_IndexUint64,
    Schema_GetUint64Count,
    Schema_AddUint64,
    Schema_AddUint64List
);
impl_primitive_field!(
    i32,
    SchemaSint32,
    Schema_GetSint32,
    Schema_IndexSint32,
    Schema_GetSint32Count,
    Schema_AddSint32,
    Schema_AddSint32List
);
impl_primitive_field!(
    i64,
    SchemaSint64,
    Schema_GetSint64,
    Schema_IndexSint64,
    Schema_GetSint64Count,
    Schema_AddSint64,
    Schema_AddSint64List
);
impl_primitive_field!(
    u32,
    SchemaFixed32,
    Schema_GetFixed32,
    Schema_IndexFixed32,
    Schema_GetFixed32Count,
    Schema_AddFixed32,
    Schema_AddFixed32List
);
impl_primitive_field!(
    u64,
    SchemaFixed64,
    Schema_GetFixed64,
    Schema_IndexFixed64,
    Schema_GetFixed64Count,
    Schema_AddFixed64,
    Schema_AddFixed64List
);
impl_primitive_field!(
    i32,
    SchemaSfixed32,
    Schema_GetSfixed32,
    Schema_IndexSfixed32,
    Schema_GetSfixed32Count,
    Schema_AddSfixed32,
    Schema_AddSfixed32List
);
impl_primitive_field!(
    i64,
    SchemaSfixed64,
    Schema_GetSfixed64,
    Schema_IndexSfixed64,
    Schema_GetSfixed64Count,
    Schema_AddSfixed64,
    Schema_AddSfixed64List
);
impl_primitive_field!(
    i64,
    SchemaEntityId,
    Schema_GetEntityId,
    Schema_IndexEntityId,
    Schema_GetEntityIdCount,
    Schema_AddEntityId,
    Schema_AddEntityIdList
);
impl_primitive_field!(
    u32,
    SchemaEnum,
    Schema_GetEnum,
    Schema_IndexEnum,
    Schema_GetEnumCount,
    Schema_AddEnum,
    Schema_AddEnumList
);

impl<'a> SchemaPrimitiveField<bool> for SchemaFieldContainer<'a, SchemaBool> {
    fn get_or_default(&self) -> bool {
        unsafe { Schema_GetBool(self.container.internal, self.field_id) != 0 }
    }
    fn index(&self, index: usize) -> bool {
        unsafe { Schema_IndexBool(self.container.internal, self.field_id, index as u32) != 0 }
    }
    fn count(&self) -> usize {
        unsafe { Schema_GetBoolCount(self.container.internal, self.field_id) as usize }
    }

    fn add(&mut self, value: bool) {
        unsafe {
            Schema_AddBool(self.container.internal, self.field_id, value as u8);
        }
    }
    fn add_list(&mut self, value: &[bool]) {
        let converted_list: Vec<u8> = value.iter().map(|v| if *v { 1u8 } else { 0u8 }).collect();
        unsafe {
            let ptr = converted_list.as_ptr();
            Schema_AddBoolList(
                self.container.internal,
                self.field_id,
                ptr,
                value.len() as u32,
            );
        }
    }
}

impl<'a> SchemaStringField for SchemaFieldContainer<'a, SchemaString> {
    fn get_or_default(&self) -> String {
        let slice = unsafe {
            let bytes_ptr = Schema_GetBytes(self.container.internal, self.field_id);
            let bytes_len = Schema_GetBytesLength(self.container.internal, self.field_id);
            slice::from_raw_parts(bytes_ptr, bytes_len as usize)
        };
        String::from_utf8_lossy(slice).to_string()
    }
    fn index(&self, index: usize) -> String {
        let slice = unsafe {
            let bytes_ptr = Schema_IndexBytes(self.container.internal, self.field_id, index as u32);
            let bytes_len =
                Schema_IndexBytesLength(self.container.internal, self.field_id, index as u32);
            slice::from_raw_parts(bytes_ptr, bytes_len as usize)
        };
        String::from_utf8_lossy(slice).to_string()
    }
    fn count(&self) -> usize {
        unsafe { Schema_GetBytesCount(self.container.internal, self.field_id) as usize }
    }

    fn add(&mut self, value: &String) {
        let utf8_bytes = value.as_bytes();
        unsafe {
            Schema_AddBytes(
                self.container.internal,
                self.field_id,
                utf8_bytes.as_ptr(),
                utf8_bytes.len() as u32,
            );
        }
    }
    fn add_list(&mut self, value: &[String]) {
        for str in value.iter() {
            self.add(str);
        }
    }
}

impl<'a> SchemaBytesField for SchemaFieldContainer<'a, SchemaBytes> {
    fn get_or_default(&self) -> Vec<u8> {
        let slice = unsafe {
            let bytes_ptr = Schema_GetBytes(self.container.internal, self.field_id);
            let bytes_len = Schema_GetBytesLength(self.container.internal, self.field_id);
            slice::from_raw_parts(bytes_ptr, bytes_len as usize)
        };
        slice.to_vec()
    }
    fn index(&self, index: usize) -> Vec<u8> {
        let slice = unsafe {
            let bytes_ptr = Schema_IndexBytes(self.container.internal, self.field_id, index as u32);
            let bytes_len =
                Schema_IndexBytesLength(self.container.internal, self.field_id, index as u32);
            slice::from_raw_parts(bytes_ptr, bytes_len as usize)
        };
        slice.to_vec()
    }
    fn count(&self) -> usize {
        unsafe { Schema_GetBytesCount(self.container.internal, self.field_id) as usize }
    }

    fn add(&mut self, value: &[u8]) {
        unsafe {
            Schema_AddBytes(
                self.container.internal,
                self.field_id,
                value.as_ptr(),
                value.len() as u32,
            );
        }
    }
}

impl<'a> SchemaObjectField for SchemaFieldContainer<'a, SchemaObject> {
    fn get_or_default(&self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetObject(self.container.internal, self.field_id) },
        }
    }
    fn index(&self, index: usize) -> SchemaObject {
        SchemaObject {
            internal: unsafe {
                Schema_IndexObject(self.container.internal, self.field_id, index as u32)
            },
        }
    }
    fn count(&self) -> usize {
        unsafe { Schema_GetObjectCount(self.container.internal, self.field_id) as usize }
    }

    fn add(&mut self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_AddObject(self.container.internal, self.field_id) },
        }
    }
}
