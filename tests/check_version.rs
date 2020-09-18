use spiro_sys;
use std::ffi::CStr;

#[test]
fn check_version() {
    let version: String;
    unsafe {
        let v = spiro_sys::LibSpiroVersion();
        let c_v = CStr::from_ptr(v);
        version = c_v.to_str().unwrap().to_string();
    }
    // Defined in spiro-config.h
    assert_eq!(&version, "rust");
}
