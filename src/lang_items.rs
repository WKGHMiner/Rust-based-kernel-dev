use core::panic::PanicInfo;
use crate::{
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

    // Panic message log.
    info_print!("Message: ");
    println!("{}", info.message());

    shutdown!(true);
}

#[cfg(test)]
#[panic_handler]
pub fn handle_panic(info: &PanicInfo) -> ! {
    use crate::warn;

    warn!("Test failed.");
    
    shutdown!(true);
}
