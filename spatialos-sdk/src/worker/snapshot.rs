use crate::{
    worker::component::DATABASE, worker::entity::Entity,
    worker::internal::utils::cstr_to_string, worker::EntityId,
};
use spatialos_sdk_sys::worker::*;
use std::{ffi::CString, path::Path};

pub struct SnapshotOutputStream {
    ptr: *mut Worker_SnapshotOutputStream,
}

impl SnapshotOutputStream {
    pub fn new<P: AsRef<Path>>(filename: P) -> Result<Self, String> {
        let filename_cstr = CString::new(filename.as_ref().to_str().unwrap()).unwrap();

        let params = Worker_SnapshotParameters {
            component_vtable_count: DATABASE.len() as u32,
            component_vtables: DATABASE.to_worker_sdk(),
            default_component_vtable: std::ptr::null(),
        };

        let ptr = unsafe { Worker_SnapshotOutputStream_Create(filename_cstr.as_ptr(), &params) };

        let stream = SnapshotOutputStream { ptr };

        let err_ptr = unsafe { Worker_SnapshotOutputStream_GetError(ptr) };
        if !err_ptr.is_null() {
            unsafe { Worker_SnapshotOutputStream_Destroy(ptr) };
            return Err(cstr_to_string(err_ptr));
        }

        Ok(stream)
    }

    pub fn write_entity(&self, id: EntityId, entity: &Entity) -> Result<(), String> {
        let components = entity.raw_component_data();

        let wrk_entity = Worker_Entity {
            entity_id: id.id,
            components: components.components.as_ptr(),
            component_count: components.components.len() as u32,
        };

        let success =
            unsafe { Worker_SnapshotOutputStream_WriteEntity(self.ptr, &wrk_entity) != 0 };

        if success {
            Ok(())
        } else {
            let msg_cstr = unsafe { Worker_SnapshotOutputStream_GetError(self.ptr) };
            let msg = cstr_to_string(msg_cstr);
            Err(msg)
        }
    }
}

impl Drop for SnapshotOutputStream {
    fn drop(&mut self) {
        unsafe { Worker_SnapshotOutputStream_Destroy(self.ptr) };
    }
}

pub struct SnapshotInputStream {
    ptr: *mut Worker_SnapshotInputStream,
}

impl SnapshotInputStream {
    pub fn new<P: AsRef<Path>>(filename: P) -> Result<Self, String> {
        let filename_cstr = CString::new(filename.as_ref().to_str().unwrap()).unwrap();

        let params = Worker_SnapshotParameters {
            component_vtable_count: DATABASE.len() as u32,
            component_vtables: DATABASE.to_worker_sdk(),
            default_component_vtable: std::ptr::null(),
        };

        let ptr = unsafe { Worker_SnapshotInputStream_Create(filename_cstr.as_ptr(), &params) };

        let stream = SnapshotInputStream { ptr };

        let err_ptr = unsafe { Worker_SnapshotInputStream_GetError(ptr) };
        if !err_ptr.is_null() {
            unsafe {
                Worker_SnapshotInputStream_Destroy(ptr);
            }
            return Err(cstr_to_string(err_ptr));
        }

        Ok(stream)
    }

    pub fn has_next(&mut self) -> bool {
        unsafe { Worker_SnapshotInputStream_HasNext(self.ptr) != 0 }
    }

    pub fn read_entity(&mut self) -> Result<Entity, String> {
        let wrk_entity_ptr = unsafe { Worker_SnapshotInputStream_ReadEntity(self.ptr) };
        let err_ptr = unsafe { Worker_SnapshotInputStream_GetError(self.ptr) };

        if !err_ptr.is_null() {
            return Err(cstr_to_string(err_ptr));
        }

        let wrk_entity = unsafe { *wrk_entity_ptr };

        unsafe { Entity::from_worker_sdk(&wrk_entity) }
    }
}

impl Drop for SnapshotInputStream {
    fn drop(&mut self) {
        unsafe { Worker_SnapshotInputStream_Destroy(self.ptr) }
    }
}
