#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
mod lang_items;
mod batch;

#[macro_use]
pub mod sbi;
pub mod sync;
mod trap;

pub use lang_items::handle_panic;
pub use sbi::*;
pub use trap::init as trap_init;
pub use batch::{init as batch_init, print_app_info, run_next_app};

pub fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }

    for addr in sbss as usize..ebss as usize {
        unsafe { (addr as *mut u8).write_volatile(0) }
    }
}
