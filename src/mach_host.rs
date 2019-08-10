//! This module corresponds to `mach/mach_host.h`

use kern_return::kern_return_t;
use mach_types::{host_t, clock_serv_t};
use clock_types::clock_id_t;

extern "C" {
    pub fn host_get_clock_service(host: host_t, clock_id: clock_id_t, clock_serv: *mut clock_serv_t) -> kern_return_t;
}
