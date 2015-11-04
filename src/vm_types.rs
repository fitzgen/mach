//! This module roughly corresponds to `mach/i386/vm_types.h`.

pub type natural_t = ::libc::c_uint;

pub type user_addr_t = usize;

pub type mach_vm_address_t = u64;
pub type mach_vm_offset_t  = u64;
pub type mach_vm_size_t    = u64;
pub type vm_map_offset_t   = u64;
pub type vm_map_address_t  = u64;
pub type vm_map_size_t     = u64;

pub type mach_port_context_t = mach_vm_address_t;
