//! This module roughly corresponds to `mach/clock_types.h`.

use libc::types::os::arch::c95;

pub type alarm_type_t = c95::c_int;
pub type sleep_type_t = c95::c_int;
pub type clock_id_t = c95::c_int;
pub type clock_flavor_t = c95::c_int;
pub type clock_attr_t = *mut c95::c_int;
pub type clock_res_t = c95::c_int;

#[repr(C)]
struct mach_timespec {
    tv_sec: c95::c_uint,
    tv_nsec: clock_res_t
}
pub type mach_timespec_t = mach_timespec;
