use std::path::Path;

use std::ffi::CStr;
use std::ffi::CString;
use worker::core::entity_snapshot::EntitySnapshot;
use worker::core::parameters::SnapshotParameters;
use worker::internal::bindings::*;

pub struct SnapshotOutputStream {
    internal_ptr: *mut Worker_SnapshotOutputStream,
}

impl SnapshotOutputStream {
    pub fn new<P: AsRef<Path>>(filename: P, params: SnapshotParameters) -> Self {
        let filename_cstr = CString::new(filename.as_ref().to_str().unwrap()).unwrap();

        let ptr = unsafe {
            Worker_SnapshotOutputStream_Create(filename_cstr.as_ptr(), &params.to_worker_sdk())
        };

        SnapshotOutputStream { internal_ptr: ptr }
    }

    pub fn write_entity(&self, snapshot: &EntitySnapshot) -> Result<(), String> {
        let _ = unsafe {
            Worker_SnapshotOutputStream_WriteEntity(self.internal_ptr, &snapshot.to_worker_sdk())
        };
        let error_msg = unsafe { Worker_SnapshotOutputStream_GetError(self.internal_ptr) };

        if error_msg.is_null() {
            Ok(())
        } else {
            let cstr = unsafe { CStr::from_ptr(error_msg) };
            Err(cstr.to_owned().into_string().unwrap())
        }
    }
}

impl Drop for SnapshotOutputStream {
    fn drop(&mut self) {
        unsafe { Worker_SnapshotOutputStream_Destroy(self.internal_ptr) };
    }
}
