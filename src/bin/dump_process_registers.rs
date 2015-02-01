//! A script to read and dump to stdout the current register values of a
//! process.

#![allow(unstable)]

extern crate libc;
extern crate mach;

use std::io;
use std::mem;

use libc::types::os::arch::c95;

use mach::kern_return::{KERN_SUCCESS};
use mach::port::{mach_port_name_t};
use mach::traps::{mach_task_self, task_for_pid};

fn read_int() -> Result<c95::c_int, ()> {
    let mut stdin = io::stdin();
    let line = stdin.read_line().ok().unwrap();
    let mut value : c95::c_int = 0;
    for c in line.chars().take_while(|&c| c != '\n') {
        if let Some(d) = c.to_digit(10) {
            value = value * 10 + (d as c95::c_int);
        } else {
            return Err(());
        }
    }
    return Ok(value);
}

fn main() {
    print!("Enter pid: ");
    let pid = match read_int() {
        Ok(v) => v,
        Err(_) => {
            println!("Bad pid!");
            return;
        },
    };

    let task : mach_port_name_t = 0;
    unsafe {
        let rv = task_for_pid(mach_task_self() as mach_port_name_t,
                              pid,
                              mem::transmute(&task));
        if rv != KERN_SUCCESS {
            println!("Did not succeed in getting task for pid {}", pid);
            println!("kern_return_t error {}", rv);
            println!("");
            pringln!("Did you forget to run with 'sudo'? This script will");
            println!("probably fail without it.");
            return;
        }
    }

    println!("task = 0x{:x}", task);

    println!("Success!");
}