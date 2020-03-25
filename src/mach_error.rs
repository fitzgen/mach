//! This module roughly corresponds to `mach/mach_error.h`.

use crate::error::mach_error_t;
use libc::c_char;

extern "C" {
    pub fn mach_error_string(error_value: mach_error_t) -> *mut c_char;
    pub fn mach_error(s: *mut c_char, error_value: mach_error_t);
    pub fn mach_error_type(error_value: mach_error_t) -> *mut c_char;
}