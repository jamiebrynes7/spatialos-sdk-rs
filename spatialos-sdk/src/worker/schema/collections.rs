use crate::worker::schema::{Field, FieldId, SchemaObject};
use spatialos_sdk_sys::worker::*;
use std::{collections::BTreeMap, marker::PhantomData};

pub struct Optional<T>(PhantomData<T>);

impl<T> Field for Optional<T>
where
    T: Field,
{
    type RustType = Option<T::RustType>;

    fn get_or_default(object: &SchemaObject, field: FieldId) -> Self::RustType {
        if T::count(object, field) > 0 {
            Some(object.get::<T>(field))
        } else {
            None
        }
    }

    fn add(object: &mut SchemaObject, field: FieldId, value: &Self::RustType) {
        if let Some(value) = value {
            object.add::<T>(field, value);
        }
    }

    fn index(_object: &SchemaObject, _field: FieldId, _index: usize) -> Self::RustType {
        panic!("Optional fields cannot be indexed into")
    }

    fn count(_object: &SchemaObject, _field: FieldId) -> usize {
        panic!("Optional fields cannot be counted")
    }
}

pub struct List<T>(PhantomData<T>);

impl<T> Field for List<T>
where
    T: Field,
{
    type RustType = Vec<T::RustType>;

    fn get_or_default(object: &SchemaObject, field: FieldId) -> Self::RustType {
        let count = object.count::<T>(field);
        let mut result = Vec::with_capacity(count);
        for index in 0..count {
            result.push(object.get_index::<T>(field, index));
        }

        result
    }

    fn add(object: &mut SchemaObject, field: FieldId, values: &Self::RustType) {
        for value in values {
            object.add::<T>(field, value);
        }
    }

    fn index(_object: &SchemaObject, _field: FieldId, _index: usize) -> Self::RustType {
        panic!("List fields cannot be indexed into")
    }

    fn count(_object: &SchemaObject, _field: FieldId) -> usize {
        panic!("List fields cannot be counted")
    }
}

pub struct Map<K, V>(PhantomData<(K, V)>);

impl<K, V> Field for Map<K, V>
where
    K: Field,
    V: Field,
    K::RustType: Ord,
{
    type RustType = BTreeMap<K::RustType, V::RustType>;

    fn get_or_default(object: &SchemaObject, field: FieldId) -> Self::RustType {
        let mut result = BTreeMap::new();

        // Load each of the key-value pairs from the map object.
        //
        // Map fields are represented in schema as a list of pairs of key and value. Each
        // entry in the map is an object field with field ID corresponding to the map's
        // field ID, and each object should have a key field with ID `SCHEMA_MAP_KEY_FIELD_ID`
        // and a value field with ID `SCHEMA_MAP_VALUE_FIELD_ID`.
        let count = object.object_count(field);
        for index in 0..count {
            let pair = object.index_object(field, index);
            let key = K::get_or_default(&pair, SCHEMA_MAP_KEY_FIELD_ID);
            let value = V::get_or_default(&pair, SCHEMA_MAP_VALUE_FIELD_ID);
            result.insert(key, value);
        }

        result
    }

    fn add(object: &mut SchemaObject, field: FieldId, map: &Self::RustType) {
        // Create a key-value pair object for each entry in the map.
        //
        // Map fields are represented in schema as a list of pairs of key and value. Each
        // entry in the map is an object field with field ID corresponding to the map's
        // field ID, and each object should have a key field with ID `SCHEMA_MAP_KEY_FIELD_ID`
        // and a value field with ID `SCHEMA_MAP_VALUE_FIELD_ID`.
        for (key, value) in map {
            let pair = object.add_object(field);
            pair.add::<K>(SCHEMA_MAP_KEY_FIELD_ID, key);
            pair.add::<V>(SCHEMA_MAP_VALUE_FIELD_ID, value);
        }
    }

    fn index(_object: &SchemaObject, _field: FieldId, _index: usize) -> Self::RustType {
        panic!("Map fields cannot be indexed into");
    }

    fn count(_object: &SchemaObject, _field: FieldId) -> usize {
        panic!("Map fields cannot be counted");
    }
}
