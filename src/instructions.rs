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
        rd: u8,
        rs1: u8,
        rs2: u8,
        imm: u32,
    },
    Bne {
        rd: u8,
        rs1: u8,
        rs2: u8,
        imm: u32,
    },
    Blt {
        rd: u8,
        rs1: u8,
        rs2: u8,
        imm: u32,
    },
    Bge {
        rd: u8,
        rs1: u8,
        rs2: u8,
        imm: u32,
    },
    Bltu {
        rd: u8,
        rs1: u8,
        rs2: u8,
        imm: u32,
    },
    Bgeu {
        rd: u8,
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
impl From<u32> for Instruction {
    fn from(value: u32) -> Self {
        let opcode = (value & 0x7f) as u8;

        match opcode {
            opcode::ADD => {
                // R-type
                let rd = ((value >> 7) & 0x1f) as u8;
                let rs1 = ((value >> 15) & 0x1f) as u8;
                let rs2 = ((value >> 20) & 0x1f) as u8;
                let funct3 = (value >> 12) & 0x7;
                let funct7 = (value >> 25) & 0x7f;
                if funct3 == 0x0 && funct7 == 0x00 {
                    return Instruction::Add { rd, rs1, rs2 };
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
