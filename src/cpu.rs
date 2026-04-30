use crate::instructions::Instruction;
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

    fn u8_sign_extend(value: u8) -> u32 {
        value as u8 as i8 as i32 as u32
    }
    fn u16_sign_extend(value: u16) -> u32 {
        value as u16 as i16 as i32 as u32
    }

    fn step(&mut self, mem: &mut impl MemoryDevice) {
        let instr: Instruction = mem.load(self.pc, AccessSize::Word).into();

        match instr {
            Instruction::Lui { rd, imm } => self.regs[usize::from(rd)] = imm,
            Instruction::Auipc { rd, imm } => self.regs[usize::from(rd)] = imm + self.pc,
            Instruction::Jal { rd, imm } => todo!(),
            Instruction::Jalr { rd, rs1, imm } => todo!(),
            Instruction::Beq { rd, rs1, rs2, imm } => todo!(),
            Instruction::Bne { rd, rs1, rs2, imm } => todo!(),
            Instruction::Blt { rd, rs1, rs2, imm } => todo!(),
            Instruction::Bge { rd, rs1, rs2, imm } => todo!(),
            Instruction::Bltu { rd, rs1, rs2, imm } => todo!(),
            Instruction::Bgeu { rd, rs1, rs2, imm } => todo!(),
            Instruction::Lb { rd, rs1, imm } => {
                self.regs[usize::from(rd)] = Self::u8_sign_extend(mem.load(
                    self.regs[usize::from(rs1)].wrapping_add(imm as u32),
                    AccessSize::Byte,
                ) as u8);
                self.pc = self.pc.wrapping_add(4)
            }
            Instruction::Lh { rd, rs1, imm } => {
                self.regs[usize::from(rd)] = Self::u16_sign_extend(mem.load(
                    self.regs[rs1 as usize].wrapping_add(imm as u32),
                    AccessSize::Half,
                ) as u16);
                self.pc = self.pc.wrapping_add(4)
            }
            Instruction::Lw { rd, rs1, imm } => {
                self.regs[usize::from(rd)] = mem.load(
                    self.regs[rs1 as usize].wrapping_add(imm as u32),
                    AccessSize::Word,
                );
                self.pc = self.pc.wrapping_add(4)
            }
            Instruction::Lbu { rd, rs1, imm } => {
                self.regs[usize::from(rd)] = mem.load(
                    self.regs[rs1 as usize].wrapping_add(imm as u32),
                    AccessSize::Half,
                ) as u8 as u32;
                self.pc = self.pc.wrapping_add(4)
            }
            Instruction::Lhu { rd, rs1, imm } => {
                self.regs[usize::from(rd)] = mem.load(
                    self.regs[rs1 as usize].wrapping_add(imm as u32),
                    AccessSize::Half,
                ) as u16 as u32;
                self.pc = self.pc.wrapping_add(4)
            }
            Instruction::Sb { rs1, rs2, imm } => {
                mem.store(
                    self.regs[usize::from(rs1)] + imm,
                    AccessSize::Byte,
                    self.regs[usize::from(rs2)],
                );
                self.pc = self.pc.wrapping_add(4)
            }
            Instruction::Sh { rs1, rs2, imm } => {
                mem.store(
                    self.regs[usize::from(rs1)] + imm,
                    AccessSize::Half,
                    self.regs[usize::from(rs2)],
                );
                self.pc = self.pc.wrapping_add(4)
            }
            Instruction::Sw { rs1, rs2, imm } => {
                mem.store(
                    self.regs[usize::from(rs1)] + imm,
                    AccessSize::Word,
                    self.regs[usize::from(rs2)],
                );
                self.pc = self.pc.wrapping_add(4)
            }
            Instruction::Addi { rd, rs1, imm } => {
                self.regs[usize::from(rd)] = self.regs[usize::from(rs1)].wrapping_add(imm);
                self.pc = self.pc.wrapping_add(4)
            }
            Instruction::Slli { rd, rs1, shamt } => {
                self.regs[usize::from(rd)] = self.regs[usize::from(rs1)] << shamt;
                self.pc = self.pc.wrapping_add(4)
            }
            Instruction::Srli { rd, rs1, shamt } => {
                self.regs[usize::from(rd)] = self.regs[usize::from(rs1)] >> shamt;
                self.pc = self.pc.wrapping_add(4)
            }
            Instruction::Srai { rd, rs1, shamt } => {
                self.regs[usize::from(rd)] = ((self.regs[usize::from(rs1)] as i32) >> shamt) as u32;
                self.pc = self.pc.wrapping_add(4)
            }
            Instruction::Add { rd, rs1, rs2 } => {
                self.regs[usize::from(rd)] =
                    self.regs[usize::from(rs1)].wrapping_add(self.regs[usize::from(rs2)]);
                self.pc = self.pc.wrapping_add(4);

                println!("ADD"); //TODO: remove this shi
            }
            Instruction::Sub { rd, rs1, rs2 } => {
                self.regs[usize::from(rd)] =
                    self.regs[usize::from(rs1)].wrapping_sub(self.regs[usize::from(rs2)]);
                self.pc = self.pc.wrapping_add(4)
            }
            Instruction::Sll { rd, rs1, rs2 } => {
                self.regs[usize::from(rd)] =
                    self.regs[usize::from(rs1)] << self.regs[usize::from(rs2)];
                self.pc = self.pc.wrapping_add(4)
            }
            Instruction::Slt { rd, rs1, rs2 } => {
                self.regs[rd as usize] =
                    if (self.regs[rs1 as usize] as i32) < (self.regs[rs2 as usize] as i32) {
                        1
                    } else {
                        0
                    };
                self.pc = self.pc.wrapping_add(4)
            }
            Instruction::Sltu { rd, rs1, rs2 } => {
                self.regs[usize::from(rd)] =
                    if self.regs[usize::from(rs1)] < self.regs[usize::from(rs2)] {
                        1
                    } else {
                        0
                    };
                self.pc = self.pc.wrapping_add(4)
            }
            Instruction::Xor { rd, rs1, rs2 } => {
                self.regs[usize::from(rd)] =
                    self.regs[usize::from(rs1)] ^ self.regs[usize::from(rs2)];
                self.pc = self.pc.wrapping_add(4)
            }
            Instruction::Srl { rd, rs1, rs2 } => {
                self.regs[usize::from(rd)] =
                    self.regs[usize::from(rs1)] >> self.regs[usize::from(rs2)];
                self.pc = self.pc.wrapping_add(4)
            }
            Instruction::Sra { rd, rs1, rs2 } => {
                self.regs[usize::from(rd)] =
                    ((self.regs[usize::from(rs1)] as i32) >> self.regs[usize::from(rs2)]) as u32;
                self.pc = self.pc.wrapping_add(4)
            }
            Instruction::Or { rd, rs1, rs2 } => {
                self.regs[usize::from(rd)] =
                    self.regs[usize::from(rs1)] | self.regs[usize::from(rs2)];
                self.pc = self.pc.wrapping_add(4)
            }
            Instruction::And { rd, rs1, rs2 } => {
                self.regs[usize::from(rd)] =
                    self.regs[usize::from(rs1)] & self.regs[usize::from(rs2)];
                self.pc = self.pc.wrapping_add(4)
            }
            Instruction::Fence {
                rd,
                rs1,
                succ,
                pred,
                fm,
            } => todo!(),
            Instruction::FenceTso => todo!(),
            Instruction::Pause => todo!(),
            Instruction::Ecall => todo!(),
            Instruction::Ebreak => todo!(),
            _ => panic!("unimplemented opcode"),
        }

        self.pc = self.pc.wrapping_add(4);
    }
}
