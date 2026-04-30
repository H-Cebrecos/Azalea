use crate::memory_device::*;

pub struct Cpu {
    regs: [u32; 32],
    pc: u32,
}

impl Cpu {
    pub fn new(initial_pc: u32) -> Self {
        Self {
            pc: initial_pc,
            regs: [0; 32],
        }
    }

    pub fn run(&mut self, mem: &mut impl MemoryDevice) {
        loop {
            self.step(mem);
        }
    }

    fn step(&mut self, mem: &mut impl MemoryDevice) {
        let instr = mem.load(self.pc, AccessSize::Word);
        let opcode = instr & 0x7f;

        match opcode {
            0x33 => {
                // R-type
                let rd = ((instr >> 7) & 0x1f) as usize;
                let rs1 = ((instr >> 15) & 0x1f) as usize;
                let rs2 = ((instr >> 20) & 0x1f) as usize;
                let funct3 = (instr >> 12) & 0x7;
                let funct7 = (instr >> 25) & 0x7f;

                if funct3 == 0x0 && funct7 == 0x00 {
                    // ADD
                    self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
                    println!("executing add");
                }
            }
            _ => panic!("unimplemented opcode"),
        }

        self.pc = self.pc.wrapping_add(4);
    }
}
