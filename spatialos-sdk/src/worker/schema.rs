use crate::worker::EntityId;
use spatialos_sdk_sys::worker::*;
use std::marker::PhantomData;
use std::slice;

mod command_request;
mod command_response;
mod component_data;
mod component_update;
mod object;
mod primitives;

pub use self::{
    command_request::*, command_response::*, component_data::*, component_update::*, object::*,
    primitives::*,
};

pub type FieldId = u32;

// A schema field. T is a schema type tag.
#[derive(Debug)]
pub struct SchemaFieldContainer<'a, T> {
    field_id: FieldId,
    container: &'a SchemaObject,
    _phantom: PhantomData<T>,
}

// A primitive schema field.
pub trait SchemaPrimitiveField<T> {
    fn get(&self) -> Option<T> {
        if self.count() == 0 {
            None
        } else {
            Some(self.get_or_default())
        }
    }

    fn get_or_default(&self) -> T;
    fn index(&self, index: usize) -> T;
    fn count(&self) -> usize;

    fn add(&mut self, value: T);
    fn add_list(&mut self, value: &[T]);
}

// A bytes schema field.
pub trait SchemaBytesField {
    fn get(&self) -> Option<Vec<u8>> {
        if self.count() == 0 {
            None
        } else {
            Some(self.get_or_default())
        }
    }

    fn get_or_default(&self) -> Vec<u8>;
    fn index(&self, index: usize) -> Vec<u8>;
    fn count(&self) -> usize;
    fn add(&mut self, value: &[u8]);
}

// A string schema field.
#[allow(clippy::ptr_arg)]
pub trait SchemaStringField {
    fn get(&self) -> Option<String> {
        if self.count() == 0 {
            None
        } else {
            Some(self.get_or_default())
        }
    }

    fn get_or_default(&self) -> String;
    fn index(&self, index: usize) -> String;
    fn count(&self) -> usize;

    fn add(&mut self, value: &String);
    fn add_list(&mut self, value: &[String]);
}

// An object schema field.
pub trait SchemaObjectField {
    fn get(&self) -> Option<SchemaObject> {
        if self.count() == 0 {
            None
        } else {
            Some(self.get_or_default())
        }
    }

    fn get_or_default(&self) -> SchemaObject;
    fn index(&self, index: usize) -> SchemaObject;
    fn count(&self) -> usize;

    fn add(&mut self) -> SchemaObject;
}

impl<'a> SchemaPrimitiveField<EntityId> for SchemaFieldContainer<'a, SchemaEntityId> {
    fn get_or_default(&self) -> EntityId {
        EntityId::new(unsafe { Schema_GetEntityId(self.container.internal, self.field_id) })
    }
    fn index(&self, index: usize) -> EntityId {
        EntityId::new(unsafe {
            Schema_IndexEntityId(self.container.internal, self.field_id, index as u32)
        })
    }
    fn count(&self) -> usize {
        unsafe { Schema_GetEntityIdCount(self.container.internal, self.field_id) as usize }
    }

    fn add(&mut self, value: EntityId) {
        unsafe {
            Schema_AddEntityId(self.container.internal, self.field_id, value.id);
        }
    }
    fn add_list(&mut self, value: &[EntityId]) {
        let converted_list: Vec<i64> = value.iter().map(|v| v.id).collect();
        unsafe {
            let ptr = converted_list.as_ptr();
            Schema_AddEntityIdList(
                self.container.internal,
                self.field_id,
                ptr,
                value.len() as u32,
            );
        }
    }
}

impl<'a> SchemaPrimitiveField<bool> for SchemaFieldContainer<'a, SchemaBool> {
    fn get_or_default(&self) -> bool {
        unsafe { Schema_GetBool(self.container.internal, self.field_id) != 0 }
    }
    fn index(&self, index: usize) -> bool {
        unsafe { Schema_IndexBool(self.container.internal, self.field_id, index as u32) != 0 }
    }
    fn count(&self) -> usize {
        unsafe { Schema_GetBoolCount(self.container.internal, self.field_id) as usize }
    }

    fn add(&mut self, value: bool) {
        unsafe {
            Schema_AddBool(self.container.internal, self.field_id, value as u8);
        }
    }
    fn add_list(&mut self, value: &[bool]) {
        let converted_list: Vec<u8> = value.iter().map(|v| if *v { 1u8 } else { 0u8 }).collect();
        unsafe {
            let ptr = converted_list.as_ptr();
            Schema_AddBoolList(
                self.container.internal,
                self.field_id,
                ptr,
                value.len() as u32,
            );
        }
    }
}

impl<'a> SchemaStringField for SchemaFieldContainer<'a, SchemaString> {
    fn get_or_default(&self) -> String {
        let slice = unsafe {
            let bytes_ptr = Schema_GetBytes(self.container.internal, self.field_id);
            let bytes_len = Schema_GetBytesLength(self.container.internal, self.field_id);
            slice::from_raw_parts(bytes_ptr, bytes_len as usize)
        };
        String::from_utf8_lossy(slice).to_string()
    }
    fn index(&self, index: usize) -> String {
        let slice = unsafe {
            let bytes_ptr = Schema_IndexBytes(self.container.internal, self.field_id, index as u32);
            let bytes_len =
                Schema_IndexBytesLength(self.container.internal, self.field_id, index as u32);
            slice::from_raw_parts(bytes_ptr, bytes_len as usize)
        };
        String::from_utf8_lossy(slice).to_string()
    }
    fn count(&self) -> usize {
        unsafe { Schema_GetBytesCount(self.container.internal, self.field_id) as usize }
    }

    fn add(&mut self, value: &String) {
        let utf8_bytes = value.as_bytes();
        unsafe {
            Schema_AddBytes(
                self.container.internal,
                self.field_id,
                utf8_bytes.as_ptr(),
                utf8_bytes.len() as u32,
            );
        }
    }
    fn add_list(&mut self, value: &[String]) {
        for str in value.iter() {
            self.add(str);
        }
    }
}

impl<'a> SchemaBytesField for SchemaFieldContainer<'a, SchemaBytes> {
    fn get_or_default(&self) -> Vec<u8> {
        let slice = unsafe {
            let bytes_ptr = Schema_GetBytes(self.container.internal, self.field_id);
            let bytes_len = Schema_GetBytesLength(self.container.internal, self.field_id);
            slice::from_raw_parts(bytes_ptr, bytes_len as usize)
        };
        slice.to_vec()
    }
    fn index(&self, index: usize) -> Vec<u8> {
        let slice = unsafe {
            let bytes_ptr = Schema_IndexBytes(self.container.internal, self.field_id, index as u32);
            let bytes_len =
                Schema_IndexBytesLength(self.container.internal, self.field_id, index as u32);
            slice::from_raw_parts(bytes_ptr, bytes_len as usize)
        };
        slice.to_vec()
    }
    fn count(&self) -> usize {
        unsafe { Schema_GetBytesCount(self.container.internal, self.field_id) as usize }
    }

    fn add(&mut self, value: &[u8]) {
        unsafe {
            Schema_AddBytes(
                self.container.internal,
                self.field_id,
                value.as_ptr(),
                value.len() as u32,
            );
        }
    }
}

impl<'a> SchemaObjectField for SchemaFieldContainer<'a, SchemaObject> {
    fn get_or_default(&self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_GetObject(self.container.internal, self.field_id) },
        }
    }
    fn index(&self, index: usize) -> SchemaObject {
        SchemaObject {
            internal: unsafe {
                Schema_IndexObject(self.container.internal, self.field_id, index as u32)
            },
        }
    }
    fn count(&self) -> usize {
        unsafe { Schema_GetObjectCount(self.container.internal, self.field_id) as usize }
    }

    fn add(&mut self) -> SchemaObject {
        SchemaObject {
            internal: unsafe { Schema_AddObject(self.container.internal, self.field_id) },
        }
    }
}
