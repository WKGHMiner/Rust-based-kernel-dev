#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(user::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::ptr::null_mut;

#[macro_use]
extern crate user;

#[unsafe(no_mangle)]
fn main() -> i32 {
    debug!("Task 1.");
    info!("This application trys to write value on a null pointer.");
    info!("The kernel will terminate this undefined behaviour.");

    unsafe { null_mut::<u8>().write_volatile(1) }

    0
}