#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(user::test_runner)]
#![reexport_test_harness_main = "test_main"]
use riscv::register::sstatus::{self, SPP};

#[macro_use]
extern crate user;

#[unsafe(no_mangle)]
fn main() -> i32 {
    debug!("Task 4.");
    info_print!("The kernel would instantly kill this application,");
    info!("as it trys to access priviledged CSR in user mode.");
    
    unsafe { sstatus::set_spp(SPP::User) }

    0
}