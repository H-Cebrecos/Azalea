mod cpu;
mod hooks;
mod loader;
mod memory_device;

use cpu::Cpu32;
use memory_device::*;

use crate::{
    hooks::{BranchKind, CpuHooks},
    instructions::Instruction,
};
pub struct BranchTrace {
    next: Option<Box<dyn CpuHooks>>,
}

impl BranchTrace {
    pub fn new() -> Self {
        Self { next: None }
    }
}

impl CpuHooks for BranchTrace {
    fn set_next(&mut self, next: Box<dyn CpuHooks>) {
        self.next = Some(next);
    }

    fn on_instruction(&mut self, pc: u32, instr: &Instruction) {
        if let Some(next) = self.next.as_deref_mut() {
            next.on_instruction(pc, instr);
        }
    }

    fn on_mem_read(&mut self, pc: u32, addr: u32, size: AccessSize, value: u32) {
        if let Some(next) = self.next.as_deref_mut() {
            next.on_mem_read(pc, addr, size, value);
        }
    }

    fn on_mem_write(&mut self, pc: u32, addr: u32, size: AccessSize, value: u32) {
        if let Some(next) = self.next.as_deref_mut() {
            next.on_mem_write(pc, addr, size, value);
        }
    }

    fn on_branch(&mut self, pc: u32, target: u32, taken: bool, _kind: BranchKind) {
        if taken {
            println!("{:08x} -> {:08x}", pc, target);
        } else {
            println!("{:08x} -> x", pc);
        }

        if let Some(next) = self.next.as_deref_mut() {
            next.on_branch(pc, target, taken, _kind);
        }
    }
}

mod instructions;

fn main() {
    let mut cpu = Cpu32::new();
    let ram = Ram::new(0x80000000, 65 * 1024);
    let uart = peripherals::uart::SimpleUart::new(
        0xFF000000,
        peripherals::uart::backends::TcpBackend::bind("127.0.0.1:5555"),
    );

    let mut bus = Bus::new();

    bus.add_device(ram);
    bus.add_device(uart);

    let start = loader::load(
        "F:/lab/rust/Azalea-Emulator/test-app/target/riscv32i-unknown-none-elf/debug/test-app",
        &mut bus,
    );

    let mut hooks = BranchTrace::new();
    //hooks.set_next(Box::new(BranchTrace::new()));

    cpu.run(start, &mut bus, &mut hooks);
    println!("Core paused execution by executing a system instruction.");
}
