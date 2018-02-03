#![allow(bad_style)]

extern crate libc;
extern crate mach;

use mach::boolean::*;
use mach::bootstrap::*;
use mach::clock_types::*;
use mach::kern_return::*;
use mach::mach_port::*;
// FIXME: mach::mach_types is not used,
// can probably be removed:
#[allow(unused_imports)]
use mach::mach_types::*;
use mach::memory_object_types::*;
use mach::message::*;
use mach::port::*;
use mach::structs::*;
use mach::task::*;
use mach::task_info::*;
use mach::thread_act::*;
use mach::thread_status::*;
use mach::traps::*;
use mach::types::*;
use mach::vm::*;
use mach::vm_attributes::*;
use mach::vm_behavior::*;
use mach::vm_inherit::*;
// FIXME: vm_page_size is not used => not tested?
#[allow(unused_imports)]
use mach::vm_page_size::*;
use mach::vm_prot::*;
use mach::vm_purgable::*;
use mach::vm_region::*;
use mach::vm_statistics::*;
use mach::vm_sync::*;
use mach::vm_types::*;

// FIXME: exported by mach::types and mach::mach_types
use mach::types::task_name_t;

// FIXME: these types are not re-exported by mach::types but they are required.
use libc::{c_int, c_uchar, c_uint, clock_t};

include!(concat!(env!("OUT_DIR"), "/all.rs"));
