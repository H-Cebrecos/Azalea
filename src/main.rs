mod bus;
mod cpu;
mod memory_device;
mod ram;

use cpu::Cpu;
use memory_device::*;
use ram::Ram;

use crate::bus::Bus;

fn main() {
    let mut cpu = Cpu::new(0);
    let mem = Ram::new(0, 1024 * 1024);
    let mut bus = Bus::new();
    bus.add_device(Box::new(mem));
    // add x3, x1, x2
    let instr: u32 = 0b0000000_00010_00001_000_00011_0110011;
    bus.store(0, AccessSize::Word, instr);
    cpu.run(&mut bus);
}
