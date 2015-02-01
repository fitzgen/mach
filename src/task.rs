//! This module corresponds to `mach/task.defs`.

use kern_return::{kern_return_t};
use message::{mach_msg_type_number_t};
use types::{task_t, thread_act_array_t};

extern "C" {
    pub fn task_threads(target_task: task_t,
                        act_list: *mut thread_act_array_t,
                        act_list_cnt: *mut mach_msg_type_number_t) -> kern_return_t;
}