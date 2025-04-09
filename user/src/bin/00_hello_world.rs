#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(user::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[macro_use]
extern crate user;

#[unsafe(no_mangle)]
fn main() -> i32 {
    debug!("Task 0.");

    println!("Hello, world!");

    0
}