mod cpu;
mod memory_device;

use cpu::Cpu32;
use memory_device::*;

mod instructions;

fn main() {
    let add: u32 = 0b0000000_00010_00001_000_00011_0110011;
    let pause: u32 = 0b0000_0001_0000_00000_000_00000_0001111;

    let mut code = Vec::new();
    code.extend_from_slice(&add.to_le_bytes());
    code.extend_from_slice(&pause.to_le_bytes());

    let mut cpu = Cpu32::new(0);
    let rom = Rom::new(0, &code);
    let ram = Ram::new(0x1000, 1024 * 1024);
    let mut bus = Bus::new();

    bus.add_device(Box::new(rom));
    bus.add_device(Box::new(ram));

    cpu.run(&mut bus);
    println!("Core paused execution by executing a system instruction.")
}
