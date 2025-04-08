#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(user::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::arch::asm;

#[macro_use]
extern crate user;

#[unsafe(no_mangle)]
fn main() -> i32 {
    debug!("Task 3.");
    info!("The kernel would instantly kill this application,");
    info!("as it trys to run priviledged instructions in user mode.");
    
    unsafe {asm! {
        "sret"
    }}

    0
}