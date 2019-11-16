/// Generates static assertion tests around safety invariants for pointer types.
///
/// Generates the following tests:
///
/// * Verify that the type is zero sized.
/// * Verify that the alignment of the raw pointer matches the alignment of
///   references to the Rust type.
/// * Verify that the type implements `Send`.
/// * Verify that the type does not implement `Sync`.
///
/// See the documentation for `DataPointer` for more details about the safety
/// invariants that these tests enforce.
macro_rules! pointer_type_tests {
    ($type:ty) => {
        static_assertions::const_assert!(std::mem::size_of::<$type>() == 0);

        static_assertions::assert_eq_align!(
            *mut <$type as $crate::worker::schema::private::DataPointer>::Raw,
            &$type,
            &mut $type,
        );

        static_assertions::assert_impl_all!($type: Send);
        static_assertions::assert_not_impl_any!($type: Sync);
    };
}
