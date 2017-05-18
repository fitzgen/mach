//! This module corresponds to `mach/task.defs`.

use libc::c_int;
use libc::uintptr_t;
use kern_return::{kern_return_t};
use message::{mach_msg_type_number_t};
use port::{mach_port_t};
use types::{task_t, thread_act_array_t};

pub type task_special_port_t = ::std::os::raw::c_int;

pub const TASK_KERNEL_PORT:    task_special_port_t = 1;
pub const TASK_HOST_PORT:      task_special_port_t = 2;
pub const TASK_NAME_PORT:      task_special_port_t = 3;
pub const TASK_BOOTSTRAP_PORT: task_special_port_t = 4;

extern "C" {
    pub fn task_resume(target_task: task_t) -> kern_return_t;
    pub fn task_suspend(target_task: task_t) -> kern_return_t;
    pub fn task_get_special_port(task: task_t,
                                 which_port: task_special_port_t,
                                 special_port: *mut mach_port_t) -> kern_return_t;
    pub fn task_threads(target_task: task_t,
                        act_list: *mut thread_act_array_t,
                        act_list_cnt: *mut mach_msg_type_number_t) -> kern_return_t;
    pub fn task_info(target_task: task_t,
                        flavor: c_int,
                        //task_info: task_basic_info_t,
                        // uintptr_t type
                        task_info: uintptr_t,
                        task_info_count: *mut mach_msg_type_number_t) -> kern_return_t;
}
