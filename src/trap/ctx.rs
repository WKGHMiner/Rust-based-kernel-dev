use core::ops::{Index, IndexMut};
use riscv::register::sstatus::{self, Sstatus, SPP};

#[repr(C)]
pub struct TrapContext {
    pub regs: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize
}

impl TrapContext {
    pub fn new(entry: usize, sp: usize) -> Self {
        let mut status = sstatus::read();
        status.set_spp(SPP::User);

        let mut ctx = Self {
            regs: [0usize; 32],
            sstatus: status,
            sepc: entry
        };
        ctx.set_stack_pointer(sp);

        ctx
    }

    pub fn set_stack_pointer(&mut self, ptr: usize) {
        self.regs[2] = ptr;
    }
}

impl Index<usize> for TrapContext {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.regs[index]
    }
}

impl IndexMut<usize> for TrapContext {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.regs[index]
    }
}