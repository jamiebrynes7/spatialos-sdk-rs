use crate::worker::{component::Component, component::ComponentId, entity::Entity};
use std::collections::{HashMap, HashSet};

const ENTITY_ACL_COMPONENT_ID: ComponentId = 50;
const METADATA_COMPONENT_ID: ComponentId = 53;
const POSITION_COMPONENT_ID: ComponentId = 54;
const PERSISTENCE_COMPONENT_ID: ComponentId = 55;

pub struct EntityBuilder {
    entity: Entity,

    _position: (f64, f64, f64),
    is_persistent: bool,
    metadata: Option<String>,

    write_permissions: HashMap<ComponentId, String>,
    read_permissions: HashSet<String>,

    error: Option<String>,
}

impl EntityBuilder {
    pub fn new<T: Into<String>>(x: f64, y: f64, z: f64, position_write_layer: T) -> Self {
        let mut builder = EntityBuilder {
            entity: Entity::new(),
            is_persistent: false,
            metadata: None,
            _position: (x, y, z),
            write_permissions: HashMap::new(),
            read_permissions: HashSet::new(),
            error: None,
        };

        builder.add_write_access(POSITION_COMPONENT_ID, position_write_layer);
        builder
    }

    pub fn add<C: Component + Send, T: Into<String>>(
        &mut self,
        data: C,
        write_layer: T,
    ) -> Result<(), String> {
        self.entity.add(data)?;
        self.add_write_access(C::ID, write_layer);

        Ok(())
    }

    pub fn add_serialized<C: Component, T: Into<String>>(
        &mut self,
        data: &C,
        write_layer: T,
    ) -> Result<(), String> {
        self.entity.add_serialized(data)?;
        self.add_write_access(C::ID, write_layer);

        Ok(())
    }

    pub fn add_handle<C: Component + Send, T: Into<String>>(
        &mut self,
        data: C,
        write_layer: T,
    ) -> Result<(), String> {
        self.entity.add_handle(data)?;
        self.add_write_access(C::ID, write_layer);

        Ok(())
    }

    pub fn set_persistent<T: Into<String>>(&mut self, write_layer: T) {
        self.is_persistent = true;
        self.add_write_access(PERSISTENCE_COMPONENT_ID, write_layer);
    }

    pub fn set_metadata<T: Into<String>, U: Into<String>>(
        &mut self,
        entity_type: T,
        write_layer: U,
    ) {
        self.metadata = Some(entity_type.into());
        self.add_write_access(METADATA_COMPONENT_ID, write_layer);
    }

    pub fn add_read_access<T: Into<String>>(&mut self, layer: T) {
        self.read_permissions.insert(layer.into());
    }

    pub fn set_entity_acl_write_access<T: Into<String>>(&mut self, layer: T) {
        self.add_write_access(ENTITY_ACL_COMPONENT_ID, layer);
    }

    fn add_write_access<T: Into<String>>(&mut self, id: ComponentId, layer: T) {
        let layer = layer.into();
        self.write_permissions.insert(id, layer.clone());
        self.read_permissions.insert(layer);
    }

    pub fn build(self) -> Result<Entity, String> {
        if let Some(e) = self.error {
            return Err(e);
        }

        unimplemented!("TODO: Add the various automatic components");
        // unsafe { self.entity.add_serialized(self.serialize_position())? };
        // unsafe { self.entity.add_serialized(self.serialize_acl())? };

        // if self.metadata.is_some() {
        //     unsafe { self.entity.add_serialized(self.serialize_metadata())? }
        // }

        // if self.is_persistent {
        //     unsafe { self.entity.add_serialized(self.serialize_persistence())? }
        // }

        // Ok(self.entity)
    }
}
