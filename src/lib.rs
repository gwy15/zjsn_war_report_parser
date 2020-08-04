mod model;
mod utils;
mod writer;

use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

unsafe fn ptr_to_string(ptr: *const c_char) -> String {
    CStr::from_ptr(ptr)
        .to_str()
        .expect("UTF8 valid error")
        .to_owned()
}

#[no_mangle]
pub extern "C" fn convert_directory(ptr: *const c_char) -> c_int {
    let path = unsafe { ptr_to_string(ptr) };
    utils::parse_directory(path).expect("Unknown error happened") as c_int
}

#[no_mangle]
pub extern "C" fn count_directory(ptr: *const c_char) -> c_int {
    let path = unsafe { ptr_to_string(ptr) };
    let files =
        utils::ParseTarget::from_path(path).expect("Unknown error happened while counting files");
    files.len() as c_int
}
