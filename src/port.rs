//! This module corresponds to `mach/port.h`

#[repr(C)]
struct ipc_port;

#[repr(C)]
pub type ipc_port_t = *mut ipc_port;

#[repr(C)]
pub type mach_port_t = ipc_port_t;