//! Logic for serializing schema data.
//!
//! # Examples
//!
//! Serialization example using `Optional`:
//!
//! ```
//! use spatialos_sdk::schema::*;
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
//! let some_result = object.get::<Optional<SchemaInt32>>(1).unwrap();
//! let none_result = object.get::<Optional<SchemaString>>(2).unwrap();
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
//! use spatialos_sdk::schema::{self, *};
//!
//! #[derive(Debug, Default, Clone)]
//! pub struct MyType {
//!     pub optional_value: Option<i32>,
//! }
//!
//! impl ObjectField for MyType {
//!     fn from_object(object: &SchemaObject) -> schema::Result<Self> {
//!         Ok(Self {
//!             optional_value: object
//!                 .get::<Optional<SchemaInt32>>(1)
//!                 .map_err(Error::at_field::<Self>(1))?,
//!         })
//!     }
//!
//!     fn into_object(&self, object: &mut SchemaObject) {
//!         object.add::<Optional<SchemaInt32>>(1, &self.optional_value);
//!     }
//! }
//! ```

use crate::commands::CommandIndex;
use std::{
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};

// NOTE: This module defines macros that are used in other submodules, so it must be
// declared first in order for those macros to be visible to all sibling modules.
#[cfg(test)]
#[macro_use]
mod macros;

mod bundle;
mod collections;
mod command_request;
mod command_response;
mod component_data;
mod component_update;
mod float_ord;
mod generic_data;
mod object;
mod primitives;
mod ptr;

pub mod owned;

pub use self::{
    bundle::*, collections::*, command_request::*, command_response::*, component_data::*,
    component_update::*, float_ord::*, generic_data::*, object::*, owned::Owned, primitives::*,
};
#[doc(inline)]
pub use crate::impl_field_for_enum_field;

pub(crate) use self::ptr::*;

pub type FieldId = u32;

pub trait Field {
    type RustType: Sized;

    fn get(object: &SchemaObject, field: FieldId) -> Result<Self::RustType>;

    fn add(object: &mut SchemaObject, field: FieldId, value: &Self::RustType);

    fn has_update(update: &SchemaComponentUpdate, field: FieldId) -> bool;

    fn get_update(
        update: &SchemaComponentUpdate,
        field: FieldId,
    ) -> Result<Option<Self::RustType>> {
        if Self::has_update(update, field) {
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
            Self::add(update.fields_mut(), field, value);
        }
    }
}

pub trait IndexedField: Field {
    fn count(object: &SchemaObject, field: FieldId) -> usize;

    fn index(object: &SchemaObject, field: FieldId, index: usize) -> Result<Self::RustType>;

    fn add_list(object: &mut SchemaObject, field: FieldId, values: &[Self::RustType]) {
        for value in values {
            Self::add(object, field, value);
        }
    }

    fn get_list(object: &SchemaObject, field: FieldId) -> Result<Vec<Self::RustType>> {
        let count = Self::count(object, field);
        (0..count)
            .map(|index| Self::index(object, field, index))
            .collect()
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
    fn from_object(object: &SchemaObject) -> Result<Self>;

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

    fn get(object: &SchemaObject, field: FieldId) -> Result<Self::RustType> {
        Self::from_object(object.get_object(field))
    }

    fn add(object: &mut SchemaObject, field: FieldId, value: &Self::RustType) {
        value.into_object(object.add_object(field));
    }

    fn has_update(update: &SchemaComponentUpdate, field: FieldId) -> bool {
        Self::count(update.fields(), field) > 0
    }
}

impl<T> IndexedField for T
where
    T: ObjectField,
{
    fn count(object: &SchemaObject, field: FieldId) -> usize {
        object.object_count(field)
    }

    fn index(object: &SchemaObject, field: FieldId, index: usize) -> Result<Self::RustType> {
        Self::from_object(object.index_object(field, index))
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
        impl $crate::schema::Field for $type {
            type RustType = Self;

            fn get(
                object: &$crate::schema::SchemaObject,
                field: $crate::schema::FieldId,
            ) -> $crate::schema::Result<Self::RustType> {
                Self::try_from(object.get::<$crate::schema::SchemaEnum>(field)?)
                    .map_err(Into::into)
            }

            fn add(
                object: &mut $crate::schema::SchemaObject,
                field: $crate::schema::FieldId,
                value: &Self::RustType,
            ) {
                object.add::<$crate::schema::SchemaEnum>(field, &(*value).into());
            }

            fn has_update(
                update: &$crate::schema::SchemaComponentUpdate,
                field: $crate::schema::FieldId,
            ) -> bool {
                Self::count(update.fields(), field) > 0
            }
        }

        impl $crate::schema::IndexedField for $type {
            fn count(
                object: &$crate::schema::SchemaObject,
                field: $crate::schema::FieldId,
            ) -> usize {
                object.count::<$crate::schema::SchemaEnum>(field)
            }

            fn index(
                object: &$crate::schema::SchemaObject,
                field: $crate::schema::FieldId,
                index: usize,
            ) -> $crate::schema::Result<Self::RustType> {
                Self::try_from(
                    object.get_index::<$crate::schema::SchemaEnum>(field, index)?,
                )
                .map_err(Into::into)
            }
        }
    };
}

pub type Result<T> = std::result::Result<T, Error>;

/// An error that can occur during schema deserialization.
#[derive(Debug)]
pub struct Error {
    type_name: &'static str,
    kind: ErrorKind,
}

impl Error {
    pub fn unknown_discriminant<T: EnumField>(value: u32) -> Self {
        Self {
            type_name: std::any::type_name::<T>(),
            kind: ErrorKind::UnknownDiscriminant(value),
        }
    }

    pub fn unknown_command<T>(command_index: CommandIndex) -> Self {
        Self {
            type_name: std::any::type_name::<T>(),
            kind: ErrorKind::UnknownCommand(command_index),
        }
    }

    pub fn missing_field<T>() -> Self {
        Self {
            type_name: std::any::type_name::<T>(),
            kind: ErrorKind::MissingField,
        }
    }

    pub fn index_out_of_bounds<T>(index: usize, count: usize) -> Self {
        Self {
            type_name: std::any::type_name::<T>(),
            kind: ErrorKind::IndexOutOfBounds { index, count },
        }
    }

    pub fn at_field<T>(field: FieldId) -> impl FnOnce(Self) -> Self {
        move |error| Self {
            type_name: std::any::type_name::<T>(),
            kind: ErrorKind::InvalidValue {
                field,
                index: None,
                error: Box::new(error),
            },
        }
    }

    pub fn at_index<T>(field: FieldId, index: usize) -> impl FnOnce(Self) -> Self {
        move |error| Self {
            type_name: std::any::type_name::<T>(),
            kind: ErrorKind::InvalidValue {
                field,
                index: Some(index),
                error: Box::new(error),
            },
        }
    }

    pub fn schema_error<T>(msg: String) -> Self {
        Self {
            type_name: std::any::type_name::<T>(),
            kind: ErrorKind::SchemaError(msg),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::UnknownDiscriminant(value) => write!(
                f,
                "Unknown discriminant {} for enum {}",
                value, self.type_name
            ),

            ErrorKind::UnknownCommand(command_index) => write!(
                f,
                "Unknown command index {} for command {}",
                command_index, self.type_name
            ),

            ErrorKind::MissingField => write!(f, "Missing field of type {}", self.type_name),

            ErrorKind::IndexOutOfBounds { index, count } => write!(
                f,
                "Index out of bounds for type {}, index: {}, count: {}",
                self.type_name, index, count
            ),

            ErrorKind::InvalidValue {
                field,
                index,
                error,
            } => match index {
                Some(index) => write!(
                    f,
                    "Invalid value in field {} at index {} for type {}: {}",
                    field, index, self.type_name, error
                ),

                None => write!(
                    f,
                    "Invalid value in field {} for type {}: {}",
                    field, self.type_name, error
                ),
            },

            ErrorKind::SchemaError(msg) => write!(f, "Generic schema error {}", msg),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let ErrorKind::InvalidValue { error, .. } = &self.kind {
            return Some(&**error as &dyn std::error::Error);
        }

        None
    }
}

impl From<UnknownDiscriminantError> for Error {
    fn from(from: UnknownDiscriminantError) -> Self {
        Self {
            type_name: from.type_name,
            kind: ErrorKind::UnknownDiscriminant(from.value),
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    UnknownDiscriminant(u32),
    UnknownCommand(CommandIndex),
    MissingField,
    IndexOutOfBounds {
        index: usize,
        count: usize,
    },
    InvalidValue {
        field: FieldId,
        index: Option<usize>,
        error: Box<Error>,
    },
    SchemaError(String),
}
