use crate::worker::{component::ComponentId, EntityId};
use spatialos_sdk_sys::worker::*;
use std::collections::BTreeMap;

pub type FieldId = u32;

#[derive(Debug)]
pub struct SchemaComponentUpdate {
    pub component_id: ComponentId,
    pub internal: *mut Schema_ComponentUpdate,
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

    // TODO: Cleared fields.
}

#[derive(Debug)]
pub struct SchemaComponentData {
    pub component_id: ComponentId,
    pub internal: *mut Schema_ComponentData,
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

#[derive(Debug)]
pub struct SchemaCommandRequest {
    pub component_id: ComponentId,
    pub internal: *mut Schema_CommandRequest,
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

#[derive(Debug)]
pub struct SchemaCommandResponse {
    pub component_id: ComponentId,
    pub internal: *mut Schema_CommandResponse,
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

#[derive(Debug)]
pub struct SchemaObject {
    internal: *mut Schema_Object,
}

impl SchemaObject {
    pub fn field<T: SchemaField>(&self, field: FieldId) -> T::RustType {
        T::get_field(self, field)
    }

    pub fn add_field<T: SchemaField>(&mut self, field: FieldId, value: &T::RustType) {
        T::add_field(self, field, value);
    }

    pub fn deserialize<T: SchemaObjectType>(&self) -> T {
        T::from_object(self)
    }
}

// =================================================================================================
// Schema Conversion Traits
// =================================================================================================

pub trait SchemaField: Sized {
    type RustType: Sized;

    fn add_field(object: &mut SchemaObject, field: FieldId, value: &Self::RustType);

    fn get_field(object: &SchemaObject, field: FieldId) -> Self::RustType;
}

pub trait IndexedField: SchemaField {
    fn field_count(object: &SchemaObject, field: FieldId) -> u32;

    fn index_field(object: &SchemaObject, field: FieldId, index: u32) -> Self::RustType;
}

pub trait ArrayField: IndexedField {
    fn get_field_list(object: &SchemaObject, field: FieldId, data: &mut Vec<Self::RustType>);
}

/// A type that can be deserialized from an entire `SchemaObject`.
pub trait SchemaObjectType: Sized {
    fn into_object(&self, object: &mut SchemaObject);
    fn from_object(object: &SchemaObject) -> Self;
}

impl<T: SchemaObjectType> SchemaField for T {
    type RustType = Self;

    fn add_field(object: &mut SchemaObject, field: FieldId, value: &Self::RustType) {
        let field_object = unsafe { Schema_AddObject(object.internal, field) };
        value.into_object(&mut SchemaObject {
            internal: field_object,
        });
    }

    fn get_field(object: &SchemaObject, field: FieldId) -> Self::RustType {
        let field_object = unsafe { Schema_GetObject(object.internal, field) };
        T::from_object(&SchemaObject {
            internal: field_object,
        })
    }
}

impl<T: SchemaObjectType> IndexedField for T {
    fn field_count(object: &SchemaObject, field: FieldId) -> u32 {
        unsafe { Schema_GetObjectCount(object.internal, field) }
    }

    fn index_field(object: &SchemaObject, field: FieldId, index: u32) -> Self::RustType {
        let field_object = unsafe { Schema_IndexObject(object.internal, field, index) };
        T::from_object(&SchemaObject {
            internal: field_object,
        })
    }
}

// =================================================================================================
// Schema Conversion Implementations for Primitive Types
// =================================================================================================

macro_rules! impl_primitive_field {
    (
        $rust_type:ty,
        $schema_type:ident,
        $schema_get:ident,
        $schema_index:ident,
        $schema_count:ident,
        $schema_add:ident,
        $schema_add_list:ident,
        $schema_get_list:ident,
    ) => {
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $schema_type;

        impl SchemaField for $schema_type {
            type RustType = $rust_type;

            fn add_field(object: &mut SchemaObject, field: FieldId, value: &Self::RustType) {
                unsafe {
                    $schema_add(object.internal, field, *value);
                }
            }

            fn get_field(object: &SchemaObject, field: FieldId) -> Self::RustType {
                unsafe { $schema_get(object.internal, field) }
            }
        }

        impl IndexedField for $schema_type {
            fn field_count(object: &SchemaObject, field: FieldId) -> u32 {
                unsafe { $schema_count(object.internal, field) }
            }

            fn index_field(object: &SchemaObject, field: FieldId, index: u32) -> Self::RustType {
                unsafe { $schema_index(object.internal, field, index) }
            }
        }

        impl ArrayField for $schema_type {
            fn get_field_list(
                object: &SchemaObject,
                field: FieldId,
                data: &mut Vec<Self::RustType>,
            ) {
                let count = Self::field_count(object, field) as usize;

                // Ensure that there is enough capacity for the elements in the schema field.
                if data.capacity() < count {
                    data.reserve(count - data.capacity());
                }

                // Replace the contents of `data` with the list of values in the schema field.
                unsafe {
                    data.set_len(count);
                    $schema_get_list(object.internal, field, data.as_mut_ptr());
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
    Schema_AddFloatList,
    Schema_GetFloatList,
);
impl_primitive_field!(
    f64,
    SchemaDouble,
    Schema_GetDouble,
    Schema_IndexDouble,
    Schema_GetDoubleCount,
    Schema_AddDouble,
    Schema_AddDoubleList,
    Schema_GetDoubleList,
);
impl_primitive_field!(
    i32,
    SchemaInt32,
    Schema_GetInt32,
    Schema_IndexInt32,
    Schema_GetInt32Count,
    Schema_AddInt32,
    Schema_AddInt32List,
    Schema_GetInt32List,
);
impl_primitive_field!(
    i64,
    SchemaInt64,
    Schema_GetInt64,
    Schema_IndexInt64,
    Schema_GetInt64Count,
    Schema_AddInt64,
    Schema_AddInt64List,
    Schema_GetInt64List,
);
impl_primitive_field!(
    u32,
    SchemaUint32,
    Schema_GetUint32,
    Schema_IndexUint32,
    Schema_GetUint32Count,
    Schema_AddUint32,
    Schema_AddUint32List,
    Schema_GetUint32List,
);
impl_primitive_field!(
    u64,
    SchemaUint64,
    Schema_GetUint64,
    Schema_IndexUint64,
    Schema_GetUint64Count,
    Schema_AddUint64,
    Schema_AddUint64List,
    Schema_GetUint64List,
);
impl_primitive_field!(
    i32,
    SchemaSint32,
    Schema_GetSint32,
    Schema_IndexSint32,
    Schema_GetSint32Count,
    Schema_AddSint32,
    Schema_AddSint32List,
    Schema_GetSint32List,
);
impl_primitive_field!(
    i64,
    SchemaSint64,
    Schema_GetSint64,
    Schema_IndexSint64,
    Schema_GetSint64Count,
    Schema_AddSint64,
    Schema_AddSint64List,
    Schema_GetSint64List,
);
impl_primitive_field!(
    u32,
    SchemaFixed32,
    Schema_GetFixed32,
    Schema_IndexFixed32,
    Schema_GetFixed32Count,
    Schema_AddFixed32,
    Schema_AddFixed32List,
    Schema_GetFixed32List,
);
impl_primitive_field!(
    u64,
    SchemaFixed64,
    Schema_GetFixed64,
    Schema_IndexFixed64,
    Schema_GetFixed64Count,
    Schema_AddFixed64,
    Schema_AddFixed64List,
    Schema_GetFixed64List,
);
impl_primitive_field!(
    i32,
    SchemaSfixed32,
    Schema_GetSfixed32,
    Schema_IndexSfixed32,
    Schema_GetSfixed32Count,
    Schema_AddSfixed32,
    Schema_AddSfixed32List,
    Schema_GetSfixed32List,
);
impl_primitive_field!(
    i64,
    SchemaSfixed64,
    Schema_GetSfixed64,
    Schema_IndexSfixed64,
    Schema_GetSfixed64Count,
    Schema_AddSfixed64,
    Schema_AddSfixed64List,
    Schema_GetSfixed64List,
);
impl_primitive_field!(
    u32,
    SchemaEnum,
    Schema_GetEnum,
    Schema_IndexEnum,
    Schema_GetEnumCount,
    Schema_AddEnum,
    Schema_AddEnumList,
    Schema_GetEnumList,
);

impl SchemaField for EntityId {
    type RustType = Self;

    fn add_field(object: &mut SchemaObject, field: FieldId, value: &Self::RustType) {
        unsafe {
            Schema_AddEntityId(object.internal, field, value.id);
        }
    }

    fn get_field(object: &SchemaObject, field: FieldId) -> Self::RustType {
        let id = unsafe { Schema_GetEntityId(object.internal, field) };
        Self { id }
    }
}

impl IndexedField for EntityId {
    fn field_count(object: &SchemaObject, field: FieldId) -> u32 {
        unsafe { Schema_GetEntityIdCount(object.internal, field) }
    }

    fn index_field(object: &SchemaObject, field: FieldId, index: u32) -> Self::RustType {
        let id = unsafe { Schema_IndexEntityId(object.internal, field, index) };
        Self { id }
    }
}

impl ArrayField for EntityId {
    fn get_field_list(object: &SchemaObject, field: FieldId, data: &mut Vec<Self::RustType>) {
        let count = Self::field_count(object, field) as usize;

        // Ensure that there is enough capacity for the elements in the schema field.
        if data.capacity() < count {
            data.reserve(count - data.capacity());
        }

        // Replace the contents of `data` with the list of values in the schema field.
        unsafe {
            data.set_len(count);
            Schema_GetEntityIdList(object.internal, field, data.as_mut_ptr() as *mut _);
        }
    }
}

impl SchemaField for bool {
    type RustType = Self;

    fn add_field(object: &mut SchemaObject, field: FieldId, value: &Self::RustType) {
        unsafe {
            Schema_AddBool(object.internal, field, *value as u8);
        }
    }

    fn get_field(object: &SchemaObject, field: FieldId) -> Self::RustType {
        let raw = unsafe { Schema_GetBool(object.internal, field) };
        raw != 0
    }
}

impl IndexedField for bool {
    fn field_count(object: &SchemaObject, field: FieldId) -> u32 {
        unsafe { Schema_GetBoolCount(object.internal, field) }
    }

    fn index_field(object: &SchemaObject, field: FieldId, index: u32) -> Self::RustType {
        let raw = unsafe { Schema_IndexBool(object.internal, field, index) };
        raw != 0
    }
}

impl<T: IndexedField> SchemaField for Option<T> {
    type RustType = Option<T::RustType>;

    fn add_field(object: &mut SchemaObject, field: FieldId, value: &Self::RustType) {
        if let Some(value) = value {
            T::add_field(object, field, value);
        }
    }

    fn get_field(object: &SchemaObject, field: FieldId) -> Self::RustType {
        match T::field_count(object, field) {
            0 => None,
            _ => Some(T::get_field(object, field)),
        }
    }
}

impl<K, V> SchemaField for BTreeMap<K, V>
where
    K: IndexedField,
    V: IndexedField,
    K::RustType: Ord,
{
    type RustType = BTreeMap<K::RustType, V::RustType>;

    fn add_field(object: &mut SchemaObject, field: FieldId, value: &Self::RustType) {
        unimplemented!();
    }

    fn get_field(object: &SchemaObject, field: FieldId) -> Self::RustType {
        // Get the map's schema object from the specified field on `object`.
        let object = &SchemaObject {
            internal: unsafe { Schema_GetObject(object.internal, field) },
        };

        // Load each of the key-value pairs from the map object.
        let count = K::field_count(object, SCHEMA_MAP_KEY_FIELD_ID);
        let mut result = BTreeMap::new();
        for index in 0..count {
            let key = K::index_field(object, SCHEMA_MAP_KEY_FIELD_ID, index);
            let value = V::index_field(object, SCHEMA_MAP_VALUE_FIELD_ID, index);
            result.insert(key, value);
        }

        result
    }
}

impl<T: IndexedField> SchemaField for Vec<T> {
    type RustType = Vec<T::RustType>;

    fn add_field(object: &mut SchemaObject, field: FieldId, values: &Self::RustType) {
        // TODO: Provide a specialized version for types implementing `SchemaListType`.
        for value in values {
            T::add_field(object, field, value);
        }
    }

    fn get_field(object: &SchemaObject, field: FieldId) -> Self::RustType {
        let count = T::field_count(object, field);

        // TODO: Provide a specialized version for types implementing `SchemaListType`.
        let mut result = Vec::with_capacity(count as usize);
        for index in 0..count {
            result.push(T::index_field(object, field, index));
        }

        result
    }
}

impl SchemaField for String {
    type RustType = Self;

    fn add_field(object: &mut SchemaObject, field: FieldId, value: &Self::RustType) {
        unimplemented!();
    }

    fn get_field(object: &SchemaObject, field: FieldId) -> Self::RustType {
        let bytes = get_bytes(object, field);
        std::str::from_utf8(bytes)
            .expect("Schema string was invalid UTF-8")
            .into()
    }
}

impl IndexedField for String {
    fn field_count(object: &SchemaObject, field: FieldId) -> u32 {
        unsafe { Schema_GetBytesCount(object.internal, field) }
    }

    fn index_field(object: &SchemaObject, field: FieldId, index: u32) -> Self::RustType {
        let bytes = index_bytes(object, field, index);
        std::str::from_utf8(bytes)
            .expect("Schema string was invalid UTF-8")
            .into()
    }
}

impl SchemaField for Vec<u8> {
    type RustType = Self;

    fn add_field(object: &mut SchemaObject, field: FieldId, value: &Self::RustType) {
        unimplemented!();
    }

    fn get_field(object: &SchemaObject, field: FieldId) -> Self::RustType {
        get_bytes(object, field).into()
    }
}

impl IndexedField for Vec<u8> {
    fn field_count(object: &SchemaObject, field: FieldId) -> u32 {
        unsafe { Schema_GetBytesCount(object.internal, field) }
    }

    fn index_field(object: &SchemaObject, field: FieldId, index: u32) -> Self::RustType {
        index_bytes(object, field, index).into()
    }
}

fn get_bytes(object: &SchemaObject, field: FieldId) -> &[u8] {
    unsafe {
        let data = Schema_GetBytes(object.internal, field);
        let len = Schema_GetBytesLength(object.internal, field);
        std::slice::from_raw_parts(data, len as usize)
    }
}

fn index_bytes(object: &SchemaObject, field: FieldId, index: u32) -> &[u8] {
    unsafe {
        let data = Schema_IndexBytes(object.internal, field, index);
        let len = Schema_IndexBytesLength(object.internal, field, index);
        std::slice::from_raw_parts(data, len as usize)
    }
}
