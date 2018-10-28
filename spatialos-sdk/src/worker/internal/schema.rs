use spatialos_sdk_sys::worker::*;
use std::marker::PhantomData;

pub struct SchemaComponentUpdate {
    pub component_id: u32,
    pub internal: *mut Schema_ComponentUpdate,
}

pub struct SchemaComponentData {
    pub component_id: u32,
    pub internal: *mut Schema_ComponentData,
}

pub struct SchemaObject {
    internal: *mut Schema_Object,
}

pub struct SchemaFieldContainer<'a, T> {
    field_id: u32,
    container: &'a SchemaObject,
    _phantom: PhantomData<T>,
}

pub trait SchemaField<T> {
    fn get(&self) -> Option<T> {
        if self.count() == 0 { None } else { Some(self.get_or_default()) }
    }

    fn get_or_default(&self) -> T;
    fn index(&self, index: usize) -> T;
    fn count(&self) -> usize;

    fn add(&mut self, value: T);
    fn add_list(&mut self, value: &[T]);
}

impl SchemaComponentUpdate {
    pub(crate) fn from_worker_sdk(component_id: u32, component_update: *mut Schema_ComponentUpdate) -> SchemaComponentUpdate {
        SchemaComponentUpdate {
            component_id: component_id,
            internal: component_update
        }
    }

    pub fn new(component_id: u32) -> SchemaComponentUpdate {
        SchemaComponentUpdate {
            component_id: component_id,
            internal: unsafe { Schema_CreateComponentUpdate(component_id) }
        }
    }

    pub fn fields(&self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetComponentUpdateFields(self.internal) }
        }
    }

    pub fn fields_mut(&mut self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetComponentUpdateFields(self.internal) }
        }
    }
}

impl SchemaObject {
    pub fn field<T>(&self, field_id: usize) -> SchemaFieldContainer<T> {
        SchemaFieldContainer { field_id: field_id as u32, container: self, _phantom: PhantomData }
    }
}

impl<'a> SchemaField<f32> for SchemaFieldContainer<'a, f32> {
    fn get_or_default(&self) -> f32 {
        unsafe { Schema_GetFloat(self.container.internal, self.field_id) }
    }
    fn index(&self, index: usize) -> f32 {
        unsafe { Schema_IndexFloat(self.container.internal, self.field_id, index as u32) }
    }
    fn count(&self) -> usize {
        unsafe { Schema_GetFloatCount(self.container.internal, self.field_id) as usize }
    }

    fn add(&mut self, value: f32) {
        unsafe { Schema_AddFloat(self.container.internal, self.field_id, value); }
    }
    fn add_list(&mut self, value: &[f32]) {
        unsafe {
            let ptr = value.as_ptr();
            Schema_AddFloatList(self.container.internal, self.field_id, ptr, value.len() as u32);
        }
    }
}