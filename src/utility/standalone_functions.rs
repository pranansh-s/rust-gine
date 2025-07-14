pub unsafe fn get_cstring(name: &str) -> std::ffi::CString {
    std::ffi::CString::new(name).expect("Failed to create CString")
}
