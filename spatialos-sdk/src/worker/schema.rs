//! Rust wrappers around the various schema data types defined by the C SDK.
//!
//! TODO: Document the conventions around how we handle correctness and safety
//! around schema data:
//!
//! * By convention there are two lifetimes for each type: `'owner` and `'data`. The
//!   immutable view only uses `'owner`, the owned version uses `'data`.
//! * There are two Rust types for each C type: An immutable, borrowed version; And
//!   a mutable, owned version.

use crate::worker::{
    component::{Component, ComponentId},
    internal::Sealed,
    EntityId,
};
use spatialos_sdk_sys::worker::*;
use std::{collections::BTreeMap, marker::PhantomData, mem, ptr::NonNull};

pub type FieldId = u32;

/// Serialized schema data for a component, owned by the Rust SDK.
///
/// For maximum efficiency, the serialized data may borrow data from the component
/// used to create an `OwnedComponentData` instance. The lifetime parameter
/// tracks this borrow, such that an `OwnedComponentData` cannot outlive the
/// data it borrows.
#[derive(Debug)]
pub(crate) struct ComponentData<'data> {
    raw: NonNull<Schema_ComponentData>,
    _marker: PhantomData<&'data ()>,
}

impl<'data> ComponentData<'data> {
    pub fn new<C: Component>(component: &'data C) -> Self {
        // Create the underlying `Schema_ComponentData` and retrieve the fields object.
        let raw = NonNull::new(unsafe { Schema_CreateComponentData(C::ID) }).unwrap();
        let mut result = Self {
            raw,
            _marker: PhantomData,
        };

        // Populate the schema data from the component.
        component.into_object(&mut result.fields_mut());

        result
    }

    pub fn component_id(&self) -> ComponentId {
        unsafe { Schema_GetComponentDataComponentId(self.raw.as_ref()) }
    }

    pub fn fields<'owner>(&'owner self) -> ObjectRef<'owner> {
        self.as_ref().fields()
    }

    pub fn fields_mut<'owner>(&'owner mut self) -> ObjectMut<'owner, 'data> {
        unsafe { ObjectMut::from_raw(&mut *Schema_GetComponentDataFields(self.raw.as_ptr())) }
    }

    pub fn as_ref<'owner>(&'owner self) -> ComponentDataRef<'owner> {
        ComponentDataRef {
            raw: unsafe { self.raw.as_ref() },
        }
    }

    pub fn deserialize<T: SchemaObjectType>(&self) -> T {
        T::from_object(self.fields())
    }

    /// Converts the `OwnedComponentData` into a `*mut Schema_ComponentData` that can be
    /// passed to the C API.
    ///
    /// This transfers ownership of the data to the caller, so the caller needs to
    /// ensure that the appropriate steps are taken to free the data. If the raw data is
    /// passed to the C API, the C SDK will take ownership of the data and will free it
    /// when it's done.
    pub fn into_raw(self) -> *mut Schema_ComponentData {
        let raw = self.raw;
        mem::forget(self);
        raw.as_ptr()
    }
}

impl<'data> Drop for ComponentData<'data> {
    fn drop(&mut self) {
        unsafe {
            Schema_DestroyComponentData(self.raw.as_ptr());
        }
    }
}

/// Serialized schema data for a compnent owned by the C SDK.
///
/// The lifetime parameter tracks the parent data that owns the schema data
/// (generally an `OpList`), such the `SchemaComponentData` instance cannot live
/// its parent.
#[derive(Debug)]
pub(crate) struct ComponentDataRef<'owner> {
    raw: &'owner Schema_ComponentData,
}

impl<'owner> ComponentDataRef<'owner> {
    pub unsafe fn from_raw(raw: &'owner Schema_ComponentData) -> Self {
        Self { raw }
    }

    pub fn component_id(&self) -> ComponentId {
        unsafe { Schema_GetComponentDataComponentId(self.raw) }
    }

    pub fn fields(&self) -> ObjectRef<'owner> {
        unsafe {
            ObjectRef::from_raw(&*Schema_GetComponentDataFields(
                self.raw as *const _ as *mut _,
            ))
        }
    }

    pub fn deserialize<T: SchemaObjectType>(&self) -> T {
        T::from_object(self.fields())
    }
}

#[derive(Debug)]
pub struct ObjectMut<'owner, 'data> {
    raw: &'owner mut Schema_Object,
    _marker: PhantomData<&'data ()>,
}

impl<'owner, 'data> ObjectMut<'owner, 'data> {
    pub(crate) unsafe fn from_raw(raw: &'owner mut Schema_Object) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    pub fn add_field<T: SchemaField>(&mut self, field: FieldId, value: &'data T::RustType) {
        T::add_field(self, field, value);
    }

    pub fn add_field_array<T: ArrayField>(&mut self, field: FieldId, value: &'data [T::RustType]) {
        T::add_field_list(self, field, value);
    }

    pub fn as_ref(&'owner self) -> ObjectRef<'owner> {
        ObjectRef { raw: self.raw }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ObjectRef<'a> {
    raw: &'a Schema_Object,
}

impl<'a> ObjectRef<'a> {
    pub unsafe fn from_raw(raw: &'a Schema_Object) -> Self {
        Self { raw }
    }

    pub fn field<T: SchemaField>(self, field: FieldId) -> T::RustType {
        T::get_field(self, field)
    }

    pub fn field_array<T: ArrayField>(self, field: FieldId) -> Vec<T::RustType> {
        let mut result = Vec::new();
        T::get_field_list(self, field, &mut result);
        result
    }
}

// =================================================================================================
// Schema Conversion Traits
// =================================================================================================

pub trait SchemaField: Sized + Sealed {
    type RustType: Sized;

    fn add_field<'owner, 'data>(
        object: &mut ObjectMut<'owner, 'data>,
        field: FieldId,
        value: &'data Self::RustType,
    );

    fn get_field(object: ObjectRef, field: FieldId) -> Self::RustType;
}

pub trait IndexedField: SchemaField {
    fn field_count(object: ObjectRef, field: FieldId) -> u32;

    fn index_field(object: ObjectRef, field: FieldId, index: u32) -> Self::RustType;
}

pub trait ArrayField: IndexedField {
    fn add_field_list<'owner, 'data>(
        object: &mut ObjectMut<'owner, 'data>,
        field: FieldId,
        data: &'data [Self::RustType],
    );

    fn get_field_list(object: ObjectRef, field: FieldId, data: &mut Vec<Self::RustType>);
}

/// A type that can be deserialized from an entire `SchemaObject`.
pub trait SchemaObjectType: Sized {
    fn into_object<'owner, 'data>(&'data self, object: &mut ObjectMut<'owner, 'data>);
    fn from_object(object: ObjectRef) -> Self;
}

impl<T: SchemaObjectType> Sealed for T {}

impl<T: SchemaObjectType> SchemaField for T {
    type RustType = Self;

    fn add_field<'owner, 'data>(
        object: &mut ObjectMut<'owner, 'data>,
        field: FieldId,
        value: &'data Self::RustType,
    ) {
        let mut field_object =
            unsafe { ObjectMut::from_raw(&mut *Schema_AddObject(object.raw, field)) };
        value.into_object(&mut field_object);
    }

    fn get_field<'owner>(object: ObjectRef<'owner>, field: FieldId) -> Self::RustType {
        let field_object = unsafe {
            ObjectRef::from_raw(&*Schema_GetObject(object.raw as *const _ as *mut _, field))
        };
        T::from_object(field_object)
    }
}

impl<T: SchemaObjectType> IndexedField for T {
    fn field_count<'owner>(object: ObjectRef<'owner>, field: FieldId) -> u32 {
        unsafe { Schema_GetObjectCount(object.raw, field) }
    }

    fn index_field<'owner>(
        object: ObjectRef<'owner>,
        field: FieldId,
        index: u32,
    ) -> Self::RustType {
        let field_object = unsafe {
            ObjectRef::from_raw(&*Schema_IndexObject(
                object.raw as *const _ as *mut _,
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

            fn add_field<'owner, 'data>(
                object: &mut ObjectMut<'owner, 'data>,
                field: FieldId,
                value: &'data Self::RustType,
            ) {
                let value = (*value).into();
                unsafe {
                    $schema_add(object.raw, field, value);
                }
            }

            #[allow(clippy::useless_transmute, clippy::transmute_int_to_bool)]
            fn get_field(object: ObjectRef, field: FieldId) -> Self::RustType {
                unsafe { mem::transmute($schema_get(object.raw, field)) }
            }
        }

        impl IndexedField for $schema_type {
            fn field_count(object: ObjectRef, field: FieldId) -> u32 {
                unsafe { $schema_count(object.raw, field) }
            }

            #[allow(clippy::useless_transmute, clippy::transmute_int_to_bool)]
            fn index_field(object: ObjectRef, field: FieldId, index: u32) -> Self::RustType {
                unsafe { mem::transmute($schema_index(object.raw, field, index)) }
            }
        }

        impl ArrayField for $schema_type {
            fn add_field_list<'owner, 'data>(
                object: &mut ObjectMut<'owner, 'data>,
                field: FieldId,
                data: &'data [Self::RustType],
            ) {
                unsafe {
                    $schema_add_list(
                        object.raw,
                        field,
                        data.as_ptr() as *const _,
                        data.len() as u32,
                    );
                }
            }

            fn get_field_list(object: ObjectRef, field: FieldId, data: &mut Vec<Self::RustType>) {
                let count = Self::field_count(object, field) as usize;

                // Ensure that there is enough capacity for the elements in the schema field.
                if data.capacity() < count {
                    data.reserve(count - data.capacity());
                }

                // Replace the contents of `data` with the list of values in the schema field.
                unsafe {
                    data.set_len(count);
                    $schema_get_list(object.raw, field, data.as_mut_ptr() as *mut _);
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

    fn add_field<'owner, 'data>(
        object: &mut ObjectMut<'owner, 'data>,
        field: FieldId,
        value: &'data Self::RustType,
    ) {
        if let Some(value) = value {
            T::add_field(object, field, value);
        }
    }

    fn get_field<'owner>(object: ObjectRef<'owner>, field: FieldId) -> Self::RustType {
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

    fn add_field<'owner, 'data>(
        object: &mut ObjectMut<'owner, 'data>,
        field: FieldId,
        map: &'data Self::RustType,
    ) {
        // Create a key-value pair object for each entry in the map.
        for (key, value) in map {
            let mut pair =
                unsafe { ObjectMut::from_raw(&mut *Schema_AddObject(object.raw, field)) };
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
                    object.raw as *const _ as *mut _,
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

    fn add_field<'owner, 'data>(
        object: &mut ObjectMut<'owner, 'data>,
        field: FieldId,
        values: &'data Self::RustType,
    ) {
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

    fn add_field<'owner, 'data>(
        object: &mut ObjectMut<'owner, 'data>,
        field: FieldId,
        value: &'data Self::RustType,
    ) {
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
        unsafe { Schema_GetBytesCount(object.raw, field) }
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

    fn add_field<'owner, 'data>(
        object: &mut ObjectMut<'owner, 'data>,
        field: FieldId,
        value: &'data Self::RustType,
    ) {
        add_bytes(object, field, value);
    }

    fn get_field(object: ObjectRef, field: FieldId) -> Self::RustType {
        get_bytes(object, field).into()
    }
}

impl IndexedField for Vec<u8> {
    fn field_count(object: ObjectRef, field: FieldId) -> u32 {
        unsafe { Schema_GetBytesCount(object.raw, field) }
    }

    fn index_field(object: ObjectRef, field: FieldId, index: u32) -> Self::RustType {
        index_bytes(object, field, index).into()
    }
}

fn add_bytes<'owner, 'data>(
    object: &mut ObjectMut<'owner, 'data>,
    field: FieldId,
    bytes: &'data [u8],
) {
    unsafe {
        Schema_AddBytes(object.raw, field, bytes.as_ptr(), bytes.len() as u32);
    }
}

fn get_bytes(object: ObjectRef, field: FieldId) -> &[u8] {
    unsafe {
        let data = Schema_GetBytes(object.raw, field);
        let len = Schema_GetBytesLength(object.raw, field);
        std::slice::from_raw_parts(data, len as usize)
    }
}

fn index_bytes(object: ObjectRef, field: FieldId, index: u32) -> &[u8] {
    unsafe {
        let data = Schema_IndexBytes(object.raw, field, index);
        let len = Schema_IndexBytesLength(object.raw, field, index);
        std::slice::from_raw_parts(data, len as usize)
    }
}
