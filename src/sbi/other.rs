use sbi_rt::*;

/// Shutdown the kernel, which also quit qemu simulator.
pub fn _shutdown(failure: bool) -> ! {
    if failure {
        system_reset(Shutdown, SystemFailure);
    } else {
        system_reset(Shutdown, NoReason);
    }

    unreachable!()
}