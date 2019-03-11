#[macro_use]
extern crate helix;
extern crate libc;
#[macro_use]
extern crate rutie;

use std::ffi::CStr;
use libc::c_char;
use helix::ruby;
use rutie::{Class, Object, RString, Boolean};

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


// Using Rutie.
class!(RutieBlank);

methods!{
    RutieBlank,
    _itself,
    fn rutie_is_blank(input: RString) -> Boolean {
        Boolean::new(input.unwrap().to_str().chars().all(|c| c.is_whitespace()))
    }
}

#[no_mangle]
pub extern fn Init_rutie_blank() {
    Class::new("RutieBlank", None).define(|itself| {
        itself.def_self("rutie_is_blank", rutie_is_blank);
    });
}
