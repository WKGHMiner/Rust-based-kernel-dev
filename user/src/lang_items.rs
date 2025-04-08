use core::panic::PanicInfo;
use crate::println;

#[cfg(not(test))]
#[panic_handler]
pub fn handle_panic(info: &PanicInfo) -> ! {
    use crate::{error, error_print, info_print, info};

    if let Some(location) = info.location() {
        // Panic location log.
        error_print!("Panicked at file [");
        info_print!("{}", location.file());
        error_print!("], line [");
        info_print!("{}", location.line());
        error!("].");
        
        // Panic message log.
        info_print!("Message: ");
        println!("{}", info.message());
    } else {
        error!("Panicked at unknown location.");
        info!("Message: {}", info.message());
    }

    loop {}
}

#[cfg(test)]
#[panic_handler]
pub fn handle_panic(info: &PanicInfo) -> ! {
    println!("Tests failed!");
    println!("{}", info);
    
    loop {}
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests...", tests.len());
    for test in tests {
        test();
    }
    println!("Tests done successfully!");
}
