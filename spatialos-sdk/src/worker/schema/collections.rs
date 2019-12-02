use crate::worker::schema::{Error, Field, FieldId, Result, SchemaComponentUpdate, SchemaObject};
use spatialos_sdk_sys::worker::*;
use std::{collections::BTreeMap, marker::PhantomData};

/// Marker type corresponding to the [`option`] schemalang collection type.
///
/// `option<T>` is represented as [`Option<U>`][option] in the generated Rust code,
/// where `U` is the Rust type corresponding to `T`. This type is named `Optional`
/// instead of `Option` to avoid conflicting with Rust's built-in [`Option`][option]
/// type.
///
/// See the [module-level documentation](index.html) for more information.
///
/// [`option`]: https://docs.improbable.io/reference/14.2/shared/schema/reference#collection-types
/// [option]: https://doc.rust-lang.org/std/option/index.html
pub struct Optional<T>(PhantomData<T>);

impl<T> Field for Optional<T>
where
    T: Field,
{
    type RustType = Option<T::RustType>;

    fn get(object: &SchemaObject, field: FieldId) -> Result<Self::RustType> {
        if T::count(object, field) > 0 {
            object.get::<T>(field).map(Some)
        } else {
            Ok(None)
        }
    }

    fn add(object: &mut SchemaObject, field: FieldId, value: &Self::RustType) {
        if let Some(value) = value {
            object.add::<T>(field, value);
        }
    }

    fn has_update(update: &SchemaComponentUpdate, field: FieldId) -> bool {
        T::count(update.fields(), field) > 0 || update.is_field_cleared(field)
    }

    fn get_update(
        update: &SchemaComponentUpdate,
        field: FieldId,
    ) -> Result<Option<Self::RustType>> {
        if update.is_field_cleared(field) {
            Ok(Some(None))
        } else {
            Self::get(update.fields(), field).map(Some)
        }
    }

    fn add_update(
        update: &mut SchemaComponentUpdate,
        field: FieldId,
        value: &Option<Self::RustType>,
    ) {
        match value {
            Some(Some(value)) => {
                update.fields_mut().add::<T>(field, value);
            }

            Some(None) => update.add_cleared(field),

            None => {}
        }
    }

    fn index(_object: &SchemaObject, _field: FieldId, _index: usize) -> Result<Self::RustType> {
        panic!("Optional fields cannot be indexed into")
    }

    fn count(_object: &SchemaObject, _field: FieldId) -> usize {
        panic!("Optional fields cannot be counted")
    }
}

/// Marker type corresponding to the [`list`] schemalang collection type.
///
/// `list<T>` is represented as [`Vec<U>`][vec] in the Rust code, where `U` is the Rust
/// type corresponding to `T`.
///
/// See the [module-level documentation](index.html) for more information.
///
/// [`list`]: https://docs.improbable.io/reference/14.2/shared/schema/reference#collection-types
/// [vec]: https://doc.rust-lang.org/std/vec/struct.Vec.html
pub struct List<T>(PhantomData<T>);

impl<T> Field for List<T>
where
    T: Field,
{
    type RustType = Vec<T::RustType>;

    fn get(object: &SchemaObject, field: FieldId) -> Result<Self::RustType> {
        let count = object.count::<T>(field);
        let mut result = Vec::with_capacity(count);
        for index in 0..count {
            let value = object
                .get_index::<T>(field, index)
                .map_err(Error::at_index::<Self>(field, index))?;
            result.push(value);
        }

        Ok(result)
    }

    fn add(object: &mut SchemaObject, field: FieldId, values: &Self::RustType) {
        for value in values {
            object.add::<T>(field, value);
        }
    }

    fn has_update(update: &SchemaComponentUpdate, field: FieldId) -> bool {
        T::count(update.fields(), field) > 0 || update.is_field_cleared(field)
    }

    fn get_update(
        update: &SchemaComponentUpdate,
        field: FieldId,
    ) -> Result<Option<Self::RustType>> {
        if update.is_field_cleared(field) {
            Ok(Some(Default::default()))
        } else if T::count(update.fields(), field) > 0 {
            Ok(Some(Self::get(update.fields(), field)?))
        } else {
            Ok(None)
        }
    }

    fn add_update(
        update: &mut SchemaComponentUpdate,
        field: FieldId,
        value: &Option<Self::RustType>,
    ) {
        if let Some(value) = value {
            if value.is_empty() {
                update.add_cleared(field);
            } else {
                Self::add(update.fields_mut(), field, value);
            }
        }
    }

    fn index(_object: &SchemaObject, _field: FieldId, _index: usize) -> Result<Self::RustType> {
        panic!("List fields cannot be indexed into")
    }

    fn count(_object: &SchemaObject, _field: FieldId) -> usize {
        panic!("List fields cannot be counted")
    }
}

/// Marker type corresponding to the [`map`] schemalang collection type.
///
/// `map<K, V>` is represented as [`BTreeMap<T, U>`][btree] in the generated Rust
/// code, where `T` is the Rust type corresponding to `K`, and `U` is the Rust type
/// corresponding to `V`.
///
/// [`BTreeMap`][btree] is used instead of [`HashMap`] in order to provide
/// deterministic ordering of values within the map. This allows the underlying
/// serialization library to generate smaller diffs when sending updates of schema
/// data containing a `map`.
///
/// See the [module-level documentation](index.html) for more information.
///
/// [`map`]: https://docs.improbable.io/reference/14.2/shared/schema/reference#collection-types
/// [btree]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
/// [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
pub struct Map<K, V>(PhantomData<(K, V)>);

impl<K, V> Field for Map<K, V>
where
    K: Field,
    V: Field,
    K::RustType: Ord,
{
    type RustType = BTreeMap<K::RustType, V::RustType>;

    fn get(object: &SchemaObject, field: FieldId) -> Result<Self::RustType> {
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

            let key = K::get(&pair, SCHEMA_MAP_KEY_FIELD_ID)
                .map_err(Error::at_index::<Self>(field, index))?;

            let value = V::get(&pair, SCHEMA_MAP_VALUE_FIELD_ID)
                .map_err(Error::at_index::<Self>(field, index))?;

            result.insert(key, value);
        }

        Ok(result)
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

    fn has_update(update: &SchemaComponentUpdate, field: FieldId) -> bool {
        update.fields().object_count(field) > 0 || update.is_field_cleared(field)
    }

    fn get_update(
        update: &SchemaComponentUpdate,
        field: FieldId,
    ) -> Result<Option<Self::RustType>> {
        if update.is_field_cleared(field) {
            Ok(Some(Default::default()))
        } else if update.fields().object_count(field) > 0 {
            Self::get(update.fields(), field).map(Some)
        } else {
            Ok(None)
        }
    }

    fn add_update(
        update: &mut SchemaComponentUpdate,
        field: FieldId,
        value: &Option<Self::RustType>,
    ) {
        if let Some(value) = value {
            if value.is_empty() {
                update.add_cleared(field);
            } else {
                Self::add(update.fields_mut(), field, value);
            }
        }
    }

    fn index(_object: &SchemaObject, _field: FieldId, _index: usize) -> Result<Self::RustType> {
        panic!("Map fields cannot be indexed into");
    }

    fn count(_object: &SchemaObject, _field: FieldId) -> usize {
        panic!("Map fields cannot be counted");
    }
}
