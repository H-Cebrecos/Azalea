use crate::instructions::bits::rd;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Lui {
        rd: u8,
        imm: u32,
    },
    Auipc {
        rd: u8,
        imm: u32,
    },
    Jal {
        rd: u8,
        imm: u32,
    },
    Jalr {
        rd: u8,
        rs1: u8,
        imm: u32,
    },
    Beq {
        rs1: u8,
        rs2: u8,
        imm: u32,
    },
    Bne {
        rs1: u8,
        rs2: u8,
        imm: u32,
    },
    Blt {
        rs1: u8,
        rs2: u8,
        imm: u32,
    },
    Bge {
        rs1: u8,
        rs2: u8,
        imm: u32,
    },
    Bltu {
        rs1: u8,
        rs2: u8,
        imm: u32,
    },
    Bgeu {
        rs1: u8,
        rs2: u8,
        imm: u32,
    },
    Lb {
        rd: u8,
        rs1: u8,
        imm: u32,
    },
    Lh {
        rd: u8,
        rs1: u8,
        imm: u32,
    },
    Lw {
        rd: u8,
        rs1: u8,
        imm: u32,
    },
    Lbu {
        rd: u8,
        rs1: u8,
        imm: u32,
    },
    Lhu {
        rd: u8,
        rs1: u8,
        imm: u32,
    },
    Sb {
        rs1: u8,
        rs2: u8,
        imm: u32,
    },
    Sh {
        rs1: u8,
        rs2: u8,
        imm: u32,
    },
    Sw {
        rs1: u8,
        rs2: u8,
        imm: u32,
    },
    Addi {
        rd: u8,
        rs1: u8,
        imm: u32,
    },
    Slli {
        rd: u8,
        rs1: u8,
        shamt: u32,
    },
    Srli {
        rd: u8,
        rs1: u8,
        shamt: u32,
    },
    Srai {
        rd: u8,
        rs1: u8,
        shamt: u32,
    },
    Add {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Sub {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Sll {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Slt {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Sltu {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Xor {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Srl {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Sra {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Or {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    And {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Fence {
        rd: u8,
        rs1: u8,
        succ: u8,
        pred: u8,
        fm: u8,
    },
    FenceTso,
    Pause,
    Ecall,
    Ebreak,
}

mod opcode {
    pub const ADD: u8 = 0x33;
}

mod bits {
    #[inline]
    pub fn rd(x: u32) -> u8 {
        ((x >> 7) & 0x1f) as u8
    }

    #[inline]
    pub fn rs1(x: u32) -> u8 {
        ((x >> 15) & 0x1f) as u8
    }

    #[inline]
    pub fn rs2(x: u32) -> u8 {
        ((x >> 20) & 0x1f) as u8
    }

    #[inline]
    pub fn funct3(x: u32) -> u32 {
        (x >> 12) & 0x7
    }

    #[inline]
    pub fn funct7(x: u32) -> u32 {
        (x >> 25) & 0x7f
    }
}

impl From<u32> for Instruction {
    fn from(value: u32) -> Self {
        use bits::*;
        let opcode = (value & 0x7f) as u8;

        match opcode {
            opcode::ADD => {
                // R-type

                if funct3(value) == 0x0 && funct7(value) == 0x00 {
                    return Instruction::Add {
                        rd: rd(value),
                        rs1: rs1(value),
                        rs2: rs2(value),
                    };
                }
                todo!()
            }
            _ => todo!(),
        }
    }
}

impl From<Instruction> for u32 {
    fn from(value: Instruction) -> Self {
        todo!()
    }
}
