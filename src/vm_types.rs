//! This module roughly corresponds to `mach/i386/vm_types.h`.

use libc::types::os::arch::c95;

pub type natural_t = c95::c_uint;

pub type user_addr_t = usize;

pub type mach_vm_address_t = u64;
pub type mach_vm_offset_t  = u64;
pub type mach_vm_size_t    = u64;
pub type vm_map_offset_t   = u64;
pub type vm_map_address_t  = u64;
pub type vm_map_size_t     = u64;

pub type mach_port_context_t = mach_vm_address_t;
