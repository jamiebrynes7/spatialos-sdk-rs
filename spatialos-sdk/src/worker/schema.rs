//! Rust wrappers around the various schema data types defined by the C SDK.
//!
//! TODO: Document the conventions around how we handle correctness and safety
//! around schema data:
//!
//! * By convention there are two lifetimes for each type: `'owner` and `'data`. The
//!   immutable view only uses `'owner`, the owned version uses `'data`.
//! * There are two Rust types for each C type: An immutable, borrowed version; And
//!   a mutable, owned version.

use crate::worker::{internal::Sealed, EntityId};
use spatialos_sdk_sys::worker::*;
use std::{collections::BTreeMap, mem, slice};

mod component_data;
mod object;

pub use self::{component_data::*, object::*};

pub type FieldId = u32;

// =================================================================================================
// Schema Conversion Traits
// =================================================================================================

pub trait SchemaField: Sized + Sealed {
    type RustType: Sized;

    fn add_field(object: &mut ObjectMut<'_>, field: FieldId, value: &Self::RustType);

    fn get_field(object: ObjectRef, field: FieldId) -> Self::RustType;
}

pub trait IndexedField: SchemaField {
    fn field_count(object: ObjectRef, field: FieldId) -> u32;

    fn index_field(object: ObjectRef, field: FieldId, index: u32) -> Self::RustType;
}

pub trait ArrayField: IndexedField {
    fn add_field_list(object: &mut ObjectMut<'_>, field: FieldId, data: &[Self::RustType]);

    fn get_field_list(object: ObjectRef, field: FieldId, data: &mut Vec<Self::RustType>);
}

/// A type that can be deserialized from an entire `SchemaObject`.
pub trait SchemaObjectType: Sized {
    fn into_object<'owner>(&self, object: &mut ObjectMut<'owner>);
    fn from_object(object: ObjectRef) -> Self;
}

impl<T: SchemaObjectType> Sealed for T {}

impl<T: SchemaObjectType> SchemaField for T {
    type RustType = Self;

    fn add_field(object: &mut ObjectMut<'_>, field: FieldId, value: &Self::RustType) {
        let mut field_object =
            unsafe { ObjectMut::from_raw(&mut *Schema_AddObject(object.as_mut(), field)) };
        value.into_object(&mut field_object);
    }

    fn get_field(object: ObjectRef<'_>, field: FieldId) -> Self::RustType {
        let field_object =
            unsafe { ObjectRef::from_raw(&*Schema_GetObject(object.as_ptr() as *mut _, field)) };
        T::from_object(field_object)
    }
}

impl<T: SchemaObjectType> IndexedField for T {
    fn field_count(object: ObjectRef<'_>, field: FieldId) -> u32 {
        unsafe { Schema_GetObjectCount(object.as_ptr(), field) }
    }

    fn index_field(object: ObjectRef<'_>, field: FieldId, index: u32) -> Self::RustType {
        let field_object = unsafe {
            ObjectRef::from_raw(&*Schema_IndexObject(
                object.as_ptr() as *const _ as *mut _,
                field,
                index,
            ))
        };
        T::from_object(field_object)
    }
}

// =================================================================================================
// Schema Conversion Implementations for Primitive Types
// =================================================================================================

macro_rules! generate_schema_primitive {
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

        impl_primitive_field!(
            $rust_type,
            $schema_type,
            $schema_get,
            $schema_index,
            $schema_count,
            $schema_add,
            $schema_add_list,
            $schema_get_list,
        );
    };
}

// NOTE: In a couple of cases (`bool` and `EntityId`), the type used by the C API
// isn't the same as the Rust types. We'd prefer to use `Into` to do the
// conversion, however `bool` doesn't have any type conversions defined (since it's
// not generally meaningful to convert a numeric type to a `bool` directly). In all
// cases, though, the Rust type is ABI-compatible with the C type:
//
// * The C API treats bools as `u8` represented as 0 or 1, which meets the criteria
//   for Rust's `bool` type.
// * `EntityId` is a wrapper around `i64` and is marked `repr(transparent)`, and so
//   is guaranteed to be ABI-compatibel with a bare `i64`.
//
// As such, it's safe to transmute between the C type and the Rust type in these
// cases.
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
        impl Sealed for $schema_type {}

        impl SchemaField for $schema_type {
            type RustType = $rust_type;

            fn add_field<'owner>(
                object: &mut ObjectMut<'owner>,
                field: FieldId,
                value: &Self::RustType,
            ) {
                let value = (*value).into();
                unsafe {
                    $schema_add(object.as_ptr(), field, value);
                }
            }

            #[allow(clippy::useless_transmute, clippy::transmute_int_to_bool)]
            fn get_field(object: ObjectRef, field: FieldId) -> Self::RustType {
                unsafe { mem::transmute($schema_get(object.as_ptr(), field)) }
            }
        }

        impl IndexedField for $schema_type {
            fn field_count(object: ObjectRef<'_>, field: FieldId) -> u32 {
                unsafe { $schema_count(object.as_ptr(), field) }
            }

            #[allow(clippy::useless_transmute, clippy::transmute_int_to_bool)]
            fn index_field(object: ObjectRef<'_>, field: FieldId, index: u32) -> Self::RustType {
                unsafe { mem::transmute($schema_index(object.as_ptr(), field, index)) }
            }
        }

        impl ArrayField for $schema_type {
            fn add_field_list(object: &mut ObjectMut<'_>, field: FieldId, data: &[Self::RustType]) {
                // Determine how large the buffer needs to be.
                //
                // NOTE: We allocate extra padding when allocating the buffer in order to have room
                // to adjust the alignment of the buffer to match the alignment of `Self::RustType`
                // and still have enough room in the buffer for the right number of elements.
                let byte_len = {
                    let data_len = data.len() * mem::size_of::<Self::RustType>();
                    let padding = mem::align_of::<Self::RustType>() - 1;
                    data_len + padding
                };

                // Allocate a buffer that is owned by `object`.
                let buffer = unsafe {
                    let data_ptr = Schema_AllocateBuffer(object.as_mut(), byte_len as _);
                    let buffer = slice::from_raw_parts_mut(data_ptr, byte_len);

                    // Convert the byte buffer into a correctly-alligned slice of the data type.
                    let (_prefix, buffer, _suffix) = buffer.align_to_mut::<Self::RustType>();
                    buffer
                };

                // Populate the buffer.
                buffer.copy_from_slice(data);

                unsafe {
                    $schema_add_list(
                        object.as_ptr(),
                        field,
                        buffer.as_ptr() as *const _,
                        buffer.len() as u32,
                    );
                }
            }

            fn get_field_list(
                object: ObjectRef<'_>,
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
                    $schema_get_list(object.as_ptr(), field, data.as_mut_ptr() as *mut _);
                }
            }
        }
    };
}

generate_schema_primitive!(
    f32,
    SchemaFloat,
    Schema_GetFloat,
    Schema_IndexFloat,
    Schema_GetFloatCount,
    Schema_AddFloat,
    Schema_AddFloatList,
    Schema_GetFloatList,
);
generate_schema_primitive!(
    f64,
    SchemaDouble,
    Schema_GetDouble,
    Schema_IndexDouble,
    Schema_GetDoubleCount,
    Schema_AddDouble,
    Schema_AddDoubleList,
    Schema_GetDoubleList,
);
generate_schema_primitive!(
    i32,
    SchemaInt32,
    Schema_GetInt32,
    Schema_IndexInt32,
    Schema_GetInt32Count,
    Schema_AddInt32,
    Schema_AddInt32List,
    Schema_GetInt32List,
);
generate_schema_primitive!(
    i64,
    SchemaInt64,
    Schema_GetInt64,
    Schema_IndexInt64,
    Schema_GetInt64Count,
    Schema_AddInt64,
    Schema_AddInt64List,
    Schema_GetInt64List,
);
generate_schema_primitive!(
    u32,
    SchemaUint32,
    Schema_GetUint32,
    Schema_IndexUint32,
    Schema_GetUint32Count,
    Schema_AddUint32,
    Schema_AddUint32List,
    Schema_GetUint32List,
);
generate_schema_primitive!(
    u64,
    SchemaUint64,
    Schema_GetUint64,
    Schema_IndexUint64,
    Schema_GetUint64Count,
    Schema_AddUint64,
    Schema_AddUint64List,
    Schema_GetUint64List,
);
generate_schema_primitive!(
    i32,
    SchemaSint32,
    Schema_GetSint32,
    Schema_IndexSint32,
    Schema_GetSint32Count,
    Schema_AddSint32,
    Schema_AddSint32List,
    Schema_GetSint32List,
);
generate_schema_primitive!(
    i64,
    SchemaSint64,
    Schema_GetSint64,
    Schema_IndexSint64,
    Schema_GetSint64Count,
    Schema_AddSint64,
    Schema_AddSint64List,
    Schema_GetSint64List,
);
generate_schema_primitive!(
    u32,
    SchemaFixed32,
    Schema_GetFixed32,
    Schema_IndexFixed32,
    Schema_GetFixed32Count,
    Schema_AddFixed32,
    Schema_AddFixed32List,
    Schema_GetFixed32List,
);
generate_schema_primitive!(
    u64,
    SchemaFixed64,
    Schema_GetFixed64,
    Schema_IndexFixed64,
    Schema_GetFixed64Count,
    Schema_AddFixed64,
    Schema_AddFixed64List,
    Schema_GetFixed64List,
);
generate_schema_primitive!(
    i32,
    SchemaSfixed32,
    Schema_GetSfixed32,
    Schema_IndexSfixed32,
    Schema_GetSfixed32Count,
    Schema_AddSfixed32,
    Schema_AddSfixed32List,
    Schema_GetSfixed32List,
);
generate_schema_primitive!(
    i64,
    SchemaSfixed64,
    Schema_GetSfixed64,
    Schema_IndexSfixed64,
    Schema_GetSfixed64Count,
    Schema_AddSfixed64,
    Schema_AddSfixed64List,
    Schema_GetSfixed64List,
);
generate_schema_primitive!(
    u32,
    SchemaEnum,
    Schema_GetEnum,
    Schema_IndexEnum,
    Schema_GetEnumCount,
    Schema_AddEnum,
    Schema_AddEnumList,
    Schema_GetEnumList,
);

// NOTE: It's safe to treat the `EntityId` array as an `i64` array because
// `EntityId` is marked `repr(transparent)`, and so is identical to an `i64` for
// the purpose of FFI.
impl_primitive_field!(
    EntityId,
    EntityId,
    Schema_GetEntityId,
    Schema_IndexEntityId,
    Schema_GetEntityIdCount,
    Schema_AddEntityId,
    Schema_AddEntityIdList,
    Schema_GetEntityIdList,
);

// NOTE: Bools in a schema object are guaranteed to be represented as either 0 or 1,
// so it's safe to directly convert the returned values to Rust a `bool` without
// intermediate conversions.
impl_primitive_field!(
    bool,
    bool,
    Schema_GetBool,
    Schema_IndexBool,
    Schema_GetBoolCount,
    Schema_AddBool,
    Schema_AddBoolList,
    Schema_GetBoolList,
);

impl<T: IndexedField> Sealed for Option<T> {}

impl<T: IndexedField> SchemaField for Option<T> {
    type RustType = Option<T::RustType>;

    fn add_field(object: &mut ObjectMut<'_>, field: FieldId, value: &Self::RustType) {
        if let Some(value) = value {
            T::add_field(object, field, value);
        }
    }

    fn get_field(object: ObjectRef<'_>, field: FieldId) -> Self::RustType {
        match T::field_count(object, field) {
            0 => None,
            _ => Some(T::get_field(object, field)),
        }
    }
}

impl<K, V> Sealed for BTreeMap<K, V>
where
    K: IndexedField,
    V: IndexedField,
    K::RustType: Ord,
{
}

impl<K, V> SchemaField for BTreeMap<K, V>
where
    K: IndexedField,
    V: IndexedField,
    K::RustType: Ord,
{
    type RustType = BTreeMap<K::RustType, V::RustType>;

    fn add_field<'owner>(object: &mut ObjectMut<'owner>, field: FieldId, map: &Self::RustType) {
        // Create a key-value pair object for each entry in the map.
        for (key, value) in map {
            let mut pair =
                unsafe { ObjectMut::from_raw(&mut *Schema_AddObject(object.as_ptr(), field)) };
            pair.add_field::<K>(SCHEMA_MAP_KEY_FIELD_ID, key);
            pair.add_field::<V>(SCHEMA_MAP_VALUE_FIELD_ID, value);
        }
    }

    fn get_field(object: ObjectRef, field: FieldId) -> Self::RustType {
        // Load each of the key-value pairs from the map object.
        let count = K::field_count(object, field);
        let mut result = BTreeMap::new();
        for index in 0..count {
            let pair = unsafe {
                ObjectRef::from_raw(&*Schema_IndexObject(
                    object.as_ptr() as *const _ as *mut _,
                    field,
                    index,
                ))
            };
            let key = K::get_field(pair, SCHEMA_MAP_KEY_FIELD_ID);
            let value = V::get_field(pair, SCHEMA_MAP_VALUE_FIELD_ID);
            result.insert(key, value);
        }

        result
    }
}

impl<T: IndexedField> Sealed for Vec<T> {}

impl<T: IndexedField> SchemaField for Vec<T> {
    type RustType = Vec<T::RustType>;

    fn add_field<'owner>(object: &mut ObjectMut<'owner>, field: FieldId, values: &Self::RustType) {
        for value in values {
            T::add_field(object, field, value);
        }
    }

    fn get_field(object: ObjectRef, field: FieldId) -> Self::RustType {
        let count = T::field_count(object, field);

        let mut result = Vec::with_capacity(count as usize);
        for index in 0..count {
            result.push(T::index_field(object, field, index));
        }

        result
    }
}

impl Sealed for String {}

impl SchemaField for String {
    type RustType = Self;

    fn add_field<'owner>(object: &mut ObjectMut<'owner>, field: FieldId, value: &Self::RustType) {
        add_bytes(object, field, value.as_bytes());
    }

    fn get_field(object: ObjectRef, field: FieldId) -> Self::RustType {
        let bytes = get_bytes(object, field);
        std::str::from_utf8(bytes)
            .expect("Schema string was invalid UTF-8")
            .into()
    }
}

impl IndexedField for String {
    fn field_count(object: ObjectRef, field: FieldId) -> u32 {
        unsafe { Schema_GetBytesCount(object.as_ptr(), field) }
    }

    fn index_field(object: ObjectRef, field: FieldId, index: u32) -> Self::RustType {
        let bytes = index_bytes(object, field, index);
        std::str::from_utf8(bytes)
            .expect("Schema string was invalid UTF-8")
            .into()
    }
}

impl Sealed for Vec<u8> {}

impl SchemaField for Vec<u8> {
    type RustType = Self;

    fn add_field<'owner>(object: &mut ObjectMut<'owner>, field: FieldId, value: &Self::RustType) {
        add_bytes(object, field, value);
    }

    fn get_field(object: ObjectRef<'_>, field: FieldId) -> Self::RustType {
        get_bytes(object, field).into()
    }
}

impl IndexedField for Vec<u8> {
    fn field_count(object: ObjectRef<'_>, field: FieldId) -> u32 {
        unsafe { Schema_GetBytesCount(object.as_ptr(), field) }
    }

    fn index_field(object: ObjectRef<'_>, field: FieldId, index: u32) -> Self::RustType {
        index_bytes(object, field, index).into()
    }
}

fn add_bytes<'owner>(object: &mut ObjectMut<'owner>, field: FieldId, bytes: &[u8]) {
    // Create a buffer owned by `object` and populate that buffer with `bytes`.
    let buffer = unsafe {
        let data = Schema_AllocateBuffer(object.as_mut(), bytes.len() as _);
        slice::from_raw_parts_mut(data, bytes.len())
    };
    buffer.copy_from_slice(bytes);

    // Add `buffer` to `object` as the field.
    unsafe {
        Schema_AddBytes(object.as_ptr(), field, buffer.as_ptr(), buffer.len() as _);
    }
}

fn get_bytes(object: ObjectRef<'_>, field: FieldId) -> &[u8] {
    unsafe {
        let data = Schema_GetBytes(object.as_ptr(), field);
        let len = Schema_GetBytesLength(object.as_ptr(), field);
        std::slice::from_raw_parts(data, len as usize)
    }
}

fn index_bytes(object: ObjectRef<'_>, field: FieldId, index: u32) -> &[u8] {
    unsafe {
        let data = Schema_IndexBytes(object.as_ptr(), field, index);
        let len = Schema_IndexBytesLength(object.as_ptr(), field, index);
        std::slice::from_raw_parts(data, len as usize)
    }
}
