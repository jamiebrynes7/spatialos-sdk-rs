use crate::worker::{
    component::{Component, ComponentId},
    schema,
    schema::*,
};
use spatialos_sdk_sys::worker::{Worker_ComponentData, Worker_Entity};
use std::collections::HashMap;
use std::result::Result;
use std::slice;

#[derive(Debug, Default, Clone)]
pub struct Entity {
    components: HashMap<ComponentId, Owned<SchemaComponentData>>,
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
            entity.pre_add_check(data.component_id)?;

            entity.components.insert(
                data.component_id,
                SchemaComponentData::from_raw(data.schema_type).to_owned(),
            );
        }

        Ok(entity)
    }

    pub(crate) fn add<C: Component>(&mut self, component: &C) -> Result<(), String> {
        self.pre_add_check(C::ID)?;

        self.components
            .insert(C::ID, SchemaComponentData::from_component(component));

        Ok(())
    }

    pub(crate) unsafe fn add_serialized(
        &mut self,
        component_id: ComponentId,
        component: Owned<SchemaComponentData>,
    ) -> Result<(), String> {
        self.pre_add_check(component_id)?;

        self.components.insert(component_id, component);

        Ok(())
    }

    pub fn get<C: Component>(&self) -> Option<schema::Result<C>> {
        self.components.get(&C::ID).map(|data| data.deserialize())
    }

    pub(crate) fn raw_component_data(&mut self) -> Vec<Worker_ComponentData> {
        self.components
            .iter_mut()
            .map(|(&component_id, data)| Worker_ComponentData {
                component_id,

                // TODO: Why does this require a `*mut Schema_ComponentData`? Is there any actual
                // chance that the underlying data will be mutated? Would it be safe for us to use
                // `as_ptr` and cast it to a `*mut`?
                schema_type: data.as_ptr_mut(),

                ..Default::default()
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

        Ok(())
    }
}
