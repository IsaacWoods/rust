#![deny(unsafe_op_in_unsafe_fn)]

use crate::io as std_io;
use crate::os::raw::c_char;

pub use crate::sys_common::os_str_bytes as os_str;

pub mod alloc;
pub mod args;
pub mod cmath;
pub mod condvar;
pub mod env;
pub mod fs;
pub mod io;
pub mod mutex;
pub mod net;
pub mod os;
#[path = "../unix/path.rs"]
pub mod path;
pub mod pipe;
pub mod process;
pub mod rwlock;
pub mod stack_overflow;
pub mod stdio;
pub mod thread;
#[cfg(target_thread_local)]
pub mod thread_local_dtor;
pub mod thread_local_key;
pub mod time;

#[cfg(not(test))]
pub fn init() {
    alloc::ALLOCATOR.lock().init();
}

pub fn unsupported<T>() -> std_io::Result<T> {
    Err(unsupported_err())
}

pub fn unsupported_err() -> std_io::Error {
    std_io::Error::new(std_io::ErrorKind::Other, "operation not supported on this platform")
}

pub fn decode_error_kind(_code: i32) -> crate::io::ErrorKind {
    crate::io::ErrorKind::Other
}

pub fn abort_internal() -> ! {
    // TODO: this should call an `exit` syscall or something instead
    core::intrinsics::abort();
}

/// This is called by the panic runtime to abort.
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn __rust_abort() {
    abort_internal()
}

pub fn hashmap_random_keys() -> (u64, u64) {
    (1, 2)
}

// This enum is used as the storage for a bunch of types which can't actually
// exist.
// TODO: why can't we use the never type?
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Void {}

pub unsafe fn strlen(mut s: *const c_char) -> usize {
    // SAFETY: The caller must guarantee `s` points to a valid 0-terminated string.
    unsafe {
        let mut n = 0;
        while *s != 0 {
            n += 1;
            s = s.offset(1);
        }
        n
    }
}

pub mod memchr {
    pub use core::slice::memchr::{memchr, memrchr};
}

// TODO: document how this differs from normal platforms
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // This resolves to `std::rt::lang_start`
    extern "C" {
        fn main(argc: isize, argc: *const *const c_char) -> i32;
    }

    let result = unsafe { main(0, 0x0 as *const *const c_char) };
    // TODO: actually exit the program
    loop {}
}
