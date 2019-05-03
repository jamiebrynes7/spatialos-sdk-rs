use crate::worker::{
    component::{Component, ComponentDataRef, ComponentId, DATABASE},
    handle::UserHandle,
    schema::{self, owned::Owned},
};
use maybe_owned::MaybeOwned;
use spatialos_sdk_sys::worker::Worker_ComponentData;
use spatialos_sdk_sys::worker::Worker_Entity;
use std::{collections::HashMap, ptr, slice};

#[derive(Debug)]
enum ComponentData {
    SchemaData(Owned<schema::ComponentData>),
    UserHandle(UserHandle),
}

/// A collection of entities
#[derive(Debug, Default)]
pub struct Entity {
    components: HashMap<ComponentId, ComponentData>,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            components: Default::default(),
        }
    }

    pub fn add<C: Component + Send>(&mut self, component: C) -> Result<(), String> {
        // TODO: Actually do something to determine if we should add it as a handle or
        // serialize it immediately.
        self.add_handle(component)
    }

    /// Adds `component` without serializing it,
    pub fn add_handle<C: Component + Send>(&mut self, component: C) -> Result<(), String> {
        if !DATABASE.has_vtable(C::ID) {
            panic!(
                "Cannot add component (ID {}) as a handle because it does not have a vtable setup",
                C::ID
            );
        }

        self.pre_add_check(C::ID)?;

        self.components
            .insert(C::ID, ComponentData::UserHandle(UserHandle::new(component)));

        Ok(())
    }

    pub fn add_serialized<C: Component>(&mut self, component: &C) -> Result<(), String> {
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
    ///
    /// Additionally, this function returns any user handles that are owned by the
    /// entity. The returned `Worker_ComponentData` objects borrow data from the user
    /// handles, and so the user handles must remain in scope until the component data
    /// has been passed to the C API. At that point, the C API will have had a chance
    /// to clone the handles, and so it is safe to drop the returned handles.
    pub(crate) fn into_raw(mut self) -> (Vec<Worker_ComponentData>, Vec<UserHandle>) {
        let mut components = Vec::with_capacity(self.components.len());
        let mut handles = Vec::with_capacity(self.components.len());

        for (id, component_data) in self.components.drain() {
            match component_data {
                ComponentData::SchemaData(schema_data) => components.push(Worker_ComponentData {
                    reserved: ptr::null_mut(),
                    component_id: id,
                    schema_type: schema_data.into_raw(),
                    user_handle: ptr::null_mut(),
                }),

                ComponentData::UserHandle(handle) => {
                    components.push(Worker_ComponentData {
                        reserved: ptr::null_mut(),
                        component_id: id,
                        schema_type: ptr::null_mut(),
                        user_handle: handle.raw(),
                    });
                    handles.push(handle);
                }
            }
        }

        (components, handles)
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
        self.components.get(&C::ID).and_then(ComponentDataRef::get)
    }
}

#[cfg(test)]
mod test {
    use crate::worker::{component::inventory, entity::Entity};
    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    };

    pub struct TestComponent(Arc<AtomicBool>);
    dummy_component!(TestComponent, TestComponentUpdate);

    impl Drop for TestComponent {
        fn drop(&mut self) {
            self.0.store(true, Ordering::SeqCst);
        }
    }

    #[test]
    fn free_handle_on_drop_entity() {
        let was_dropped = Arc::new(AtomicBool::new(false));

        {
            let mut entity = Entity::new();
            let _ = entity.add_handle(TestComponent(was_dropped.clone()));
        }

        assert!(
            was_dropped.load(Ordering::SeqCst),
            "Component handle wasn't dropped"
        );
    }

    #[test]
    fn free_handle_on_drop_entity_into_raw() {
        let was_dropped = Arc::new(AtomicBool::new(false));

        {
            let mut entity = Entity::new();
            let _ = entity.add_handle(TestComponent(was_dropped.clone()));
            let _ = entity.into_raw();
        }

        assert!(
            was_dropped.load(Ordering::SeqCst),
            "Component handle wasn't dropped"
        );
    }
}
