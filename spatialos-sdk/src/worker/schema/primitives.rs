use crate::worker::{
    schema::{DataPointer, Error, Field, FieldId, Result, SchemaComponentUpdate, SchemaObject},
    EntityId,
};
use spatialos_sdk_sys::worker::*;
use std::{convert::TryInto, mem, slice, u32};

// SAFETY: In addition to the usual caveats about FFI, the code generated by this
// macro makes use of `mem::transmute` in order to convert between Rust types and
// corresponding C API types. Specifically, `bool` and `EntityId` are the two cases
// where the Rust type doesn't line up exactly with the C type in the API: C
// represents booleans as `u8`, and entity IDs are represented as `i64`.
//
// For `EntityId` compatibility is pretty easy to verify: `EntityId` is a struct
// with a single `i64` field, and it is marked with `repr(transparent)` to ensure
// that its layout in memory is identical to `i64`.
//
// For `bool`, we have the extra requirement that the values produced by the
// SpatialOS SDK are ONLY ever 0 or 1. Producing a `bool` with any other bit pattern
// is undefined behavior. Fortunately, the SDK guarantees that it will only ever
// produce a `u8` with the value 0 or 1, even if someone erroneously calls
// `Schema_AddBool` with another value.
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

        impl Field for $schema_type {
            type RustType = $rust_type;

            fn get(object: &SchemaObject, field: FieldId) -> Result<$rust_type> {
                if Self::count(object, field) > 0 {
                    Ok(unsafe { mem::transmute($schema_get(object.as_ptr(), field)) })
                } else {
                    Err(Error::missing_field::<Self>())
                }
            }

            fn index(object: &SchemaObject, field: FieldId, index: usize) -> Result<$rust_type> {
                let count = Self::count(object, field);
                if count > index {
                    Ok(unsafe {
                        mem::transmute($schema_index(object.as_ptr(), field, index as u32))
                    })
                } else {
                    Err(Error::index_out_of_bounds::<Self>(index, count))
                }
            }

            fn count(object: &SchemaObject, field: FieldId) -> usize {
                unsafe { $schema_count(object.as_ptr(), field) as usize }
            }

            fn add(object: &mut SchemaObject, field: FieldId, value: &$rust_type) {
                unsafe {
                    $schema_add(object.as_ptr_mut(), field, mem::transmute(*value));
                }
            }

            fn add_list(object: &mut SchemaObject, field: FieldId, value: &[$rust_type]) {
                let ptr = value.as_ptr();
                let len = value
                    .len()
                    .try_into()
                    .expect("Cannot work with a super long array");
                unsafe {
                    $schema_add_list(object.as_ptr_mut(), field, ptr as *const _, len);
                }
            }

            fn has_update(update: &SchemaComponentUpdate, field: FieldId) -> bool {
                Self::count(update.fields(), field) > 0
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
impl_primitive_field!(
    EntityId,
    SchemaEntityId,
    Schema_GetEntityId,
    Schema_IndexEntityId,
    Schema_GetEntityIdCount,
    Schema_AddEntityId,
    Schema_AddEntityIdList,
);
impl_primitive_field!(
    bool,
    SchemaBool,
    Schema_GetBool,
    Schema_IndexBool,
    Schema_GetBoolCount,
    Schema_AddBool,
    Schema_AddBoolList,
);

#[derive(Debug)]
pub struct SchemaBytes;
#[derive(Debug)]
pub struct SchemaString;

impl Field for SchemaString {
    type RustType = String;

    fn get(object: &SchemaObject, field: FieldId) -> Result<String> {
        if Self::count(object, field) > 0 {
            let slice = unsafe {
                let bytes_ptr = Schema_GetBytes(object.as_ptr(), field);
                let bytes_len = Schema_GetBytesLength(object.as_ptr(), field);
                slice::from_raw_parts(bytes_ptr, bytes_len as usize)
            };
            Ok(String::from_utf8_lossy(slice).to_string())
        } else {
            Err(Error::missing_field::<Self>())
        }
    }

    fn index(object: &SchemaObject, field: FieldId, index: usize) -> Result<String> {
        let count = Self::count(object, field);
        if count > index {
            let slice = unsafe {
                let bytes_ptr = Schema_IndexBytes(object.as_ptr(), field, index as u32);
                let bytes_len = Schema_IndexBytesLength(object.as_ptr(), field, index as u32);
                slice::from_raw_parts(bytes_ptr, bytes_len as usize)
            };
            Ok(String::from_utf8_lossy(slice).to_string())
        } else {
            Err(Error::index_out_of_bounds::<Self>(index, count))
        }
    }

    fn count(object: &SchemaObject, field: FieldId) -> usize {
        unsafe { Schema_GetBytesCount(object.as_ptr(), field) as usize }
    }

    fn add(object: &mut SchemaObject, field: FieldId, value: &String) {
        let utf8_bytes = value.as_bytes();
        unsafe {
            Schema_AddBytes(
                object.as_ptr_mut(),
                field,
                utf8_bytes.as_ptr(),
                utf8_bytes.len() as u32,
            );
        }
    }

    fn add_list(object: &mut SchemaObject, field: FieldId, value: &[String]) {
        for value in value {
            Self::add(object, field, value);
        }
    }

    fn has_update(update: &SchemaComponentUpdate, field: FieldId) -> bool {
        Self::count(update.fields(), field) > 0
    }
}

impl Field for SchemaBytes {
    type RustType = Vec<u8>;

    fn get(object: &SchemaObject, field: FieldId) -> Result<Vec<u8>> {
        if Self::count(object, field) > 0 {
            let slice = unsafe {
                let bytes_ptr = Schema_GetBytes(object.as_ptr(), field);
                let bytes_len = Schema_GetBytesLength(object.as_ptr(), field);
                slice::from_raw_parts(bytes_ptr, bytes_len as usize)
            };
            Ok(slice.to_vec())
        } else {
            Err(Error::missing_field::<Self>())
        }
    }

    fn index(object: &SchemaObject, field: FieldId, index: usize) -> Result<Vec<u8>> {
        let count = Self::count(object, field);
        if count > index {
            let slice = unsafe {
                let bytes_ptr = Schema_IndexBytes(object.as_ptr(), field, index as u32);
                let bytes_len = Schema_IndexBytesLength(object.as_ptr(), field, index as u32);
                slice::from_raw_parts(bytes_ptr, bytes_len as usize)
            };
            Ok(slice.to_vec())
        } else {
            Err(Error::index_out_of_bounds::<Self>(index, count))
        }
    }

    fn count(object: &SchemaObject, field: FieldId) -> usize {
        unsafe { Schema_GetBytesCount(object.as_ptr(), field) as usize }
    }

    fn add(object: &mut SchemaObject, field: FieldId, value: &Vec<u8>) {
        unsafe {
            Schema_AddBytes(
                object.as_ptr_mut(),
                field,
                value.as_ptr(),
                value.len() as u32,
            );
        }
    }

    fn add_list(object: &mut SchemaObject, field: FieldId, value: &[Vec<u8>]) {
        for value in value {
            Self::add(object, field, value);
        }
    }

    fn has_update(update: &SchemaComponentUpdate, field: FieldId) -> bool {
        Self::count(update.fields(), field) > 0
    }
}
