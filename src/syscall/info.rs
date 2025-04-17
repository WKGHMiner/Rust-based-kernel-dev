use crate::{println, trace};
use core::arch::asm;

pub unsafe fn print_stack_trace() {
    let mut frame_ptr: *const usize;
    unsafe { asm!("mv {}, fp", out(reg) frame_ptr) }

    println!("=== Begin stack tracing ===");
    
    // Previous return address.
    let mut ra: usize;
    // Previous frame pointer.
    let mut fp: usize;
    while !frame_ptr.is_null() {
        unsafe {
            ra = frame_ptr.sub(1).read();
            fp = frame_ptr.sub(2).read();
        }

        trace!("ra: {:?}, fp: {:?}", ra, fp);

        frame_ptr = fp as *const usize;
    }

    println!("=== End stack tracin. ===");
}