//! This module corresponds to `mach/mach_port.h`

use kern_return::{kern_return_t};
use port::{mach_port_name_t};
use types::{ipc_space_t};

extern "C" {
    pub fn mach_port_deallocate(task: ipc_space_t,
                                name: mach_port_name_t) -> kern_return_t;
}
