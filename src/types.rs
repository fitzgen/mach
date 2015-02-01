//! This module corresponds to `mach/mach_types.h`

use port::{mach_port_t};

pub type task_t = mach_port_t;

#[repr(C)]
struct thread;

pub type thread_t = *mut thread;
pub type thread_act_t = mach_port_t;
pub type thread_act_array_t = *mut thread_act_t;