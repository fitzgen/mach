//! This module corresponds to `mach/i386/boolean.h`.

use libc::types::os::arch::c95;

#[cfg(target_arch = "x86_64")]
pub type boolean_t = c95::c_uint;

#[cfg(not(target_arch = "x86_64"))]
pub type boolean_t = c95::c_int;
