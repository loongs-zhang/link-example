use libc::timespec;
use std::ffi::c_int;

#[no_mangle]
pub extern "C" fn open_coroutine_init() -> c_int {
    eprintln!("dep works");
    0
}

#[no_mangle]
pub extern "C" fn nanosleep(rqtp: *const timespec, rmtp: *mut timespec) -> c_int {
    eprintln!("hook nanosleep works");
    0
}
