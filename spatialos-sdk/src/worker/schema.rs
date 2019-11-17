use crate::worker::component::TypeConversion;

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

impl<T> Field for T
where
    T: TypeConversion,
{
    type RustType = Self;

    fn get_or_default(object: &SchemaObject, field: FieldId) -> Self::RustType {
        Self::from_type(object.get_object(field))
            .unwrap_or_else(|_| panic!("Failed to deserialize {}", std::any::type_name::<Self>()))
    }

    fn index(object: &SchemaObject, field: FieldId, index: usize) -> Self::RustType {
        Self::from_type(object.index_object(field, index))
            .unwrap_or_else(|_| panic!("Failed to deserialize {}", std::any::type_name::<Self>()))
    }

    fn count(object: &SchemaObject, field: FieldId) -> usize {
        object.object_count(field)
    }

    fn add(object: &mut SchemaObject, field: FieldId, value: &Self::RustType) {
        Self::to_type(value, object.add_object(field))
            .unwrap_or_else(|_| panic!("Failed to serialize {}", std::any::type_name::<Self>()))
    }
}
