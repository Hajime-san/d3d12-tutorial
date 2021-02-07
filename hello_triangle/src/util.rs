use std::{
    str,
    ffi,
    path,
    env
};

pub fn utf16_to_vec(source: &str) -> Vec<u16> {
    source.encode_utf16().chain(Some(0)).collect()
}

pub fn get_relative_file_path(s: &str) -> path::PathBuf {
    let relative_path = path::Path::new(s);
    let pwd = env::current_dir().unwrap();
    let absolute_path = pwd.join(relative_path);

    absolute_path
}

pub fn path_to_wide_str(s: &str) -> Vec<u16> {
    let wide_str = utf16_to_vec(get_relative_file_path(s).to_str().unwrap());

    wide_str
}

pub fn get_pointer_of_interface<T>(object: &mut T) -> *mut *mut ffi::c_void {
    // we need to convert the reference to a pointer
    let raw_ptr = object as *mut T;

    // and the pointer type we can cast to the c_void type required T
    let void_ptr = raw_ptr as *mut *mut ffi::c_void;

    // in one liner
    // void_ptr as *mut *mut T as *mut *mut ffi::c_void

    void_ptr
}
