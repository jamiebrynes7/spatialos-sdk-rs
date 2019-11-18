//! Logic for serializing schema data.
//!
//! # Examples
//!
//! Serialization example using `Optional`:
//!
//! ```
//! use spatialos_sdk::worker::schema::*;
//!
//! let mut data = SchemaComponentData::new();
//! let object = data.fields_mut();
//!
//! let some_field = Some(123);
//! let none_field = None;
//!
//! // Add the fields to the serialized data.
//! object.add::<Optional<SchemaInt32>>(1, &some_field);
//! object.add::<Optional<SchemaString>>(2, &none_field);
//!
//! // Get fields from serialized data.
//! let some_result = object.get::<Optional<SchemaInt32>>(1);
//! let none_result = object.get::<Optional<SchemaString>>(2);
//!
//! assert_eq!(some_field, some_result);
//! assert_eq!(none_field, none_result);
//! ```
//!
//! For the following schema type definition:
//!
//! ```schemalang,ignore
//! type MyType {
//!     option<int32> optional_value = 1;
//! }
//! ```
//!
//! The following code would be used to handle serialization:
//!
//! ```
//! use spatialos_sdk::worker::schema::*;
//!
//! #[derive(Debug, Default, Clone)]
//! pub struct MyType {
//!     pub optional_value: Option<i32>,
//! }
//!
//! impl ObjectField for MyType {
//!     fn from_object(object: &SchemaObject) -> Self {
//!         Self {
//!             optional_value: object.get::<Optional<SchemaInt32>>(1),
//!         }
//!     }
//!
//!     fn into_object(&self, object: &mut SchemaObject) {
//!         object.add::<Optional<SchemaInt32>>(1, &self.optional_value);
//!     }
//! }
//! ```

use std::{
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};

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
#[doc(inline)]
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

    fn has_update(update: &SchemaComponentUpdate, field: FieldId) -> bool;

    fn get_update(update: &SchemaComponentUpdate, field: FieldId) -> Option<Self::RustType> {
        if Self::has_update(update, field) {
            Some(Self::get_or_default(update.fields(), field))
        } else {
            None
        }
    }

    fn add_update(update: &mut SchemaComponentUpdate, field: FieldId, value: &Self::RustType) {
        Self::add(update.fields_mut(), field, value);
    }
}

/// A struct that can be serialized into (and deserialized from) a [`SchemaObject`].
///
/// Schemalang [components] and [types], as well as the various associated types
/// generated for components (i.e. updates and commands), are represented as a
/// [`SchemaObject`] when serialized. This trait defines the conversion for such
/// types, and provides a blanket implementation of the [`Field`] trait so that the
/// types can be serialized as part of larger objects.
///
/// You should generally not have to implement this trait manually for any types.
/// The implementation for any schema-defined types will be generated for you by the
/// code generator provided with the SDK.
///
/// [components]: https://docs.improbable.io/reference/14.2/shared/schema/reference#components
/// [types]: https://docs.improbable.io/reference/14.2/shared/schema/reference#types
/// [`SchemaObject`]: struct.SchemaObject.html
/// [`Field`]: trait.Field.html
pub trait ObjectField: Sized + Clone {
    /// Deserializes an instance of this type from a [`SchemaObject`].
    ///
    /// Note that deserialization can never fail as the underlying SpatialOS API ensures
    /// that missing or invalid fields will always provide a valid default value during
    /// deserialization. As such, if the data in `object` doesn't match what is expected
    /// by the deserialization logic (e.g. due to a version mismatch between the current
    /// schema version and the version `object` was generated from), the returned object
    /// will have default values for any fields that are missing in `object` or have a
    /// type differing from what is expected.
    ///
    /// [`SchemaObject`]: struct.SchemaObject.html
    fn from_object(object: &SchemaObject) -> Self;

    /// Serializes an instance of this type into a [`SchemaObject`].
    ///
    /// Note that serialization can never fail, as there is no way to form an invalid
    /// instance of a schema object type.
    ///
    /// [`SchemaObject`]: struct.SchemaObject.html
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

    fn has_update(update: &SchemaComponentUpdate, field: FieldId) -> bool {
        Self::count(update.fields(), field) > 0
    }
}

/// The error type returned when a conversion to a schema enum type fails.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnknownDiscriminantError {
    pub type_name: &'static str,
    pub value: u32,
}

impl Display for UnknownDiscriminantError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Found unknown discriminant value {} for enum `{}` while deserializing schema data",
            self.value, self.type_name,
        )
    }
}

impl std::error::Error for UnknownDiscriminantError {}

/// A type that corresponds to a [schemalang enumeration][schema-enum].
///
/// Schemalang enums are C-like enums (i.e. they only have a discriminant and do not
/// contain any additional data) represented as `u32` values when serialized. They
/// default to the first variant defined in their schema declaration.
///
/// Converting the serialized `u32` representation back to the Rust representation
/// can fail if the serialized discriminant doesn't match any of the recognized
/// variants (e.g. if the serialized data was generated from a different schema
/// version than what the current program is using). As such, conversion is handled
/// using the [`TryFrom`] trait, with [`UnknownDiscriminantError`] indicating that
/// value couldn't be converted.
///
/// You should generally not have to implement this trait manually for any types.
/// The implementation for any schema-defined types will be generated for you by the
/// code generator provided with the SDK.
///
/// [schema-enum]: https://docs.improbable.io/reference/14.2/shared/schema/reference#enumerations
/// [`TryFrom`]: https://doc.rust-lang.org/std/convert/trait.TryFrom.html
/// [`UnknownDiscriminantError`]: struct.UnknownDiscriminantError.html
pub trait EnumField:
    Sized + Clone + Copy + Default + TryFrom<u32, Error = UnknownDiscriminantError> + Into<u32>
{
}

/// Helper macro for generating the [`Field`] implementation for a schema enum.
///
/// Ideally we would like to provide a blanket impl of [`Field`] for all types
/// implementing [`EnumField`] the way we do with [`ObjectField`], but Rust's
/// current coherence rules mean that there can only be one blanket impl for a given
/// trait. For now, this macro at least provides a lightweight way to implement the
/// trait for such types. Once the coherence rules [have been made more flexible][rfc-1053],
/// we can remove this macro and use a regular blanket impl.
///
/// You should generally not have to use this macro manually. The code generated for
/// schema-defined enums will handle this for you.
///
/// [`Field`]: trait.Field.html
/// [`EnumField`]: trait.EnumField.html
/// [`ObjectField`]: trait.ObjectField.html
/// [rfc-1053]: https://github.com/rust-lang/rfcs/issues/1053
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

            fn has_update(
                update: &$crate::worker::schema::SchemaComponentUpdate,
                field: $crate::worker::schema::FieldId,
            ) -> bool {
                Self::count(update.fields(), field) > 0
            }
        }
    };
}
