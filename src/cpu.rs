use crate::instructions::Instruction;
use crate::memory_device::*;
pub struct Cpu32 {
    regs: [u32; 32],
    pc: u32,
}

impl Cpu32 {
    pub fn new(initial_pc: u32) -> Self {
        Self {
            pc: initial_pc,
            regs: [0; 32],
        }
    }

    pub fn run(&mut self, mem: &mut impl MemoryDevice) {
        loop {
            self.pc = self.pc.wrapping_add(self.step(mem));
        }
    }

    fn u8_sign_extend(value: u8) -> u32 {
        value as u8 as i8 as i32 as u32
    }
    fn u16_sign_extend(value: u16) -> u32 {
        value as u16 as i16 as i32 as u32
    }

    fn step(&mut self, mem: &mut impl MemoryDevice) -> u32 {
        let instr: Instruction = mem.load(self.pc, AccessSize::Word).into();
        let mut pc_delta = 4;

        match instr {
            Instruction::Lui { rd, imm } => {
                self.regs[usize::from(rd)] = imm;
            }
            Instruction::Auipc { rd, imm } => {
                self.regs[usize::from(rd)] = imm.wrapping_add(self.pc);
            }
            Instruction::Jal { rd, imm } => {
                self.regs[usize::from(rd)] = self.pc.wrapping_add(4);
                pc_delta = imm;
            }
            Instruction::Jalr { rd, rs1, imm } => {
                self.regs[usize::from(rd)] = self.pc.wrapping_add(4);
                pc_delta =
                    (self.regs[usize::from(rs1)].wrapping_add(imm) & !1).wrapping_sub(self.pc);
            }
            Instruction::Beq { rs1, rs2, imm } => {
                if self.regs[usize::from(rs1)] == self.regs[usize::from(rs2)] {
                    pc_delta = imm;
                }
            }
            Instruction::Bne { rs1, rs2, imm } => {
                if self.regs[usize::from(rs1)] != self.regs[usize::from(rs2)] {
                    pc_delta = imm;
                }
            }
            Instruction::Blt { rs1, rs2, imm } => {
                if (self.regs[usize::from(rs1)] as i32) < (self.regs[usize::from(rs2)] as i32) {
                    pc_delta = imm;
                }
            }
            Instruction::Bge { rs1, rs2, imm } => {
                if (self.regs[usize::from(rs1)] as i32) >= (self.regs[usize::from(rs2)] as i32) {
                    pc_delta = imm;
                }
            }
            Instruction::Bltu { rs1, rs2, imm } => {
                if self.regs[usize::from(rs1)] < self.regs[usize::from(rs2)] {
                    pc_delta = imm;
                }
            }
            Instruction::Bgeu { rs1, rs2, imm } => {
                if self.regs[usize::from(rs1)] >= self.regs[usize::from(rs2)] {
                    pc_delta = imm;
                }
            }
            Instruction::Lb { rd, rs1, imm } => {
                self.regs[usize::from(rd)] = Self::u8_sign_extend(mem.load(
                    self.regs[usize::from(rs1)].wrapping_add(imm),
                    AccessSize::Byte,
                ) as u8);
            }
            Instruction::Lh { rd, rs1, imm } => {
                self.regs[usize::from(rd)] = Self::u16_sign_extend(mem.load(
                    self.regs[usize::from(rs1)].wrapping_add(imm),
                    AccessSize::Half,
                ) as u16);
            }
            Instruction::Lw { rd, rs1, imm } => {
                self.regs[usize::from(rd)] = mem.load(
                    self.regs[usize::from(rs1)].wrapping_add(imm),
                    AccessSize::Word,
                );
            }
            Instruction::Lbu { rd, rs1, imm } => {
                self.regs[usize::from(rd)] = mem.load(
                    self.regs[usize::from(rs1)].wrapping_add(imm),
                    AccessSize::Byte,
                ) as u8 as u32;
            }
            Instruction::Lhu { rd, rs1, imm } => {
                self.regs[usize::from(rd)] = mem.load(
                    self.regs[usize::from(rs1)].wrapping_add(imm),
                    AccessSize::Half,
                ) as u16 as u32;
            }
            Instruction::Sb { rs1, rs2, imm } => {
                mem.store(
                    self.regs[usize::from(rs1)].wrapping_add(imm),
                    AccessSize::Byte,
                    self.regs[usize::from(rs2)],
                );
            }
            Instruction::Sh { rs1, rs2, imm } => {
                mem.store(
                    self.regs[usize::from(rs1)].wrapping_add(imm),
                    AccessSize::Half,
                    self.regs[usize::from(rs2)],
                );
            }
            Instruction::Sw { rs1, rs2, imm } => {
                mem.store(
                    self.regs[usize::from(rs1)].wrapping_add(imm),
                    AccessSize::Word,
                    self.regs[usize::from(rs2)],
                );
            }
            Instruction::Addi { rd, rs1, imm } => {
                self.regs[usize::from(rd)] = self.regs[usize::from(rs1)].wrapping_add(imm);
            }
            Instruction::Slli { rd, rs1, shamt } => {
                self.regs[usize::from(rd)] = self.regs[usize::from(rs1)] << shamt;
            }
            Instruction::Srli { rd, rs1, shamt } => {
                self.regs[usize::from(rd)] = self.regs[usize::from(rs1)] >> shamt;
            }
            Instruction::Srai { rd, rs1, shamt } => {
                self.regs[usize::from(rd)] = ((self.regs[usize::from(rs1)] as i32) >> shamt) as u32;
            }
            Instruction::Add { rd, rs1, rs2 } => {
                self.regs[usize::from(rd)] =
                    self.regs[usize::from(rs1)].wrapping_add(self.regs[usize::from(rs2)]);

                println!("ADD"); //TODO: remove this shi
            }
            Instruction::Sub { rd, rs1, rs2 } => {
                self.regs[usize::from(rd)] =
                    self.regs[usize::from(rs1)].wrapping_sub(self.regs[usize::from(rs2)]);
            }
            Instruction::Sll { rd, rs1, rs2 } => {
                self.regs[usize::from(rd)] =
                    self.regs[usize::from(rs1)] << (self.regs[usize::from(rs2)] & 0x1f);
            }
            Instruction::Slt { rd, rs1, rs2 } => {
                self.regs[rd as usize] =
                    if (self.regs[rs1 as usize] as i32) < (self.regs[rs2 as usize] as i32) {
                        1
                    } else {
                        0
                    };
            }
            Instruction::Sltu { rd, rs1, rs2 } => {
                self.regs[usize::from(rd)] =
                    if self.regs[usize::from(rs1)] < self.regs[usize::from(rs2)] {
                        1
                    } else {
                        0
                    };
            }
            Instruction::Xor { rd, rs1, rs2 } => {
                self.regs[usize::from(rd)] =
                    self.regs[usize::from(rs1)] ^ self.regs[usize::from(rs2)];
            }
            Instruction::Srl { rd, rs1, rs2 } => {
                self.regs[usize::from(rd)] =
                    self.regs[usize::from(rs1)] >> (self.regs[usize::from(rs2)] & 0x1f);
            }
            Instruction::Sra { rd, rs1, rs2 } => {
                self.regs[usize::from(rd)] = ((self.regs[usize::from(rs1)] as i32)
                    >> (self.regs[usize::from(rs2)] & 0x1f))
                    as u32;
            }
            Instruction::Or { rd, rs1, rs2 } => {
                self.regs[usize::from(rd)] =
                    self.regs[usize::from(rs1)] | self.regs[usize::from(rs2)];
            }
            Instruction::And { rd, rs1, rs2 } => {
                self.regs[usize::from(rd)] =
                    self.regs[usize::from(rs1)] & self.regs[usize::from(rs2)];
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
        };

        self.regs[0] = 0;
        return pc_delta;
    }
}
