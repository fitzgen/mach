//! This module roughly corresponds to `mach/error.h`.

use crate::kern_return::kern_return_t;

pub type mach_error_t = kern_return_t;
//pub type mach_error_fn_t = Option<unsafe extern fn() -> mach_error_t>;
//pub const err_none: mach_error_t = 0;
pub const ERR_SUCCESS: mach_error_t = 0;
//pub const ERR_ROUTINE_NIL: mach_error_fn_t = None;

pub const fn err_system(x: i32) -> i32 {
    (x & 0x3f) << 26
}

pub const fn err_sub(x: i32) -> i32 {
    (x & 0xfff) << 14
}

pub const fn err_get_system(err: mach_error_t) -> i32 {
    (err >> 26) & 0x3f
}

pub const fn err_get_sub(err: mach_error_t) -> i32 {
    (err >> 14) & 0xfff
}

pub const fn err_get_code(err: mach_error_t) -> i32 {
    err & 0x3fff
}

pub const system_emask: i32 = err_system(0x3f);
pub const sub_emask: i32 = err_sub(0xfff);
pub const code_emask: i32 = 0x3fff;

// Major error sytems
/// Kernel
pub const err_kern: i32 = err_system(0x0);
/// User space library
pub const err_us: i32 = err_system(0x1);
/// User space servers
pub const err_server: i32 = err_system(0x2);
/// Old IPC errors
pub const err_ipc: i32 = err_system(0x3);
/// Mach-IPC errors
pub const err_mach_ipc: i32 = err_system(0x4);
/// Distributed IPC
pub const err_dipc: i32 = err_system(0x7);
/// User defined errors
pub const err_local: i32 = err_system(0x3e);
/// (Compatibility) Mach-IPC errors
pub const err_ipc_compat: i32 = err_system(0x3f);

pub const err_max_system: i32 = 0x3f;

/// Unix errors get lumped into one subsystem
pub const fn unix_err(errno: i32) -> mach_error_t {
    err_kern | err_sub(3) | errno
}