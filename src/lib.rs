#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![cfg_attr(not(feature = "use_std"), no_std)]
#![cfg_attr(feature = "unstable", feature(repr_packed))]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        clippy::stutter,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        clippy::trivially_copy_pass_by_ref
    )
)]

#[cfg(feature = "use_std")]
extern crate core;

extern crate libc;

pub mod boolean;
pub mod bootstrap;
pub mod clock;
pub mod clock_priv;
pub mod clock_reply;
pub mod clock_types; // TODO: test
pub mod dyld_kernel;
// pub mod error; // TODO
pub mod exc;
pub mod exception_types;
pub mod kern_return;
pub mod mach_init;
pub mod mach_port;
pub mod mach_types;
pub mod memory_object_types;
pub mod message;
pub mod port;
pub mod structs;
pub mod task;
pub mod task_info;
pub mod thread_act;
pub mod thread_status;
pub mod traps;
pub mod vm;
pub mod vm_attributes;
pub mod vm_behavior;
pub mod vm_inherit;
pub mod vm_page_size;
pub mod vm_prot;
pub mod vm_purgable;
pub mod vm_region;
pub mod vm_statistics;
pub mod vm_sync;
pub mod vm_types;
