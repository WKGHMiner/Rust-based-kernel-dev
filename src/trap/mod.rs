use crate::{
    error,
    sbi::*,
    batch::run_next_app
};
use core::arch::global_asm;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Trap},
    stval, stvec
};

// Include section.
mod ctx;

// Export section.
pub use ctx::TrapContext;

global_asm!(include_str!("trap.S"));

pub fn init() {
    unsafe extern "C" {
        fn __alltraps();
    }

    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

#[unsafe(no_mangle)]
pub fn trap_handler(ctx: &mut TrapContext) -> &mut TrapContext {
    let cause = scause::read();
    let tval = stval::read();

    match cause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            ctx.sepc += 4;
            ctx[10] = syscall(ctx[17], [ctx[10], ctx[11], ctx[12]]) as usize;

            return ctx;
        },
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            error!("[kernel] PageFault in application, kernel killed it.");
        },
        Trap::Exception(Exception::IllegalInstruction) => {
            error!("[kernel] IllegalInstruction in application, kernel killed it.");
        },
        _ => unimplemented!("Unsupported trap: {:?}, tval: {:?}.", cause.cause(), tval)
    }

    run_next_app()
}