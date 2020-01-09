use crate::worker::{
    component::{Component, ComponentId},
    handle,
    handle::UserHandle,
    schema::*,
    vtable::DATABASE,
};
use spatialos_sdk_sys::worker::Worker_ComponentData;
use std::{collections::HashMap, ptr, result::Result};

#[derive(Debug, Clone)]
enum Data {
    SchemaData(Owned<SchemaComponentData>),
    UserHandle(UserHandle),
}

#[derive(Debug, Default, Clone)]
pub struct Entity {
    components: HashMap<ComponentId, Data>,
}

impl Entity {
    pub fn new() -> Self {
        Entity::default()
    }

    pub(crate) fn add<C: Component>(&mut self, component: C) -> Result<(), String> {
        self.pre_add_check(C::ID)?;

        let handle = handle::new(Ok(component));
        self.components.insert(C::ID, Data::UserHandle(handle));

        Ok(())
    }

    pub(crate) unsafe fn add_serialized(
        &mut self,
        component_id: ComponentId,
        component: Owned<SchemaComponentData>,
    ) -> Result<(), String> {
        self.pre_add_check(component_id)?;

        self.components
            .insert(component_id, Data::SchemaData(component));

        Ok(())
    }

    pub(crate) fn as_raw(&mut self) -> Vec<Worker_ComponentData> {
        self.components
            .iter_mut()
            .map(|(&component_id, data)| match data {
                Data::UserHandle(handle) => Worker_ComponentData {
                    reserved: ptr::null_mut(),
                    component_id,
                    schema_type: ptr::null_mut(),
                    user_handle: handle::as_raw(handle),
                },

                Data::SchemaData(schema_data) => Worker_ComponentData {
                    reserved: ptr::null_mut(),
                    component_id,
                    schema_type: schema_data.as_ptr_mut(),
                    user_handle: ptr::null_mut(),
                },
            })
            .collect()
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
