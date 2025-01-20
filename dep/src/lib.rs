use libc::timespec;
use std::ffi::c_int;

#[no_mangle]
pub extern "C" fn open_coroutine_init() -> c_int {
    eprintln!("dep works");
    0
}

#[cfg(unix)]
extern "C" {
    #[cfg(not(any(target_os = "dragonfly", target_os = "vxworks")))]
    #[cfg_attr(
        any(
            target_os = "linux",
            target_os = "emscripten",
            target_os = "fuchsia",
            target_os = "l4re"
        ),
        link_name = "__errno_location"
    )]
    #[cfg_attr(
        any(
            target_os = "netbsd",
            target_os = "openbsd",
            target_os = "android",
            target_os = "redox",
            target_env = "newlib"
        ),
        link_name = "__errno"
    )]
    #[cfg_attr(
        any(target_os = "solaris", target_os = "illumos"),
        link_name = "___errno"
    )]
    #[cfg_attr(
        any(
            target_os = "macos",
            target_os = "ios",
            target_os = "freebsd",
            target_os = "watchos"
        ),
        link_name = "__error"
    )]
    #[cfg_attr(target_os = "haiku", link_name = "_errnop")]
    fn errno_location() -> *mut c_int;
}

#[cfg(unix)]
#[no_mangle]
pub unsafe extern "C" fn nanosleep(rqtp: *const timespec, rmtp: *mut timespec) -> c_int {
    eprintln!("hook nanosleep works");
    if !rmtp.is_null() {
        (*rmtp).tv_sec = 0;
        (*rmtp).tv_nsec = 0;
    }
    // reset errno
    unsafe { errno_location().write(libc::EINTR) };
    -1
}
