#[cfg(test)]
macro_rules! pointer_type_tests {
    ($type:ty) => {
        static_assertions::const_assert!(std::mem::size_of::<$type>() == 0);

        static_assertions::assert_eq_align!(
            *mut <$type as $crate::worker::schema::private::PointerType>::Raw,
            &$type,
            &mut $type,
        );

        static_assertions::assert_impl_all!($type: Send);
        static_assertions::assert_not_impl_any!($type: Sync);
    };
}
