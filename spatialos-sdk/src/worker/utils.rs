use std::ffi::CStr;

pub fn cstr_to_string(ptr: *const std::os::raw::c_char) -> String {
    assert!(!ptr.is_null());
    unsafe {
        CStr::from_ptr(ptr)
            .to_owned()
            .into_string()
            .expect("Failed to unwrap string")
    }
}

pub fn cstr_array_to_vec_string(
    char_ptr: *mut *const std::os::raw::c_char,
    count: u32,
) -> Vec<String> {
    unsafe {
        (0..count as isize)
            .map(|i| {
                let ptr = char_ptr.offset(i) as *mut *const std::os::raw::c_char;
                assert!(!ptr.is_null());
                cstr_to_string(*ptr)
            })
            .collect()
    }
}
