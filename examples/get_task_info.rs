//! A script to dump task & thread information about current self task

extern crate libc;
extern crate mach;

use mach::kern_return::{KERN_SUCCESS};
use mach::task_info::{
  TASK_EVENTS_INFO,
  TASK_EVENTS_INFO_COUNT,
  TASK_THREAD_TIMES_INFO,
  TASK_THREAD_TIMES_INFO_COUNT,
  TASK_BASIC_INFO_COUNT,
  TASK_BASIC_INFO,
  task_basic_info, 
  task_basic_info_t,
  task_events_info, 
  task_events_info_t, 
  task_thread_times_info,
  task_thread_times_info_t
};
use mach::task::task_info;
use mach::traps::{mach_task_self};

fn main() {
  println!("");
  println!("Getting information about the current task...");
  let mut basic_info = task_basic_info::new();
  let basic_info_ptr = (&mut basic_info as task_basic_info_t) as libc::uintptr_t;
  let mut count = TASK_BASIC_INFO_COUNT;
  let kernal_response = unsafe {
    task_info(mach_task_self(), TASK_BASIC_INFO, basic_info_ptr, &mut count)
  };
  match kernal_response {
    KERN_SUCCESS => {
      println!("Successfully read task basic info: ");
      println!("{:?}", basic_info);
    },
    _ => println!("Call failed - kernal responded with {}", kernal_response)
  };
  println!("--");

  println!("Getting information about the current task events...");
  let mut event_info = task_events_info::new();
  let event_info_ptr = (&mut event_info as task_events_info_t) as libc::uintptr_t;
  let mut task_event_count = TASK_EVENTS_INFO_COUNT;
  let kernal_response = unsafe {
    task_info(mach_task_self(), TASK_EVENTS_INFO, event_info_ptr, &mut task_event_count)
  };
  match kernal_response {
    KERN_SUCCESS => {
      println!("Successfully read task event info: ");
      println!("{:?}", event_info);
    },
    _ => println!("Call failed - kernal responded with {}", kernal_response)
  };
  println!("--");

  println!("Getting information about the current task thread times...");
  let mut thread_times = task_thread_times_info::new();
  let thread_times_ptr = (&mut thread_times as task_thread_times_info_t) as libc::uintptr_t;
  let mut thread_times_count = TASK_THREAD_TIMES_INFO_COUNT;
  let kernal_response = unsafe {
    task_info(mach_task_self(), TASK_THREAD_TIMES_INFO, thread_times_ptr, &mut thread_times_count)
  };
  match kernal_response {
    KERN_SUCCESS => {
      println!("Successfully read task thread times: ");
      println!("{:?}", thread_times);
    },
    _ => println!("Call failed - kernal responded with {}", kernal_response)
  };
}
