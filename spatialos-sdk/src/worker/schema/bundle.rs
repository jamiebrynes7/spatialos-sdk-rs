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

#[cfg(test)]
mod tests {
    use crate::worker::{component::ComponentId, schema::*};
    use approx::assert_abs_diff_eq;
    use serde_json::Value;
    use std::{fs::File, io::Read, path::PathBuf};

    const POSITION_COMPONENT_ID: ComponentId = 54;
    #[test]
    pub fn loading_bundle_succeeds_when_valid_bundle() {
        assert!(read_bundle(true).is_ok(), "Valid bundle failed to load.")
    }

    // TODO: This test currently fails (but it shouldn't!)
    //#[test]
    pub fn loading_bundle_fails_when_invalid_bundle() {
        assert!(
            read_bundle(false).is_err(),
            "Invalid bundle loaded without error"
        )
    }

    #[test]
    pub fn load_object_fills_object_when_valid_json() {
        const json: &str = "{\"x\":10.0,\"y\":10.0,\"z\":10.0}";

        let mut data = SchemaComponentUpdate::new();
        let fields = data.fields_mut();

        let bundle = get_valid_bundle();
        let result = bundle.load_object("improbable.Coordinates", json, fields);

        let (_, warning) = check(result);

        assert!(warning.is_none(), "Unexpected warnings");
        assert_abs_diff_eq!(
            fields
                .get::<SchemaDouble>(1)
                .expect("Could not read data from field."),
            10.0
        );
        assert_abs_diff_eq!(
            fields
                .get::<SchemaDouble>(2)
                .expect("Could not read data from field."),
            10.0
        );
        assert_abs_diff_eq!(
            fields
                .get::<SchemaDouble>(3)
                .expect("Could not read data from field."),
            10.0
        );
    }

    #[test]
    pub fn load_object_returns_error_with_malformed_json() {
        const json: &str = "{}";

        let mut data = SchemaGenericData::new();
        let obj = data.object_mut();

        let bundle = get_valid_bundle();
        let result = bundle.load_object("improbable.Coordinates", json, obj);

        assert!(result.is_err())
    }

    #[test]
    pub fn dump_object_returns_valid_json() {
        let mut data = SchemaGenericData::new();
        let fields = data.object_mut();
        fields.add::<SchemaDouble>(1, &10.0);
        fields.add::<SchemaDouble>(2, &10.0);
        fields.add::<SchemaDouble>(3, &10.0);

        let bundle = get_valid_bundle();
        let result = bundle.dump_object("improbable.Coordinates", fields);
        let (json, warning) = check(result);

        assert!(warning.is_none(), "Unexpected warnings");
        check_valid_json(json);
    }

    #[test]
    pub fn load_component_data_fills_data_when_valid_json() {
        const json: &str = "{\"coords\": {\"x\":10.0,\"y\":10.0,\"z\":10.0}}";

        let bundle = get_valid_bundle();
        let result = bundle.load_component_data(POSITION_COMPONENT_ID, json);

        let (component_data, warning) = check(result);
        assert!(warning.is_none(), "Unexpected warnings");

        assert_eq!(component_data.fields().object_count(1), 1);
        let coords_obj = component_data.fields().get_object(1);

        assert_abs_diff_eq!(
            coords_obj
                .get::<SchemaDouble>(1)
                .expect("Could not read data from field."),
            10.0
        );
        assert_abs_diff_eq!(
            coords_obj
                .get::<SchemaDouble>(2)
                .expect("Could not read data from field."),
            10.0
        );
        assert_abs_diff_eq!(
            coords_obj
                .get::<SchemaDouble>(3)
                .expect("Could not read data from field."),
            10.0
        );
    }

    #[test]
    pub fn load_component_data_returns_error_with_malformed_json() {
        const json: &str = "{}";

        let bundle = get_valid_bundle();
        let result = bundle.load_component_data(POSITION_COMPONENT_ID, json);

        assert!(result.is_err());
    }

    #[test]
    pub fn dump_component_data_returns_valid_json() {
        let mut component_data = SchemaComponentData::new();
        let fields = component_data.fields_mut();
        let coords_obj = fields.add_object(1);
        coords_obj.add::<SchemaDouble>(1, &10.0);
        coords_obj.add::<SchemaDouble>(2, &10.0);
        coords_obj.add::<SchemaDouble>(3, &10.0);

        let bundle = get_valid_bundle();
        let result = bundle.dump_component_data(POSITION_COMPONENT_ID, &mut component_data);
        let (json, warning) = check(result);

        assert!(warning.is_none(), "Unexpected warnings");
        check_valid_json(json);
    }

    #[test]
    pub fn load_component_update_is_okay_with_empty_update() {
        const json: &str = "{}";

        let bundle = get_valid_bundle();
        let result = bundle.load_component_update(POSITION_COMPONENT_ID, json);

        let (component_update, warning) = check(result);
        assert!(warning.is_none(), "Unexpected warnings");

        assert_eq!(component_update.fields().object_count(1), 0);
    }

    #[test]
    pub fn load_component_update_fills_data_when_valid_json() {
        const json: &str = "{\"coords\": {\"x\":10.0,\"y\":10.0,\"z\":10.0}}";

        let bundle = get_valid_bundle();
        let result = bundle.load_component_update(POSITION_COMPONENT_ID, json);

        let (component_update, warning) = check(result);
        assert!(warning.is_none(), "Unexpected warnings");

        assert_eq!(component_update.fields().object_count(1), 1);
        let coords_obj = component_update.fields().get_object(1);

        assert_abs_diff_eq!(
            coords_obj
                .get::<SchemaDouble>(1)
                .expect("Could not read data from field."),
            10.0
        );
        assert_abs_diff_eq!(
            coords_obj
                .get::<SchemaDouble>(2)
                .expect("Could not read data from field."),
            10.0
        );
        assert_abs_diff_eq!(
            coords_obj
                .get::<SchemaDouble>(3)
                .expect("Could not read data from field."),
            10.0
        );
    }

    #[test]
    pub fn convert_data_to_update_returns_valid_update_when_given_valid_data() {
        let mut component_data = SchemaComponentData::new();
        let fields = component_data.fields_mut();
        let coords_obj = fields.add_object(1);
        coords_obj.add::<SchemaDouble>(1, &10.0);
        coords_obj.add::<SchemaDouble>(2, &10.0);
        coords_obj.add::<SchemaDouble>(3, &10.0);

        let bundle = get_valid_bundle();
        let result = bundle.convert_data_to_update(POSITION_COMPONENT_ID, component_data);

        let component_update = match result {
            Ok(update) => update,
            Err(e) => panic!(format!("Unexpected error: {}", e)),
        };

        let coords_obj = component_update.fields().get_object(1);

        assert_abs_diff_eq!(
            coords_obj
                .get::<SchemaDouble>(1)
                .expect("Could not read data from field."),
            10.0
        );
        assert_abs_diff_eq!(
            coords_obj
                .get::<SchemaDouble>(2)
                .expect("Could not read data from field."),
            10.0
        );
        assert_abs_diff_eq!(
            coords_obj
                .get::<SchemaDouble>(3)
                .expect("Could not read data from field."),
            10.0
        );
    }

    #[test]
    pub fn dump_component_update_returns_valid_json() {
        let mut component_update = SchemaComponentUpdate::new();
        let fields = component_update.fields_mut();
        let coords_obj = fields.add_object(1);
        coords_obj.add::<SchemaDouble>(1, &10.0);
        coords_obj.add::<SchemaDouble>(2, &10.0);
        coords_obj.add::<SchemaDouble>(3, &10.0);

        let bundle = get_valid_bundle();
        let result = bundle.dump_component_update(POSITION_COMPONENT_ID, &mut component_update);
        let (json, warning) = check(result);

        assert!(warning.is_none(), "Unexpected warnings");
        check_valid_json(json);
    }

    fn get_valid_bundle() -> Bundle {
        read_bundle(true).expect("Failed to load bundle")
    }

    fn read_bundle(valid: bool) -> std::result::Result<Bundle, String> {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/");
        d.push(if valid { "good_bundle" } else { "bad_bundle" });

        let mut buffer = Vec::new();
        File::open(d)
            .expect("Could not open bundle file")
            .read_to_end(&mut buffer)
            .expect("Failed to read file from disk");

        Bundle::load(&buffer)
    }

    fn check<T>(res: JsonConversionResult<T>) -> (T, Option<String>) {
        match res {
            Ok(pair) => pair,
            Err(e) => panic!("JSON operation failed: ".to_owned() + &e),
        }
    }

    fn check_valid_json(json: String) {
        assert!(!json.is_empty(), "Dumped JSON is empty");

        // Check that the JSON is valid by parsing it into 'Value' which is a union
        // of all possible JSON values.
        let json = serde_json::from_str::<Value>(&json);
        assert!(json.is_ok());
    }
}
