//! This module corresponds to `mach/mach_init.h`.

use port::mach_port_t;
use mach_types::{thread_port_t, host_t};
use vm_types::vm_size_t;
use kern_return::kern_return_t;

extern "C" {
    pub fn mach_host_self() -> mach_port_t;
    pub fn mach_thread_self() -> thread_port_t;
    pub fn host_page_size(host: host_t, size: *mut vm_size_t) -> kern_return_t;
}

#[cfg(test)]
mod tests {
    use mach_init::*;
    use port::*;

    #[test]
    fn mach_host_self_test() {
        unsafe {
            let host = mach_host_self();
            assert!(host != 0);
        }
    }

    #[test]
    fn mach_thread_self_test() {
        unsafe {
            let this_thread = mach_thread_self();
            assert!(this_thread != MACH_PORT_NULL);
            assert!(this_thread != MACH_PORT_DEAD);
        }
    }

    #[test]
    fn host_page_size_test() {
        unsafe {
            let mut ps: vm_size_t = 0;
            assert!(0 == host_page_size(mach_host_self(), &mut ps));
            assert!(ps > 0);
        };
    }
}
