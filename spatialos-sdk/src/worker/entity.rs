use crate::worker::{
    component::{Component, ComponentId},
    handle,
    handle::UserHandle,
    schema::*,
    vtable::DATABASE,
};
use spatialos_sdk_sys::worker::Worker_ComponentData;
use std::collections::HashMap;
use std::result::Result;

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

    pub fn get<C: Component>(&self) -> Option<&C> {
        self.components.get(&C::ID).map(|data| match data {
            Data::UserHandle(handle) => unimplemented!(),
            Data::SchemaData(schema_data) => unimplemented!(),
        })
    }

    pub(crate) fn raw_component_data(&self) -> Vec<Worker_ComponentData> {
        self.components
            .iter()
            .map(|(component_id, data)| match data {
                Data::UserHandle(handle) => unimplemented!(),
                Data::SchemaData(schema_data) => unimplemented!(),
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
