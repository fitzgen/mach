//! This module corresponds to `mach/time_value.h`.

use vm_types::{integer_t};

const TIME_MICROS_MAX: integer_t = 1000000;

#[derive(Debug)]
#[repr(C)]
#[repr(packed)]
pub struct time_value {
  seconds: integer_t,
  microseconds: integer_t
}
pub type time_value_t = time_value;

impl time_value_t {
  pub fn new() -> time_value {
    time_value {
      seconds: 0,
      microseconds: 0
    }
  }
}

pub fn time_value_add_usec(val: &mut time_value_t, micros: integer_t) {
  val.microseconds = val.microseconds + micros;
  if val.microseconds >= TIME_MICROS_MAX {
    val.microseconds = val.microseconds - TIME_MICROS_MAX;
    val.seconds = val.seconds + 1;
  }
}

pub fn time_value_add(result: &mut time_value_t, addend: &time_value_t) {
  result.microseconds = result.microseconds + addend.microseconds;
  result.seconds = result.seconds + addend.seconds;
  if result.microseconds >= TIME_MICROS_MAX {
    result.microseconds = result.microseconds - TIME_MICROS_MAX;
    result.seconds = result.seconds + 1;
  }
}

#[cfg(test)]
mod tests {
  use time_value::{
    time_value,
    time_value_t,
    time_value_add_usec,
    time_value_add
  };

  #[test]
  fn check_time_value_add_usec() {
    let mut t_val = time_value::new();
    time_value_add_usec(&mut t_val, 900);
    assert!(t_val.seconds == 0 && t_val.microseconds == 900);
  }

  #[test]
  fn check_time_value_add_usec_over() {
    let mut t_val = time_value::new();
    time_value_add_usec(&mut t_val, 1000001);
    assert!(t_val.seconds == 1 && t_val.microseconds == 1);
  }

  #[test]
  fn check_time_value_add() {
    let mut t_val = time_value::new();
    let t_val_other = time_value { seconds: 3, microseconds: 100 };
    time_value_add(&mut t_val, &t_val_other);
    assert!(t_val.seconds == 3 && t_val.microseconds == 100)
  }

  #[test]
  fn check_time_value_add_over() {
    let mut t_val = time_value { seconds: 3, microseconds: 999900 };
    let t_val_other = time_value { seconds: 0, microseconds: 100 };
    time_value_add(&mut t_val, &t_val_other);
    assert!(t_val.seconds == 4 && t_val.microseconds == 0)
  }
}
