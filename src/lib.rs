#[macro_use]
extern crate helix;
extern crate libc;

use std::ffi::CStr;
use libc::c_char;
use helix::ruby;

// Using the Helix library.
ruby! {
    class HelixBenchmark {
        def is_blank(input: String) -> bool {
            input.chars().all(|c| c.is_whitespace())
        }
    }
}

// Using the FFI library.
#[no_mangle]
pub extern fn ffi_is_blank(input_ptr: *const c_char) -> bool {
    let input = unsafe { CStr::from_ptr(input_ptr).to_str().unwrap() };
    input.chars().all(|c| c.is_whitespace())
}
