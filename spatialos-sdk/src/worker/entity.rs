use crate::worker::{
    component::{self, Component, ComponentDataRef, ComponentId, UserHandle, DATABASE},
    schema,
};
use maybe_owned::MaybeOwned;
use spatialos_sdk_sys::worker::Worker_ComponentData;
use spatialos_sdk_sys::worker::Worker_Entity;
use std::{collections::HashMap, mem, ptr, slice};

#[derive(Debug)]
enum ComponentData<'a> {
    SchemaData(schema::ComponentData<'a>),
    UserHandle(UserHandle),
}

/// A collection of entities
#[derive(Debug, Default)]
pub struct Entity<'a> {
    components: HashMap<ComponentId, ComponentData<'a>>,
    use_internal_serialization: bool,
}

impl<'a> Entity<'a> {
    pub fn new(use_internal_serialization: bool) -> Self {
        Self {
            components: Default::default(),
            use_internal_serialization,
        }
    }

    /// Adds `component` without serializing it,
    pub fn add_handle<C: Component>(&mut self, component: C) -> Result<(), String> {
        if !DATABASE.has_vtable(C::ID) {
            panic!(
                "Cannot add component (ID {}) as a handle because it does not have a vtable setup",
                C::ID
            );
        }

        self.pre_add_check(C::ID)?;

        let user_handle = component::handle_allocate(component);
        self.components
            .insert(C::ID, ComponentData::UserHandle(user_handle));

        Ok(())
    }

    pub fn add_serialized<C: Component>(&mut self, component: &'a C) -> Result<(), String> {
        self.pre_add_check(C::ID)?;

        let schema_data = schema::ComponentData::new(component);
        self.components
            .insert(C::ID, ComponentData::SchemaData(schema_data));

        Ok(())
    }

    /// Converts the `Entity` into a list of raw `Worker_ComponentData` objects that can
    /// be passed to the C API.
    ///
    /// This transfers ownership of the component data to the caller, so the caller
    /// needs to ensure the appropriate steps are taken to free any allocated schema
    /// data or user handles. If the raw data is passed to the C API using
    /// `Worker_Connection_SendCreateEntityRequest`, the C API will take ownership of
    /// the data and will free it when it's done.
    pub(crate) fn into_raw(mut self) -> Vec<Worker_ComponentData> {
        self.components
            .drain()
            .map(|(id, component_data)| match component_data {
                ComponentData::SchemaData(schema_data) => Worker_ComponentData {
                    reserved: ptr::null_mut(),
                    component_id: id,
                    schema_type: schema_data.into_raw(),
                    user_handle: ptr::null_mut(),
                },

                ComponentData::UserHandle(handle) => Worker_ComponentData {
                    reserved: ptr::null_mut(),
                    component_id: id,
                    schema_type: ptr::null_mut(),
                    user_handle: handle,
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

        Ok(())
    }
}

impl<'a> Drop for Entity<'a> {
    fn drop(&mut self) {
        for (id, component_data) in self.components.drain() {
            match component_data {
                ComponentData::SchemaData(_) => {}

                ComponentData::UserHandle(user_handle) => {
                    let vtable = DATABASE.get_vtable(id).unwrap();

                    let free_data_func = vtable.component_data_free.unwrap_or_else(|| {
                        panic!("No component_data_free method defined for compnent {}", id)
                    });

                    unsafe {
                        free_data_func(id, ptr::null_mut(), user_handle);
                    }
                }
            }
        }
    }
}

/// Entity data returned from SpatialOS.
///
/// Presents a read-only view into entity data returned from the runtime.
#[derive(Debug)]
pub struct EntityQuery<'a> {
    components: HashMap<ComponentId, ComponentDataRef<'a>>,
}

impl<'a> EntityQuery<'a> {
    pub(crate) unsafe fn from_raw(raw_entity: &Worker_Entity) -> Self {
        let component_data =
            slice::from_raw_parts(raw_entity.components, raw_entity.component_count as usize);
        let components = component_data
            .iter()
            .map(|raw| {
                let id = raw.component_id;
                let component_data = ComponentDataRef::from_raw(raw);
                (id, component_data)
            })
            .collect();

        EntityQuery { components }
    }

    pub fn get<C: Component>(&self) -> Option<MaybeOwned<'a, C>> {
        self.components.get(&C::ID).and_then(|data| data.get())
    }
}
