use crate::{
    worker::component::ComponentDatabase,
    worker::component::{self, Component, ComponentId},
};
use spatialos_sdk_sys::worker::Worker_ComponentData;
use std::collections::HashMap;
use std::ptr;

#[derive(Debug, Default)]
pub struct Entity {
    components: HashMap<ComponentId, Worker_ComponentData>,
    database: ComponentDatabase,
}

impl Entity {
    pub fn new() -> Self {
        Entity::default()
    }

    pub fn add<C: Component>(&mut self, component: C) {
        assert!(
            !self.components.contains_key(&C::ID),
            "Duplicate component added to `Entity`"
        );

        assert!(
            !self.database.has_vtable(C::ID),
            format!(
                "Could not find a vtable implementation for component {}",
                C::ID
            )
        );

        let data_ptr = component::handle_allocate(component);
        let raw_data = Worker_ComponentData {
            reserved: ptr::null_mut(),
            component_id: C::ID,
            schema_type: ptr::null_mut(),
            user_handle: data_ptr as *mut _,
        };

        self.components.insert(C::ID, raw_data);
    }

    pub(crate) fn add_raw(&mut self, component: &Worker_ComponentData) {
        let id = component.component_id;

        assert!(
            !self.components.contains_key(&id),
            "Duplicate component added to `Entity`"
        );

        assert!(
            !self.database.has_vtable(id),
            format!(
                "Could not find a vtable implementation for component {}",
                id
            )
        );

        // Call copy on the component data. We don't own this Worker_ComponentData.
        let vtable = self.database.get_vtable(id).unwrap();
        let copy_data_func = vtable
            .component_data_copy
            .unwrap_or_else(|| panic!("No component_data_free method defined for {}", id));
        unsafe { copy_data_func(id, ptr::null_mut(), component.user_handle) };

        self.components.insert(
            id,
            Worker_ComponentData {
                reserved: ptr::null_mut(),
                component_id: id,
                schema_type: ptr::null_mut(),
                user_handle: component.user_handle,
            },
        );
    }

    pub fn get<C: Component>(&self) -> Option<&C> {
        self.components
            .get(&C::ID)
            .map(|data| unsafe { &*(data.user_handle as *const _) })
    }

    pub(crate) fn raw_component_data(&self) -> RawEntity {
        RawEntity::new(self.components.values())
    }
}

impl Drop for Entity {
    fn drop(&mut self) {
        for component_data in self.components.values() {
            let id = component_data.component_id;

            let vtable = self.database.get_vtable(id).unwrap();

            let free_data_func = vtable
                .component_data_free
                .unwrap_or_else(|| panic!("No component_data_free method defined for {}", id));

            unsafe { free_data_func(id, ptr::null_mut(), component_data.user_handle) };
        }
    }
}

// Required for when we call Entity::raw_component_data() and want a Vec<Worker_ComponentData> rather
// than a Vec<&Worker_ComponentData> which most callers *will* want due to how Worker_Entity is structured.
pub(crate) struct RawEntity {
    pub components: Vec<Worker_ComponentData>,
    database: ComponentDatabase,
}

impl RawEntity {
    pub fn new<'a, I>(original_data: I) -> Self
    where
        I: Iterator<Item = &'a Worker_ComponentData>,
    {
        let database = ComponentDatabase::new();

        // Go through each Worker_ComponentData object, make a copy and call handle_copy using the vtable.
        let new_data = original_data
            .map(|original_component_data| {
                let new_component_data = *original_component_data; // Is a copy operation.
                let id = original_component_data.component_id;

                let vtable = database.get_vtable(id).unwrap();

                let copy_data_func = vtable
                    .component_data_copy
                    .unwrap_or_else(|| panic!("No component_data_copy method defined for {}", id));

                unsafe { copy_data_func(id, ptr::null_mut(), original_component_data.user_handle) };

                new_component_data
            })
            .collect();

        RawEntity {
            components: new_data,
            database,
        }
    }
}

impl Drop for RawEntity {
    fn drop(&mut self) {
        for component_data in &self.components {
            let vtable = self
                .database
                .get_vtable(component_data.component_id)
                .unwrap();

            let free_data_func = vtable.component_data_free.unwrap_or_else(|| {
                panic!(
                    "No component_data_free method defined for {}",
                    component_data.component_id
                )
            });

            unsafe {
                free_data_func(
                    component_data.component_id,
                    ptr::null_mut(),
                    component_data.user_handle,
                )
            };
        }
    }
}
