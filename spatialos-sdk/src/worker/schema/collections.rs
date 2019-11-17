use crate::worker::schema::{Field, FieldId, SchemaObject};
use spatialos_sdk_sys::worker::*;
use std::{collections::BTreeMap, marker::PhantomData};

#[derive(Debug)]
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

    fn add_list(_object: &mut SchemaObject, _field: FieldId, _value: &[Self::RustType]) {
        panic!()
    }
}
