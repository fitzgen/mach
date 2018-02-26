//! This module roughly corresponds to `mach/clock.h`.

pub const clock_MSG_COUNT: ::libc::c_int = 3;

use types::{clock_serv_t, clock_reply_t};
use clock_types::{mach_timespec_t, clock_flavor_t, clock_attr_t, alarm_type_t};
use kern_return::kern_return_t;
use message::mach_msg_type_number_t;

extern "C" {
    pub fn clock_get_time(clock_serv: clock_serv_t,
	                        cur_time: *mut mach_timespec_t)
                          -> kern_return_t;
    pub fn clock_get_attributes(clock_serv: clock_serv_t,
	                              flavor: clock_flavor_t,
	                              clock_attr: clock_attr_t,
	                              clock_attrCnt: *mut mach_msg_type_number_t)
                                -> kern_return_t;
    pub fn clock_alarm(clock_serv: clock_serv_t,
	                     alarm_type: alarm_type_t,
	                     alarm_time: mach_timespec_t,
	                     alarm_port: clock_reply_t)
                       -> kern_return_t;
}
