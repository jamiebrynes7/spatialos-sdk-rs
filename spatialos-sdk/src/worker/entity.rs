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

    pub(crate) fn from_schema_field(object: &SchemaObject) -> Result<Self, schema::Error> {
        let field_ids = object.unique_field_ids();
        let mut entity = Entity::new();

        for field_id in field_ids {
            entity
                .pre_add_check(field_id)
                .map_err(Error::schema_error::<Self>)?;

            let component = object.get_object(field_id);
            let mut owned = SchemaComponentData::new();
            owned
                .fields_mut()
                .copy_from(component)
                .map_err(Error::schema_error::<Self>)?;

            entity.components.insert(field_id, owned);
        }

        Ok(entity)
    }

    pub(crate) fn to_schema_field(&self, object: &mut SchemaObject) {
        for (component_id, data) in &self.components {
            let component_field = object.add_object(*component_id);
            component_field.copy_from(data.fields()).unwrap(); // TODO: Get rid of unwrap.
        }
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

    /// Converts the entity's contents into a format that can be used with the C API.
    ///
    /// In cases where the C API takes an entity as a parameter, it does so by taking an array of
    /// `Worker_ComponentData` structs. This method gathers the serialized component data and
    /// returns a `Vec<Worker_ComponentData>` that can be used to pass data to the C API.
    ///
    /// This method takes ownership of the data owned by the `Entity` so that it may be passed
    /// to the C API. It is expected that ownership of the data will be taken by the C SDK and that
    /// the C SKD will free any allocated data. For example, `Worker_Connection_SendCreateEntityRequest`
    /// will take ownership of the underlying `Schema_ComponentData` and will free it when
    /// appropriate. If the data is not passed to a similar function in the C SDK, then the data
    /// owned by the `Entity` will leak.
    pub(crate) fn into_raw(mut self) -> Vec<Worker_ComponentData> {
        self.components
            .drain()
            .map(|(component_id, data)| Worker_ComponentData {
                component_id,
                schema_type: data.into_raw(),

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
