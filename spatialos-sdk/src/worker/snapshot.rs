use crate::worker::{vtable::DATABASE, entity::Entity, utils::cstr_to_string, EntityId};
use spatialos_sdk_sys::worker::*;
use std::{ffi::CString, path::Path};

#[derive(Debug)]
pub enum SnapshotError {
    BadState(String),
    InvalidData(String),
    EntitySerializationFailure(String),
    EOF,
}

impl From<Worker_SnapshotState> for SnapshotError {
    fn from(state: Worker_SnapshotState) -> SnapshotError {
        match Worker_StreamState::from(state.stream_state) {
            Worker_StreamState_WORKER_STREAM_STATE_BAD => {
                SnapshotError::BadState(cstr_to_string(state.error_message))
            }
            Worker_StreamState_WORKER_STREAM_STATE_INVALID_DATA => {
                SnapshotError::InvalidData(cstr_to_string(state.error_message))
            }
            Worker_StreamState_WORKER_STREAM_STATE_EOF => SnapshotError::EOF,
            _ => SnapshotError::BadState(cstr_to_string(state.error_message)),
        }
    }
}

pub struct SnapshotOutputStream {
    ptr: *mut Worker_SnapshotOutputStream,
}

impl SnapshotOutputStream {
    pub fn new<P: AsRef<Path>>(filename: P) -> Result<Self, SnapshotError> {
        let filename_cstr = CString::new(filename.as_ref().to_str().unwrap()).unwrap();

        let params = Worker_SnapshotParameters {
            component_vtable_count: DATABASE.len() as u32,
            component_vtables: DATABASE.to_worker_sdk(),
            default_component_vtable: std::ptr::null(),
        };
        let stream_ptr =
            unsafe { Worker_SnapshotOutputStream_Create(filename_cstr.as_ptr(), &params) };

        let state = unsafe { Worker_SnapshotOutputStream_GetState(stream_ptr) };
        match Worker_StreamState::from(state.stream_state) {
            Worker_StreamState_WORKER_STREAM_STATE_GOOD => {
                Ok(SnapshotOutputStream { ptr: stream_ptr })
            }
            _ => {
                unsafe { Worker_SnapshotOutputStream_Destroy(stream_ptr) };
                Err(SnapshotError::from(state))
            }
        }
    }

    pub fn write_entity(&mut self, id: EntityId, entity: &Entity) -> Result<(), SnapshotError> {
        let components = entity.raw_component_data();
        let wrk_entity = Worker_Entity {
            entity_id: id.id,
            components: components.components.as_ptr(),
            component_count: components.components.len() as u32,
        };

        let state = unsafe {
            Worker_SnapshotOutputStream_WriteEntity(self.ptr, &wrk_entity);
            Worker_SnapshotOutputStream_GetState(self.ptr)
        };
        match Worker_StreamState::from(state.stream_state) {
            Worker_StreamState_WORKER_STREAM_STATE_GOOD => Ok(()),
            _ => Err(SnapshotError::from(state)),
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
    pub fn new<P: AsRef<Path>>(filename: P) -> Result<Self, SnapshotError> {
        let filename_cstr = CString::new(filename.as_ref().to_str().unwrap()).unwrap();

        let params = Worker_SnapshotParameters {
            component_vtable_count: DATABASE.len() as u32,
            component_vtables: DATABASE.to_worker_sdk(),
            default_component_vtable: std::ptr::null(),
        };

        let stream_ptr =
            unsafe { Worker_SnapshotInputStream_Create(filename_cstr.as_ptr(), &params) };

        let state = unsafe { Worker_SnapshotInputStream_GetState(stream_ptr) };
        match Worker_StreamState::from(state.stream_state) {
            Worker_StreamState_WORKER_STREAM_STATE_GOOD => {
                Ok(SnapshotInputStream { ptr: stream_ptr })
            }
            _ => {
                unsafe { Worker_SnapshotInputStream_Destroy(stream_ptr) };
                Err(SnapshotError::from(state))
            }
        }
    }

    pub fn has_next(&mut self) -> bool {
        unsafe { Worker_SnapshotInputStream_HasNext(self.ptr) != 0 }
    }

    pub fn read_entity(&mut self) -> Result<Entity, SnapshotError> {
        let entity_ptr = unsafe { Worker_SnapshotInputStream_ReadEntity(self.ptr) };
        let state = unsafe { Worker_SnapshotInputStream_GetState(self.ptr) };

        match Worker_StreamState::from(state.stream_state) {
            Worker_StreamState_WORKER_STREAM_STATE_GOOD => unsafe {
                let entity = Entity::from_worker_sdk(&*entity_ptr);
                if let Err(message) = entity {
                    Err(SnapshotError::EntitySerializationFailure(message))
                } else {
                    Ok(entity.unwrap())
                }
            },
            _ => Err(SnapshotError::from(state)),
        }
    }
}

impl Drop for SnapshotInputStream {
    fn drop(&mut self) {
        unsafe { Worker_SnapshotInputStream_Destroy(self.ptr) }
    }
}
