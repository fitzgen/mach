//! This module corresponds to `mach/time_value.h`.

use vm_types::{integer_t};

#[derive(Debug)]
#[repr(C)]
#[repr(packed)]
pub struct time_value {
  seconds: integer_t,
  microseconds: integer_t
}
pub type time_value_t = time_value;

impl Default for time_value_t {
  fn default() -> time_value {
    time_value {
      seconds: 0,
      microseconds: 0
    }
  }
}
