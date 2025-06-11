use crate::{
    error, error_print, warn, warn_print,
    syscall::*,
    batch::run_next_app
};
use core::arch::global_asm;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Trap, Exception, Interrupt},
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

    unsafe { stvec::write(__alltraps as usize, TrapMode::Direct) }
}

fn handle_exception(ctx: &mut TrapContext, exc: Exception) -> &mut TrapContext {
    use scause::Exception::*;

    match exc {
        UserEnvCall => {
            ctx.sepc += 4;
            ctx[10] = syscall(ctx[17], [ctx[10], ctx[11], ctx[12]]) as usize;

            return ctx;
        },
        StoreFault | StorePageFault => {
            error!("[kernel] PageFault in application, kernel killed it.");
        },
        IllegalInstruction => {
            error!("[kernel] IllegalInstruction in application, kernel killed it.");
        },
        _ => {
            error_print!("Unsupported trap: ");
            warn_print!("Exception({:?}), tval: {:?}", exc, stval::read());
            error!(".");
            warn!("Kernel skips the exception as it is not implemented.");
        }
    }

    run_next_app()
} 

fn handle_interrupt(_ctx: &mut TrapContext, int: Interrupt) -> &mut TrapContext {
    use scause::Interrupt::*;

    match int {
        UserTimer => unimplemented!("There is expected to be a interval switch between processes."),
        _ => {
            error_print!("Unsupported trap: ");
            warn_print!("Interrupt({:?}), tval: {:?}", int, stval::read());
            error!(".");
            warn!("Kernel skips the exception as it is not implemented.");
        }
    }

    run_next_app()
}

#[unsafe(no_mangle)]
pub fn trap_handler(ctx: &mut TrapContext) -> &mut TrapContext {
    let cause = scause::read();
    let trap = cause.cause();

    match trap {
        Trap::Exception(exc) => handle_exception(ctx, exc),
        Trap::Interrupt(int) => handle_interrupt(ctx, int)
    }
}