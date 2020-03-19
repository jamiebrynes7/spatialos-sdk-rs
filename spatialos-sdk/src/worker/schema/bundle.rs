use crate::worker::{component::ComponentId, schema::*, utils::cstr_to_string};
use spatialos_sdk_sys::worker::*;
use std::ffi::CString;
use std::ptr::NonNull;

pub type JsonConversionResult<T> = std::result::Result<(T, Option<String>), String>;

pub struct Bundle {
    ptr: NonNull<Schema_Bundle>,
}

impl Bundle {
    pub fn load(buffer: &[u8]) -> std::result::Result<Self, String> {
        unsafe {
            let ptr = Schema_Bundle_Load(buffer.as_ptr(), buffer.len() as u32);
            let ptr = NonNull::new(ptr)
                .ok_or_else(|| "Received null pointer from Schema_Bundle_Load".to_string())?;
            let err = Schema_Bundle_GetError(ptr.as_ptr());

            if !err.is_null() {
                let msg = cstr_to_string(err);
                Schema_Bundle_Destroy(ptr.as_ptr());
                Err(msg)
            } else {
                Ok(Bundle { ptr })
            }
        }
    }

    pub fn load_object<T: AsRef<str>, U: AsRef<str>>(
        &self,
        qualified_type_name: T,
        json: U,
        dest: &mut SchemaObject,
    ) -> JsonConversionResult<()> {
        let type_name = CString::new(qualified_type_name.as_ref())
            .map_err(|_| "Null byte found in 'qualified_type_name'".to_string())?;
        let json =
            CString::new(json.as_ref()).map_err(|_| "Null byte found in 'json'".to_string())?;

        unsafe {
            let success = Schema_Json_LoadObject(
                self.ptr.as_ptr(),
                type_name.as_ptr(),
                json.as_ptr(),
                dest.as_ptr_mut(),
            );

            if success == 0 {
                return Err(Bundle::get_last_error());
            }

            Ok(((), Bundle::get_last_warning()))
        }
    }

    // TODO: Why is this mut?
    pub fn dump_object<T: AsRef<str>>(
        &self,
        qualified_type_name: T,
        src: &mut SchemaObject,
    ) -> JsonConversionResult<String> {
        let type_name = CString::new(qualified_type_name.as_ref())
            .map_err(|_| "Null byte found in 'qualified_type_name'".to_string())?;

        unsafe {
            let json_ptr =
                Schema_Json_DumpObject(self.ptr.as_ptr(), type_name.as_ptr(), src.as_ptr_mut());

            if json_ptr.is_null() {
                return Err(Bundle::get_last_error());
            }

            let json = cstr_to_string(Schema_Json_GetJsonString(json_ptr));
            Schema_Json_Destroy(json_ptr);

            Ok((json, Bundle::get_last_warning()))
        }
    }

    pub fn load_component_data<T: AsRef<str>>(
        &self,
        component_id: ComponentId,
        json: T,
    ) -> JsonConversionResult<Owned<SchemaComponentData>> {
        let json =
            CString::new(json.as_ref()).map_err(|_| "Null byte found in 'json'".to_string())?;

        unsafe {
            self.load_generic(move || {
                Schema_Json_LoadComponentData(self.ptr.as_ptr(), component_id, json.as_ptr())
            })
        }
    }

    pub fn dump_component_data(
        &self,
        component_id: ComponentId,
        src: &mut SchemaComponentData,
    ) -> JsonConversionResult<String> {
        unsafe {
            self.dump_generic(move || {
                Schema_Json_DumpComponentData(self.ptr.as_ptr(), component_id, src.as_ptr_mut())
            })
        }
    }

    pub fn load_component_update<T: AsRef<str>>(
        &self,
        component_id: ComponentId,
        json: T,
    ) -> JsonConversionResult<Owned<SchemaComponentUpdate>> {
        let json =
            CString::new(json.as_ref()).map_err(|_| "Null byte found in 'json'".to_string())?;

        unsafe {
            self.load_generic(move || {
                Schema_Json_LoadComponentUpdate(self.ptr.as_ptr(), component_id, json.as_ptr())
            })
        }
    }

    pub fn dump_component_update(
        &self,
        component_id: ComponentId,
        src: &mut SchemaComponentUpdate,
    ) -> JsonConversionResult<String> {
        unsafe {
            self.dump_generic(move || {
                Schema_Json_DumpComponentUpdate(self.ptr.as_ptr(), component_id, src.as_ptr_mut())
            })
        }
    }

    pub fn load_command_request<T: AsRef<str>>(
        &self,
        component_id: ComponentId,
        command_index: u32,
        json: T,
    ) -> JsonConversionResult<Owned<SchemaCommandRequest>> {
        let json =
            CString::new(json.as_ref()).map_err(|_| "Null byte found in 'json'".to_string())?;

        unsafe {
            self.load_generic(move || {
                Schema_Json_LoadCommandRequest(
                    self.ptr.as_ptr(),
                    component_id,
                    command_index,
                    json.as_ptr(),
                )
            })
        }
    }

    pub fn dump_command_request(
        &self,
        component_id: ComponentId,
        command_index: u32,
        src: &mut SchemaCommandRequest,
    ) -> JsonConversionResult<String> {
        unsafe {
            self.dump_generic(move || {
                Schema_Json_DumpCommandRequest(
                    self.ptr.as_ptr(),
                    component_id,
                    command_index,
                    src.as_ptr_mut(),
                )
            })
        }
    }

    pub fn load_command_response<T: AsRef<str>>(
        &self,
        component_id: ComponentId,
        command_index: u32,
        json: T,
    ) -> JsonConversionResult<Owned<SchemaCommandResponse>> {
        let json =
            CString::new(json.as_ref()).map_err(|_| "Null byte found in 'json'".to_string())?;

        unsafe {
            self.load_generic(move || {
                Schema_Json_LoadCommandResponse(
                    self.ptr.as_ptr(),
                    component_id,
                    command_index,
                    json.as_ptr(),
                )
            })
        }
    }

    pub fn dump_command_response(
        &self,
        component_id: ComponentId,
        command_index: u32,
        src: &mut SchemaCommandResponse,
    ) -> JsonConversionResult<String> {
        unsafe {
            self.dump_generic(move || {
                Schema_Json_DumpCommandResponse(
                    self.ptr.as_ptr(),
                    component_id,
                    command_index,
                    src.as_ptr_mut(),
                )
            })
        }
    }

    unsafe fn load_generic<
        T: OwnedPointer + ToOwned<Owned = Owned<T>>,
        F: FnOnce() -> *mut T::Raw,
    >(
        &self,
        load: F,
    ) -> JsonConversionResult<Owned<T>> {
        let data = load();

        if data.is_null() {
            return Err(Bundle::get_last_error());
        }

        let concrete = T::from_raw_mut(data).to_owned();
        T::DESTROY_FN(data);
        Ok((concrete, Bundle::get_last_warning()))
    }

    unsafe fn dump_generic<F: FnOnce() -> *mut Schema_Json>(
        &self,
        dump: F,
    ) -> JsonConversionResult<String> {
        let json_ptr = dump();

        if json_ptr.is_null() {
            return Err(Bundle::get_last_error());
        }

        let json = cstr_to_string(Schema_Json_GetJsonString(json_ptr));
        Schema_Json_Destroy(json_ptr);

        Ok((json, Bundle::get_last_warning()))
    }

    unsafe fn get_last_error() -> String {
        let err = Schema_Json_GetLastError();
        if err.is_null() {
            "Unknown error in loading object".to_string()
        } else {
            cstr_to_string(err)
        }
    }

    unsafe fn get_last_warning() -> Option<String> {
        let maybe_warning = Schema_Json_GetLastWarning();
        if maybe_warning.is_null() {
            None
        } else {
            Some(cstr_to_string(maybe_warning))
        }
    }

    pub fn convert_data_to_update(
        &self,
        component_id: ComponentId,
        data: Owned<SchemaComponentData>,
    ) -> std::result::Result<Owned<SchemaComponentUpdate>, String> {
        let mut error: Option<String> = None;
        unsafe {
            let update_ptr = Schema_ConvertComponentDataIntoUpdate(
                self.ptr.as_ptr(),
                component_id,
                data.into_raw(),
                &mut error as *mut _ as *mut _,
                Some(Bundle::record_error),
            );

            if update_ptr.is_null() {
                return Err(error.unwrap_or_else(|| {
                    "Unknown error occurred when converting data into update.".to_string()
                }));
            }

            let update = SchemaComponentUpdate::from_raw_mut(update_ptr).to_owned();
            Schema_DestroyComponentUpdate(update_ptr);

            Ok(update)
        }
    }

    extern "C" fn record_error(
        user_data: *mut ::std::os::raw::c_void,
        error: *const ::std::os::raw::c_char,
    ) {
        unsafe {
            if error.is_null() {
                return;
            }

            let data: &mut Option<String> = &mut *(user_data as *mut Option<String>);
            let error = cstr_to_string(error);
            *data = Some(error);
        }
    }
}

// SAFETY: It should be safe to send a `Bundle` between threads, so long as it's only ever accessed
// from one thread at a time. It has unsychronized internal mutability (only storing the 'last' error
// and warning) so it cannot be Sync.
unsafe impl Send for Bundle {}

impl Drop for Bundle {
    fn drop(&mut self) {
        unsafe {
            Schema_Bundle_Destroy(self.ptr.as_ptr());
        }
    }
}

