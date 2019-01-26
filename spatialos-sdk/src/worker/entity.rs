use crate::worker::component::{self, Component, ComponentId};
use spatialos_sdk_sys::worker::Worker_ComponentData;
use std::collections::HashSet;
use std::ptr;

#[derive(Debug, Default)]
pub struct Entity {
    component_ids: HashSet<ComponentId>,
    pub(crate) component_data: Vec<Worker_ComponentData>,
    drops: Vec<unsafe fn(*mut std::ffi::c_void)>,
}

impl Entity {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<C: Component>(&mut self, component: C) {
        assert!(
            !self.component_ids.contains(&C::component_id()),
            "Duplicate component added to `Entity`"
        );

        let data_ptr = component::handle_allocate(component);
        let component_data = Worker_ComponentData {
            reserved: ptr::null_mut(),
            component_id: C::component_id(),
            schema_type: ptr::null_mut(),
            user_handle: data_ptr as *mut _,
        };

        self.component_ids.insert(C::component_id());
        self.component_data.push(component_data);
        self.drops.push(drop_raw::<C>);
    }
}

impl Drop for Entity {
    fn drop(&mut self) {
        for (raw_data, drop) in self.component_data.iter().zip(self.drops.iter()) {
            unsafe {
                drop(raw_data.user_handle);
            }
        }
    }
}

unsafe fn drop_raw<T>(raw: *mut std::ffi::c_void) {
    component::handle_free::<T>(raw);
}
