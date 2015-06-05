//! This module corresponds to `mach/port.h`

use libc::types::os::arch::c95;

use vm_types::{natural_t};

#[repr(C)]
pub type mach_port_name_t = natural_t;

#[repr(C)]
struct ipc_port;

#[repr(C)]
pub type ipc_port_t = *mut ipc_port;

pub type mach_port_t = c95::c_uint;

pub const MACH_PORT_NULL: mach_port_t = 0;
