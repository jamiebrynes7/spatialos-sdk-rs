use std::ffi::{CStr, CString};

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
    let mut strings = Vec::new();
    unsafe {
        for i in 0..count as isize {
            let ptr = char_ptr.offset(i) as *mut *const std::os::raw::c_char;
            assert!(!ptr.is_null());
            strings.push(cstr_to_string(*ptr));
        }
    }
    strings
}

pub struct WrappedNativeStructWithString<T> {
    pub native_struct: T,
    pub native_string_ref: CString,
}
