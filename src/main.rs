#![no_std]
#![no_main]
use core::arch::global_asm;
use os::*;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

/// This main function is only used for executing some regular jobs.
/// It must be called within [`rust_main`].
fn main() {
    trap_init();
    batch_init();
    run_next_app();
}

/// The very entry point of Rust program.
#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    log_info();

    main();
    
    shutdown!(false);
}

/// Logs some important memory layout information to the console,
/// including sections like `.text`, `.data` and so on.
fn log_info() {
    unsafe extern "C" {
        safe fn stext(); // begin addr of text segment
        safe fn etext(); // end addr of text segment
        safe fn srodata(); // start addr of Read-Only data segment
        safe fn erodata(); // end addr of Read-Only data ssegment
        safe fn sdata(); // start addr of data segment
        safe fn edata(); // end addr of data segment
        safe fn sbss(); // start addr of BSS segment
        safe fn ebss(); // end addr of BSS segment
        safe fn boot_stack_lower_bound(); // stack lower bound
        safe fn boot_stack_top(); // stack top
    }

    println!("[kernel] Hello, world!");
    trace!(
        "[kernel] .text [{:#x}, {:#x})",
        stext as usize, etext as usize
    );
    debug!(
        "[kernel] .rodata [{:#x}, {:#x})",
        srodata as usize, erodata as usize
    );
    info!(
        "[kernel] .data [{:#x}, {:#x})",
        sdata as usize, edata as usize
    );
    warn!(
        "[kernel] boot_stack top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top as usize, boot_stack_lower_bound as usize
    );
    error!("[kernel] .bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
}
