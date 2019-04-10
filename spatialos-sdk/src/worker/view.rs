use crate::worker::Authority::Authoritative;
use crate::worker::{
    component::internal::ComponentData,
    component::{Component, ComponentId, DATABASE},
    op::*,
    Authority, EntityId,
};
use spatialos_sdk_sys::{
    worker::Worker_AcquireComponentData, worker::Worker_ComponentData,
    worker::Worker_ReleaseComponentData,
};
use std::{
    collections::{hash_map::HashMap, HashSet},
    ops::Deref,
};

pub struct View {
    data: HashMap<ComponentId, HashMap<EntityId, OwnedComponentData>>,
    authority: HashMap<ComponentId, HashMap<EntityId, Authority>>,
    entities: HashSet<EntityId>,
}

impl View {
    pub fn new() -> Self {
        let mut view = View {
            data: HashMap::new(),
            authority: HashMap::new(),
            entities: HashSet::new(),
        };

        for id in DATABASE.get_registered_component_ids() {
            view.data.insert(id, HashMap::new());
            view.authority.insert(id, HashMap::new());
        }

        view
    }

    pub fn process_ops(&mut self, ops: &OpList) {
        for op in ops.iter() {
            match op {
                WorkerOp::AddEntity(op) => self.add_entity(op.entity_id),
                WorkerOp::RemoveEntity(op) => self.remove_entity(op.entity_id),
                WorkerOp::AddComponent(op) => self.add_component(&op),
                WorkerOp::RemoveComponent(op) => self.remove_component(&op),
                WorkerOp::ComponentUpdate(op) => self.handle_component_update(&op),
                WorkerOp::AuthorityChange(op) => self.set_authority(&op),
                _ => {}
            }
        }
    }

    pub fn get_component<C: Component>(&self, entity_id: &EntityId) -> Option<&C> {
        self.data
            .get(&C::ID)
            .unwrap()
            .get(entity_id)
            .map(|data| unsafe { (&*((*data).user_handle as *const C)) })
    }

    pub fn get_authority<C: Component>(&self, entity_id: &EntityId) -> Option<Authority> {
        self.authority
            .get(&C::ID)
            .unwrap()
            .get(entity_id)
            .map(|data| data.clone())
    }

    pub fn is_authoritative<C: Component>(&self, entity_id: &EntityId) -> bool {
        self.get_authority::<C>(entity_id)
            .map_or(false, |auth| auth == Authoritative)
    }

    pub fn iter_entities(&self) -> ViewEntityIterator {
        ViewEntityIterator::new(self)
    }

    pub fn has_entity(&self, entity_id: &EntityId) -> bool {
        self.entities.contains(entity_id)
    }

    pub fn query<'a, T : ViewQuery<'a, T> + 'a>(&'a self) -> (impl Iterator<Item = T> + 'a) {
        self.iter_entities().filter(move |id| T::filter(self, id))
            .map(move |id| T::select(self, id.clone()))
    }

    fn add_entity(&mut self, entity_id: EntityId) {
        self.entities.insert(entity_id);
    }

    fn remove_entity(&mut self, entity_id: EntityId) {
        self.entities.remove(&entity_id);
    }

    fn add_component(&mut self, op: &AddComponentOp) {
        let data = OwnedComponentData::from(op.data());
        self.data
            .get_mut(&op.component_id)
            .unwrap()
            .insert(op.entity_id, data);
    }

    fn remove_component(&mut self, op: &RemoveComponentOp) {
        let mut _data = self
            .data
            .get_mut(&op.component_id)
            .unwrap()
            .remove(&op.entity_id)
            .unwrap();

        self.authority
            .get_mut(&op.component_id)
            .unwrap()
            .remove(&op.entity_id);
    }

    fn set_authority(&mut self, op: &AuthorityChangeOp) {
        self.authority
            .get_mut(&op.component_id)
            .unwrap()
            .insert(op.entity_id, op.authority);
    }

    fn handle_component_update(&mut self, op: &ComponentUpdateOp) {
        let merge = DATABASE.get_merge_method(op.component_id).unwrap();
        let data = self
            .data
            .get_mut(&op.component_id)
            .unwrap()
            .get_mut(&op.entity_id)
            .unwrap();
        unsafe { merge(data.user_handle, op.component_update.user_handle) };
    }
}

pub struct ViewEntityIterator<'a> {
    iter: ::std::collections::hash_set::Iter<'a, EntityId>,
    view: &'a View,
}

impl<'a> ViewEntityIterator<'a> {
    fn new(view: &'a View) -> Self {
        ViewEntityIterator {
            iter: view.entities.iter(),
            view,
        }
    }
}

pub trait ViewQuery<'a, T> {
    fn filter(view: &View, entity_id: &EntityId) -> bool;
    fn select(view: &'a View, entity_id: EntityId) -> T;
}

impl<'a> Iterator for ViewEntityIterator<'a> {
    type Item = &'a EntityId;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

struct OwnedComponentData {
    data: Worker_ComponentData,
}

impl<'a> From<&'a ComponentData<'a>> for OwnedComponentData {
    fn from(data: &ComponentData) -> Self {
        let mut internal = Worker_ComponentData {
            reserved: data.reserved,
            component_id: data.component_id,
            schema_type: data.schema_type.internal,
            user_handle: data.user_handle as *mut _,
        };

        OwnedComponentData {
            data: unsafe { *Worker_AcquireComponentData(&mut internal) },
        }
    }
}

impl Deref for OwnedComponentData {
    type Target = Worker_ComponentData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl Drop for OwnedComponentData {
    fn drop(&mut self) {
        unsafe { Worker_ReleaseComponentData(&mut self.data) };
    }
}
