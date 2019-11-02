use crate::worker::{
    schema::{FieldId, SchemaObject, SchemaPrimitiveField},
    EntityId,
};
use spatialos_sdk_sys::worker::*;
use std::{convert::TryInto, u32};

macro_rules! impl_primitive_field {
    (
        $rust_type:ty,
        $schema_type:ident,
        $schema_get:ident,
        $schema_index:ident,
        $schema_count:ident,
        $schema_add:ident,
        $schema_add_list:ident,
    ) => {
        #[derive(Debug)]
        pub struct $schema_type;

        impl SchemaPrimitiveField for $schema_type {
            type RustType = $rust_type;

            fn get_or_default(object: &SchemaObject, field: FieldId) -> Self::RustType {
                unsafe { $schema_get(object.internal, field) }
            }

            fn index(object: &SchemaObject, field: FieldId, index: usize) -> $rust_type {
                unsafe { $schema_index(object.internal, field, index as u32) }
            }

            fn count(object: &SchemaObject, field: FieldId) -> usize {
                unsafe { $schema_count(object.internal, field) as usize }
            }

            fn add(object: &mut SchemaObject, field: FieldId, value: &Self::RustType) {
                unsafe {
                    $schema_add(object.internal, field, *value);
                }
            }

            fn add_list(object: &mut SchemaObject, field: FieldId, value: &[$rust_type]) {
                let ptr = value.as_ptr();
                let len = value
                    .len()
                    .try_into()
                    .expect("Cannot work with a super long array");
                unsafe {
                    $schema_add_list(object.internal, field, ptr, len);
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
);
impl_primitive_field!(
    f64,
    SchemaDouble,
    Schema_GetDouble,
    Schema_IndexDouble,
    Schema_GetDoubleCount,
    Schema_AddDouble,
    Schema_AddDoubleList,
);
impl_primitive_field!(
    i32,
    SchemaInt32,
    Schema_GetInt32,
    Schema_IndexInt32,
    Schema_GetInt32Count,
    Schema_AddInt32,
    Schema_AddInt32List,
);
impl_primitive_field!(
    i64,
    SchemaInt64,
    Schema_GetInt64,
    Schema_IndexInt64,
    Schema_GetInt64Count,
    Schema_AddInt64,
    Schema_AddInt64List,
);
impl_primitive_field!(
    u32,
    SchemaUint32,
    Schema_GetUint32,
    Schema_IndexUint32,
    Schema_GetUint32Count,
    Schema_AddUint32,
    Schema_AddUint32List,
);
impl_primitive_field!(
    u64,
    SchemaUint64,
    Schema_GetUint64,
    Schema_IndexUint64,
    Schema_GetUint64Count,
    Schema_AddUint64,
    Schema_AddUint64List,
);
impl_primitive_field!(
    i32,
    SchemaSint32,
    Schema_GetSint32,
    Schema_IndexSint32,
    Schema_GetSint32Count,
    Schema_AddSint32,
    Schema_AddSint32List,
);
impl_primitive_field!(
    i64,
    SchemaSint64,
    Schema_GetSint64,
    Schema_IndexSint64,
    Schema_GetSint64Count,
    Schema_AddSint64,
    Schema_AddSint64List,
);
impl_primitive_field!(
    u32,
    SchemaFixed32,
    Schema_GetFixed32,
    Schema_IndexFixed32,
    Schema_GetFixed32Count,
    Schema_AddFixed32,
    Schema_AddFixed32List,
);
impl_primitive_field!(
    u64,
    SchemaFixed64,
    Schema_GetFixed64,
    Schema_IndexFixed64,
    Schema_GetFixed64Count,
    Schema_AddFixed64,
    Schema_AddFixed64List,
);
impl_primitive_field!(
    i32,
    SchemaSfixed32,
    Schema_GetSfixed32,
    Schema_IndexSfixed32,
    Schema_GetSfixed32Count,
    Schema_AddSfixed32,
    Schema_AddSfixed32List,
);
impl_primitive_field!(
    i64,
    SchemaSfixed64,
    Schema_GetSfixed64,
    Schema_IndexSfixed64,
    Schema_GetSfixed64Count,
    Schema_AddSfixed64,
    Schema_AddSfixed64List,
);
impl_primitive_field!(
    u32,
    SchemaEnum,
    Schema_GetEnum,
    Schema_IndexEnum,
    Schema_GetEnumCount,
    Schema_AddEnum,
    Schema_AddEnumList,
);

#[derive(Debug)]
pub struct SchemaBool;
#[derive(Debug)]
pub struct SchemaEntityId;
#[derive(Debug)]
pub struct SchemaBytes;
#[derive(Debug)]
pub struct SchemaString;

impl SchemaPrimitiveField for SchemaEntityId {
    type RustType = EntityId;

    fn get_or_default(object: &SchemaObject, field: FieldId) -> EntityId {
        EntityId::new(unsafe { Schema_GetEntityId(self.container.internal, self.field_id) })
    }
    fn index(&self, index: usize) -> EntityId {
        EntityId::new(unsafe {
            Schema_IndexEntityId(self.container.internal, self.field_id, index as u32)
        })
    }
    fn count(&self) -> usize {
        unsafe { Schema_GetEntityIdCount(self.container.internal, self.field_id) as usize }
    }

    fn add(&mut self, value: EntityId) {
        unsafe {
            Schema_AddEntityId(self.container.internal, self.field_id, value.id);
        }
    }
    fn add_list(&mut self, value: &[EntityId]) {
        let converted_list: Vec<i64> = value.iter().map(|v| v.id).collect();
        unsafe {
            let ptr = converted_list.as_ptr();
            Schema_AddEntityIdList(
                self.container.internal,
                self.field_id,
                ptr,
                value.len() as u32,
            );
        }
    }
}

impl SchemaPrimitiveField for SchemaBool {
    type RustType = bool;

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

impl SchemaPrimitiveField for SchemaString {
    type RustType = String;

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

impl SchemaPrimitiveField for SchemaBytes {
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

impl SchemaPrimitiveField for SchemaObject {
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
