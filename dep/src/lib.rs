use std::ffi::c_int;

#[no_mangle]
pub extern "C" fn open_coroutine_init() -> c_int {
    eprintln!("dep works");
    0
}
