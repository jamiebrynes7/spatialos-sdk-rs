// NOTE: This module defines macros that are used in other submodules, so it must be
// declared first in order for those macros to be visible to all sibling modules.
#[cfg(test)]
#[macro_use]
mod macros;

mod command_request;
mod command_response;
mod component_data;
mod component_update;
mod object;
mod primitives;

pub mod owned;

pub use self::{
    command_request::*, command_response::*, component_data::*, component_update::*, object::*,
    owned::Owned, primitives::*,
};

pub(crate) use self::private::*;

pub type FieldId = u32;

pub trait SchemaPrimitiveField {
    type RustType: Sized;

    fn get_or_default(object: &SchemaObject, field: FieldId) -> Self::RustType;
    fn index(object: &SchemaObject, field: FieldId, index: usize) -> Self::RustType;
    fn count(object: &SchemaObject, field: FieldId) -> usize;
    fn add(object: &mut SchemaObject, field: FieldId, value: &Self::RustType);
    fn add_list(object: &mut SchemaObject, field: FieldId, value: &[Self::RustType]);

    fn get_list(object: &SchemaObject, field: FieldId) -> Vec<Self::RustType> {
        let count = Self::count(object, field);
        let mut result = Vec::with_capacity(count);
        for index in 0..count {
            result.push(Self::index(object, field, index));
        }
        result
    }
}

mod private {
    /// A type that acts as an alias to some schema data hidden behind a pointer.
    ///
    /// `Schema_Object` and the related schema data "wrapper types" are all represented
    /// as opaque types hidden behind pointers in the C API; You always work with a
    /// pointer to the struct, never an instance of the struct itself. To represent
    /// these in Rust, we define dummy types that can similarly always be put behind a
    /// reference without ever allowing the user to directly "hold" an instance of the
    /// type. This allows us to simply cast the raw pointer to a reference to our dummy
    /// type and preserve the desired semantics of working with a reference, including
    /// the ownership rules and lifetimes that come with it.
    ///
    /// This trait is implemented for all such types, and provides the common methods
    /// used for type punning with the raw pointer.
    ///
    /// # Safety
    ///
    /// In order for the implementation of this trait to be safe, the following
    /// invariants must be upheld:
    ///
    /// * It must be a [zero sized type][zst].
    /// * It must have a private field, such that the type can never be directly
    ///   instantiated by a user.
    /// * A reference to `Self` must never be de-referenced. It may only ever be
    ///   converted to a pointer with `as_ptr()`, which may then be passed to the
    ///   appropriate functions in the C API.
    ///
    /// The `pointer_type_tests!` macro should be used to generate tests for any type
    /// implementing this trait.
    ///
    /// [zst]:  https://doc.rust-lang.org/nomicon/exotic-sizes.html#zero-sized-types-zsts
    ///
    /// # Ownable Types
    ///
    /// This trait broadly covers all types that can be converted from a pointer to a
    /// reference, but some of the types (well, all of them except `SchemaObject`) can
    /// also be owned in addition to being borrowed. For these, see the `owned` module
    /// and its corresponding `Ownable` trait which extends `DataPointer` with
    /// additional functionality for creating and destroying owned instances of the
    /// type.
    ///
    /// # Extern Types
    ///
    /// There is an [accepted RFC][rfc] for more accurately representing types like
    /// this. If it is ever implemented and stabilized, we should switch to representing
    /// these types as extern types, as it would better enforce necessary safety
    /// invariants.
    ///
    /// [rfc]: https://github.com/rust-lang/rfcs/blob/master/text/1861-extern-types.md
    pub unsafe trait DataPointer: Sized {
        type Raw;

        unsafe fn from_raw<'a>(raw: *const Self::Raw) -> &'a Self {
            &*(raw as *const _)
        }

        unsafe fn from_raw_mut<'a>(raw: *mut Self::Raw) -> &'a mut Self {
            &mut *(raw as *mut _)
        }

        fn as_ptr(&self) -> *const Self::Raw {
            self as *const _ as *const _
        }

        fn as_ptr_mut(&mut self) -> *mut Self::Raw {
            self as *mut _ as *mut _
        }
    }

    /// A data pointer type that can be owned directly by user code.
    pub unsafe trait OwnedPointer: DataPointer {
        const CREATE_FN: unsafe extern "C" fn() -> *mut Self::Raw;
        const DESTROY_FN: unsafe extern "C" fn(*mut Self::Raw);
    }
}
