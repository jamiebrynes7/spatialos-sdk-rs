use std::convert::TryFrom;

// NOTE: This module defines macros that are used in other submodules, so it must be
// declared first in order for those macros to be visible to all sibling modules.
#[cfg(test)]
#[macro_use]
mod macros;

mod collections;
mod command_request;
mod command_response;
mod component_data;
mod component_update;
mod object;
mod primitives;
mod ptr;

pub mod owned;

pub use self::{
    collections::*, command_request::*, command_response::*, component_data::*,
    component_update::*, object::*, owned::Owned, primitives::*,
};
pub use crate::impl_field_for_enum_field;

pub(crate) use self::ptr::*;

pub type FieldId = u32;

pub trait Field {
    type RustType: Sized;

    fn get_or_default(object: &SchemaObject, field: FieldId) -> Self::RustType;
    fn index(object: &SchemaObject, field: FieldId, index: usize) -> Self::RustType;
    fn count(object: &SchemaObject, field: FieldId) -> usize;
    fn add(object: &mut SchemaObject, field: FieldId, value: &Self::RustType);

    fn add_list(object: &mut SchemaObject, field: FieldId, values: &[Self::RustType]) {
        for value in values {
            Self::add(object, field, value);
        }
    }

    fn get_list(object: &SchemaObject, field: FieldId) -> Vec<Self::RustType> {
        let count = Self::count(object, field);
        let mut result = Vec::with_capacity(count);
        for index in 0..count {
            result.push(Self::index(object, field, index));
        }
        result
    }
}

pub trait ObjectField: Sized + Clone {
    fn from_object(object: &SchemaObject) -> Self;
    fn into_object(&self, object: &mut SchemaObject);
}

impl<T> Field for T
where
    T: ObjectField,
{
    type RustType = Self;

    fn get_or_default(object: &SchemaObject, field: FieldId) -> Self::RustType {
        Self::from_object(object.get_object(field))
    }

    fn index(object: &SchemaObject, field: FieldId, index: usize) -> Self::RustType {
        Self::from_object(object.index_object(field, index))
    }

    fn count(object: &SchemaObject, field: FieldId) -> usize {
        object.object_count(field)
    }

    fn add(object: &mut SchemaObject, field: FieldId, value: &Self::RustType) {
        value.into_object(object.add_object(field));
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnknownDiscriminantError;

pub trait EnumField:
    Sized + Clone + Copy + Default + TryFrom<u32, Error = UnknownDiscriminantError> + Into<u32>
{
}

#[macro_export]
macro_rules! impl_field_for_enum_field {
    ($type:ty) => {
        impl $crate::worker::schema::Field for $type {
            type RustType = Self;

            fn get_or_default(
                object: &$crate::worker::schema::SchemaObject,
                field: $crate::worker::schema::FieldId,
            ) -> Self::RustType {
                Self::try_from(object.get::<$crate::worker::schema::SchemaEnum>(field))
                    .unwrap_or_default()
            }

            fn index(
                object: &$crate::worker::schema::SchemaObject,
                field: $crate::worker::schema::FieldId,
                index: usize,
            ) -> Self::RustType {
                Self::try_from(object.get_index::<$crate::worker::schema::SchemaEnum>(field, index))
                    .unwrap_or_default()
            }

            fn count(
                object: &$crate::worker::schema::SchemaObject,
                field: $crate::worker::schema::FieldId,
            ) -> usize {
                object.count::<$crate::worker::schema::SchemaEnum>(field)
            }

            fn add(
                object: &mut $crate::worker::schema::SchemaObject,
                field: $crate::worker::schema::FieldId,
                value: &Self::RustType,
            ) {
                object.add::<$crate::worker::schema::SchemaEnum>(field, &(*value).into());
            }
        }
    };
}
