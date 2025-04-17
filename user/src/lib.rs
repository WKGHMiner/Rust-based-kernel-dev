#![no_std]
#![no_main]
#![feature(linkage, custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
use syscall::*;

#[macro_use]
pub mod console;
mod syscall;
mod lang_items;

pub use lang_items::{handle_panic, test_runner};

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
pub extern "C" fn _start() -> ! {
    clear_bss();

    exit(main());

    unreachable!("Unreachable code after system exit return.");
}

#[linkage = "weak"]
#[unsafe(no_mangle)]
fn main() -> i32 {
    panic!("No such main function.")
}

fn clear_bss() {
    unsafe extern "C" {
        fn start_bss();
        fn end_bss();
    }

    for addr in start_bss as usize..end_bss as usize {
        unsafe { (addr as *mut u8).write_volatile(0) }
    }
}

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}

pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}
