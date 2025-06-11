use core::panic::PanicInfo;
use crate::{
    syscall::print_stack_trace,
    error, error_print, info_print, println, shutdown
};

#[cfg(not(test))]
#[panic_handler]
pub fn handle_panic(info: &PanicInfo) -> ! {
    // Panic location log.
    if let Some(location) = info.location() {
        error_print!("Panicked at file [");
        info_print!("{}", location.file());
        error_print!("], line [");
        info_print!("{}", location.line());
        error!("].");
    } else {
        error!("Panicked at unknown location.");
    }

    unsafe { print_stack_trace() }

    // Panic message log.
    info_print!("Message: ");
    println!("{}", info.message());

    shutdown!(true);
}

#[cfg(test)]
#[panic_handler]
pub fn handle_panic(info: &PanicInfo) -> ! {
    error!("Test failed.");
    unsafe { print_stack_trace() }
    
    shutdown!(true);
}
