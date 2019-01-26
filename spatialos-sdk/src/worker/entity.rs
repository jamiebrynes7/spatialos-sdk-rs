use crate::worker::component::{self, Component, ComponentId};
use spatialos_sdk_sys::worker::Worker_ComponentData;
use std::collections::HashMap;
use std::ptr;

#[derive(Debug)]
struct ComponentData {
    raw_data: Worker_ComponentData,

    // Type-erased version of the drop impl for the component type. Given the raw
    // pointer to the component data, it'll reconstruct the handle and free it.
    drop_fn: unsafe fn(*mut std::ffi::c_void),
}

#[derive(Debug, Default)]
pub struct Entity {
    components: HashMap<ComponentId, ComponentData>,
}

impl Entity {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<C: Component>(&mut self, component: C) {
        assert!(
            !self.components.contains_key(&C::ID),
            "Duplicate component added to `Entity`"
        );

        let data_ptr = component::handle_allocate(component);
        let raw_data = Worker_ComponentData {
            reserved: ptr::null_mut(),
            component_id: C::ID,
            schema_type: ptr::null_mut(),
            user_handle: data_ptr as *mut _,
        };

        self.components.insert(
            C::ID,
            ComponentData {
                raw_data,
                drop_fn: drop_raw::<C>,
            },
        );
    }

    pub fn get<C: Component>(&self) -> Option<&C> {
        self.components
            .get(&C::ID)
            .map(|data| unsafe { &*(data.raw_data.user_handle as *const _) })
    }

    pub(crate) fn raw_component_data(&self) -> Vec<Worker_ComponentData> {
        self.components.values().map(|data| data.raw_data).collect()
    }
}

impl Drop for Entity {
    fn drop(&mut self) {
        for data in self.components.values() {
            unsafe {
                (data.drop_fn)(data.raw_data.user_handle);
            }
        }
    }
}

unsafe fn drop_raw<T>(raw: *mut std::ffi::c_void) {
    component::handle_free::<T>(raw);
}
