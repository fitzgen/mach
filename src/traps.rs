//! This module corresponds to `mach/mach_traps.h`.

use port::{mach_port_t};

extern "C" {
    /// Return a send right to the caller's task_self port.
    pub fn mach_task_self() -> mach_port_t;
}

#[test]
fn mach_task_self_sanity_test() {
    unsafe {
        let this_task = mach_task_self();
        println!("{:p}", this_task);
    }
}