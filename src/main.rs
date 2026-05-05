mod cpu;
mod loader;
mod memory_device;

use cpu::Cpu32;
use memory_device::*;

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
    cpu.run(start, &mut bus);
    println!("Core paused execution by executing a system instruction.");
}
