//! This module corresponds to `mach/task_info.h`.

use libc::{c_int};
use message::{mach_msg_type_number_t};
use policy::{policy_t};
use time_value::{time_value_t};
use vm_types::{integer_t, vm_size_t};

pub const TASK_EVENTS_INFO: c_int = 2;
pub const TASK_THREAD_TIMES_INFO: c_int = 3;
pub const TASK_BASIC_INFO_32: c_int = 4;
pub const TASK_BASIC_INFO_64: c_int = 5;

#[cfg(target_pointer_width = "64")]
pub const TASK_BASIC_INFO: c_int = TASK_BASIC_INFO_64;
#[cfg(not(target_pointer_width = "64"))]
pub const TASK_BASIC_INFO: c_int = TASK_BASIC_INFO_32;

// Following const are struct sizes / 4 (Hard coded until we get const fn)
pub const TASK_EVENTS_INFO_COUNT: mach_msg_type_number_t = 8;
pub const TASK_THREAD_TIMES_INFO_COUNT: mach_msg_type_number_t = 4;

#[cfg(target_pointer_width = "64")]
pub const TASK_BASIC_INFO_COUNT: mach_msg_type_number_t = 10;
#[cfg(not(target_pointer_width = "64"))]
pub const TASK_BASIC_INFO_COUNT: mach_msg_type_number_t = 8;


#[derive(Debug)]
#[repr(C)]
#[repr(packed)]
pub struct task_basic_info {
  suspend_count: integer_t,
  virtual_size: vm_size_t,
  resident_size: vm_size_t,
  user_time: time_value_t,
  system_time: time_value_t,
  policy: policy_t
}
pub type task_basic_info_t = *mut task_basic_info;

impl task_basic_info {
  pub fn new () -> task_basic_info {
    task_basic_info {
      suspend_count: 0,
      virtual_size: 0,
      resident_size: 0,
      user_time: time_value_t::new(),
      system_time: time_value_t::new(),
      policy: 0
    }
  }
}

#[derive(Debug)]
#[repr(C)]
#[repr(packed)]
pub struct task_events_info {
  faults: integer_t,
  pageins: integer_t,
  cow_faults: integer_t,
  messages_sent: integer_t,
  messages_received: integer_t,
  syscalls_mach: integer_t,
  syscalls_unix: integer_t,
  csw: integer_t
}
pub type task_events_info_t = *mut task_events_info;

impl task_events_info {
  pub fn new() -> task_events_info {
    task_events_info {
      faults: 0,
      pageins: 0,
      cow_faults: 0,
      messages_sent: 0,
      messages_received: 0,
      syscalls_mach: 0,
      syscalls_unix: 0,
      csw: 0
    }
  }
}

#[derive(Debug)]
#[repr(C)]
#[repr(packed)]
pub struct task_thread_times_info {
  pub user_time: time_value_t,
  pub system_time: time_value_t
}
pub type task_thread_times_info_t = *mut task_thread_times_info;

impl task_thread_times_info {
  pub fn new() -> task_thread_times_info {
    task_thread_times_info {
      user_time: time_value_t::new(),
      system_time: time_value_t::new()
    }
  }
}

#[cfg(test)]
mod tests {
  use std::mem::size_of;

  use libc::c_int;

  use message::{mach_msg_type_number_t};
  use task_info;

  #[test]
  fn check_task_basic_info_size() {
    assert_eq!(size_of::<task_info::task_basic_info>() as mach_msg_type_number_t / 4, task_info::TASK_BASIC_INFO_COUNT);
  }

  #[test]
  fn check_task_events_info_size() {
    assert_eq!(size_of::<task_info::task_events_info>() as mach_msg_type_number_t / 4, task_info::TASK_EVENTS_INFO_COUNT);
  }

  #[test]
  fn check_task_thread_times_info_size() {
    assert_eq!(size_of::<task_info::task_thread_times_info>() as mach_msg_type_number_t / 4, task_info::TASK_THREAD_TIMES_INFO_COUNT);
  }
}
