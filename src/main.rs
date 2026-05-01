mod cpu;
mod memory_device;

use cpu::Cpu32;
use memory_device::*;

mod instructions;

fn main() {
    let instr: u32 = 0b0000000_00010_00001_000_00011_0110011;

    let mut cpu = Cpu32::new(0);
    let rom = Rom::new(0, &instr.to_ne_bytes());
    let ram = Ram::new(0x1000, 1024 * 1024);
    let mut bus = Bus::new();

    bus.add_device(Box::new(rom));
    bus.add_device(Box::new(ram));

    cpu.run(&mut bus);
}
