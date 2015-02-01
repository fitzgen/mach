//! This module roughly corresponds to `mach/i386/vm_types.h`.

use libc::types::os::arch::c95;

pub type natural_t = c95::c_uint;

pub type user_addr_t = usize;