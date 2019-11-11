use crate::worker::{
    component::Component,
    component::ComponentId,
    entity::Entity,
    schema::{SchemaComponentData, SchemaDouble, SchemaString, SchemaUint32},
};
use std::collections::{HashMap, HashSet};

const ENTITY_ACL_COMPONENT_ID: ComponentId = 50;
const METADATA_COMPONENT_ID: ComponentId = 53;
const POSITION_COMPONENT_ID: ComponentId = 54;
const PERSISTENCE_COMPONENT_ID: ComponentId = 55;

pub struct EntityBuilder {
    entity: Entity,

    position: (f64, f64, f64),
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
            position: (x, y, z),
            write_permissions: HashMap::new(),
            read_permissions: HashSet::new(),
            error: None,
        };

        builder.add_write_access(POSITION_COMPONENT_ID, position_write_layer);
        builder
    }

    pub fn add_component<C: Component, T: Into<String>>(&mut self, data: C, write_layer: T) {
        if let Err(e) = self.entity.add(data) {
            self.error = Some(e);
        };

        self.add_write_access(C::ID, write_layer);
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

    pub fn build(mut self) -> Result<Entity, String> {
        if let Some(e) = self.error {
            return Err(e);
        }

        unsafe {
            self.entity
                .add_serialized(POSITION_COMPONENT_ID, self.serialize_position())?
        };
        unsafe {
            self.entity
                .add_serialized(ENTITY_ACL_COMPONENT_ID, self.serialize_acl())?
        };

        if self.metadata.is_some() {
            unsafe {
                self.entity
                    .add_serialized(METADATA_COMPONENT_ID, self.serialize_metadata())?
            }
        }

        if self.is_persistent {
            unsafe {
                self.entity
                    .add_serialized(PERSISTENCE_COMPONENT_ID, self.serialize_persistence())?
            }
        }

        Ok(self.entity)
    }

    // A workaround for not having access to generated code types here. The shape of Position
    // & EntityAcl are well known, so we can manually serialize them and pass them into the
    // Entity in SchemaComponentData form.
    //
    // This does then expect that there is a valid deserialize method defined for both components
    // in the vtable.
    //
    // If this invariant is broken, then the EntityBuilder is broken. Should we assert against this
    // before we call `entity.add_serialized`?
    fn serialize_position(&self) -> SchemaComponentData {
        let mut position_schema = SchemaComponentData::new();
        let mut position_fields = position_schema.fields_mut();

        let mut coords_obj = position_fields.add_object(1);
        coords_obj.add::<SchemaDouble>(1, &self.position.0);
        coords_obj.add::<SchemaDouble>(2, &self.position.1);
        coords_obj.add::<SchemaDouble>(3, &self.position.2);

        position_schema
    }

    fn serialize_acl(&self) -> SchemaComponentData {
        let mut acl_schema = SchemaComponentData::new();
        let mut acl_fields = acl_schema.fields_mut();

        let mut read_access = acl_fields.add_object(1);
        for layer in &self.read_permissions {
            let mut attribute_set = read_access.add_object(1);
            attribute_set.add::<SchemaString>(1, layer);
        }

        for pair in &self.write_permissions {
            let mut map_obj = acl_fields.add_object(2);
            map_obj.add::<SchemaUint32>(1, pair.0);

            map_obj
                .add_object(2)
                .add_object(1)
                .add::<SchemaString>(1, pair.1);
        }

        acl_schema
    }

    fn serialize_metadata(&self) -> SchemaComponentData {
        let mut metadata_schema = SchemaComponentData::new();
        let mut metadata_fields = metadata_schema.fields_mut();
        metadata_fields.add::<SchemaString>(1, self.metadata.as_ref().unwrap());

        metadata_schema
    }

    fn serialize_persistence(&self) -> SchemaComponentData {
        SchemaComponentData::new()
    }
}
