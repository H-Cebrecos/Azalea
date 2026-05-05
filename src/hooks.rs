use crate::{instructions::Instruction, memory_device::AccessSize};

#[derive(Debug, Copy, Clone)]
pub enum BranchKind {
    TODO,
}

pub trait CpuHooks {
    fn set_next(&mut self, _next: Box<dyn CpuHooks>) {}

    fn on_instruction(&mut self, pc: u32, instr: &Instruction);

    fn on_mem_read(&mut self, pc: u32, addr: u32, size: AccessSize, value: u32);

    fn on_mem_write(&mut self, pc: u32, addr: u32, size: AccessSize, value: u32);

    fn on_branch(&mut self, pc: u32, target: u32, taken: bool, kind: BranchKind);
}
