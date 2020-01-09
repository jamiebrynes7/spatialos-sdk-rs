//! Handling for schema data types that act like pointers.
//!
//! `Schema_Object` and the related schema data "wrapper types" are all represented
//! as opaque types hidden behind pointers in the C API; You always work with a
//! pointer to the struct, never an instance of the struct itself. To represent
//! these in Rust, we define dummy types that can similarly always be put behind a
//! reference without ever allowing the user to directly "hold" an instance of the
//! type. This allows us to simply cast the raw pointer to a reference to our dummy
//! type and preserve the desired semantics of working with a reference, including
//! the ownership rules and lifetimes that come with it.
//!
//! The `DataPointer` trait is implemented for all such types, and provides the
//! common methods used for type punning with the raw pointer.
//!
//! # Safety
//!
//! In order for the implementation of `DataPointer` to be safe, the following
//! invariants must be upheld:
//!
//! * It must be a [zero sized type][zst].
//! * It must have a private field, such that the type can never be directly
//!   instantiated by a user.
//! * It must never be instantiated directly within the SDK. Instances may only ever
//!   be created by casting the raw pointers returned from the appropriate C API
//!   functions.
//! * A reference to `Self` must never be de-referenced. It may only ever be
//!   converted to a pointer with `as_ptr()`, which may then be passed to the
//!   appropriate functions in the C API.
//! * The type may be `Send`, but it must not be `Sync`.
//!
//! The `pointer_type_tests!` macro should be used to generate tests for any type
//! implementing this trait.
//!
//! # Extern Types
//!
//! There is an [accepted RFC][rfc] for more accurately representing types like
//! this. If it is ever implemented and stabilized, we should switch to representing
//! these types as extern types, as it would better enforce necessary safety
//! invariants.
//!
//! Until then, enforcing that the types are zero-sized and have a dummy private
//! field is the closest we can come to ensuring the type is safe to use. We must
//! also be sure to never directly instantiate the type *within* the SDK.
//!
//! # Interior Mutability and Thread Safety
//!
//! The `as_ptr_mut` method requires `&mut self` in order to get a mutable raw
//! pointer to the underlying data, however there are some cases where the C API
//! function takes a `*mut T` where the corresponding Rust function makes more sense
//! taking `&self`. For example, `SchemaObject::get_object` takes `&self` and
//! returns a `&SchemaObject`, indicating it would be safe to call it multiple times
//! and get multiple simultaneous borrows of the same inner object. The underlying
//! C function `Schema_GetObject` takes a `*mut Schema_Object` because a dummy
//! object may need to be created if there is not already an object in the specified
//! field.
//!
//! This setup, where the underlying data may be mutated through an immutable
//! reference, is a form of [interior mutability] and is still safe as long as we
//! maintain the necessary invariants. In particular, our Rust types cannot be
//! `Sync` because the mutability happening within the C SDK is unsynchronized.
//! The other key invariant is that we must ensure that a reference returned by a
//! method taking `&self` can never be invalidated by another method taking `&self`,
//! however the C API already guarantees this behavior in all cases and so is not
//! something we specifically have to enforce within the Rust SDK.
//!
//! [zst]:  https://doc.rust-lang.org/nomicon/exotic-sizes.html#zero-sized-types-zsts
//! [rfc]: https://github.com/rust-lang/rfcs/blob/master/text/1861-extern-types.md
//! [interior mutability]: https://doc.rust-lang.org/book/ch15-05-interior-mutability.html

/// A type that acts as an alias to some schema data hidden behind a pointer.
///
/// See the module-level docs for more information.
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
///
/// All of the data pointer types except `SchemaObject` can be directly owned by the
/// user via the `Owned<T>` smart pointer type. This trait defines the constructor
/// and destructor functions for the underlying C data needed for this purpose.
pub unsafe trait OwnedPointer: DataPointer {
    const CREATE_FN: unsafe extern "C" fn() -> *mut Self::Raw;
    const DESTROY_FN: unsafe extern "C" fn(*mut Self::Raw);
    const COPY_FN: unsafe extern "C" fn(*const Self::Raw) -> *mut Self::Raw;
}
