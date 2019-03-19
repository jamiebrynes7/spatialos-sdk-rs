use crate::worker::component::{self, Component, ComponentId, DATABASE};
use crate::worker::internal::schema::SchemaComponentData;
use spatialos_sdk_sys::worker::Worker_ComponentData;
use spatialos_sdk_sys::worker::Worker_Entity;
use std::collections::HashMap;
use std::ptr;
use std::slice;

#[derive(Debug)]
pub struct Entity {
    components: HashMap<ComponentId, Worker_ComponentData>,
}

impl Entity {
    pub fn new() -> Self {
        Entity::default()
    }

    pub(crate) unsafe fn from_worker_sdk(raw_entity: &Worker_Entity) -> Result<Self, String> {
        let mut entity = Entity::new();

        let component_data =
            slice::from_raw_parts(raw_entity.components, raw_entity.component_count as usize);

        for data in component_data {
            entity.add_raw(data)?;
        }

        Ok(entity)
    }

    pub fn add<C: Component>(&mut self, component: C) -> Result<(), String> {
        self.pre_add_check(C::ID)?;

        let data_ptr = component::handle_allocate(component);
        let raw_data = Worker_ComponentData {
            reserved: ptr::null_mut(),
            component_id: C::ID,
            schema_type: ptr::null_mut(),
            user_handle: data_ptr as *mut _,
        };

        self.components.insert(C::ID, raw_data);

        Ok(())
    }

    pub(crate) unsafe fn add_raw(
        &mut self,
        component: &Worker_ComponentData,
    ) -> Result<(), String> {
        let id = component.component_id;

        self.pre_add_check(id)?;

        // Call copy on the component data. We don't own this Worker_ComponentData.
        let vtable = DATABASE.get_vtable(id).unwrap();
        let copy_data_func = vtable
            .component_data_copy
            .unwrap_or_else(|| panic!("No component_data_free method defined for {}", id));
        copy_data_func(id, ptr::null_mut(), component.user_handle);

        self.components.insert(
            id,
            Worker_ComponentData {
                reserved: ptr::null_mut(),
                component_id: id,
                schema_type: ptr::null_mut(),
                user_handle: component.user_handle,
            },
        );

        Ok(())
    }

    pub(crate) unsafe fn add_serialized(
        &mut self,
        component: SchemaComponentData,
    ) -> Result<(), String> {

        self.pre_add_check(component.component_id)?;

        let vtable = self.database.get_vtable(component.component_id).unwrap();
        let deserialize_func = vtable.component_data_deserialize
            .unwrap_or_else(|| panic!("No component_data_deserialize method define for {}", component.component_id));

        let deserialized_data_ptr = Box::into_raw(Box::new(0)) as *mut ::std::os::raw::c_void;
        let handle_out_ptr = Box::into_raw(Box::new(deserialized_data_ptr));

        match deserialize_func(component.component_id, ptr::null_mut(), component.internal, handle_out_ptr) {
            1 => {},
            0 => return Err("Error deserializing manually serialized data. Is the SchemaComponentData malformed?".to_owned()),
            _ => panic!("Unexpected return value from deserialize function. Expected true or false. Received other.")
        };

        let component_data = Worker_ComponentData {
            reserved: ptr::null_mut(),
            component_id: component.component_id,
            schema_type: ptr::null_mut(),
            user_handle: *handle_out_ptr
        };

        self.add_raw(&component_data)
    }

    pub fn get<C: Component>(&self) -> Option<&C> {
        self.components
            .get(&C::ID)
            .map(|data| unsafe { &*(data.user_handle as *const _) })
    }

    pub(crate) fn raw_component_data(&self) -> RawEntity {
        RawEntity::new(self.components.values())
    }

    fn pre_add_check(&self, id: ComponentId) -> Result<(), String> {
        if self.components.contains_key(&id) {
            return Err(format!(
                "Duplicate component with ID {} added to `Entity`.",
                id
            ));
        }

        if !DATABASE.has_vtable(id) {
            panic!(format!(
                "Could not find a vtable implementation for component {}",
                id
            ));
        }

        Ok(())
    }
}

impl Default for Entity {
    fn default() -> Self {
        Entity {
            components: HashMap::new(),
        }
    }
}

impl Drop for Entity {
    fn drop(&mut self) {
        for component_data in self.components.values() {
            let id = component_data.component_id;

            let vtable = DATABASE.get_vtable(id).unwrap();

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
}

impl RawEntity {
    pub fn new<'a, I>(original_data: I) -> Self
    where
        I: Iterator<Item = &'a Worker_ComponentData>,
    {
        // Go through each Worker_ComponentData object, make a copy and call handle_copy using the vtable.
        let new_data = original_data
            .map(|original_component_data| {
                let new_component_data = *original_component_data; // Is a copy operation.
                let id = original_component_data.component_id;

                let vtable = DATABASE.get_vtable(id).unwrap();

                let copy_data_func = vtable
                    .component_data_copy
                    .unwrap_or_else(|| panic!("No component_data_copy method defined for {}", id));

                unsafe { copy_data_func(id, ptr::null_mut(), original_component_data.user_handle) };

                new_component_data
            })
            .collect();

        RawEntity {
            components: new_data,
        }
    }
}

impl Drop for RawEntity {
    fn drop(&mut self) {
        for component_data in &self.components {
            let vtable = DATABASE.get_vtable(component_data.component_id).unwrap();

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
