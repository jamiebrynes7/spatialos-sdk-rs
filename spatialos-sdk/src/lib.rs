extern crate spatialos_sdk_sys;

#[macro_export]
macro_rules! dummy_component {
    ($component:ident, $update:ident) => {
        impl $crate::worker::schema::SchemaObjectType for $component {
            fn from_object(_: &$crate::worker::schema::Object) -> Self {
                unimplemented!()
            }

            fn into_object(&self, _: &mut $crate::worker::schema::Object) {
                unimplemented!();
            }
        }

        impl $crate::worker::component::Component for $component {
            const ID: $crate::worker::component::ComponentId = 1234;
            type Update = $update;
        }

        inventory::submit!($crate::worker::component::VTable::new::<
            $component,
        >());

        pub struct $update;

        impl $crate::worker::component::Update for $update {
            type Component = $component;
        }
    };
}

pub(crate) mod ptr;
pub mod worker;
