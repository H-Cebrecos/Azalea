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
    Slti {
        rd: u8,
        rs1: u8,
        imm: u32,
    },
    Sltiu {
        rd: u8,
        rs1: u8,
        imm: u32,
    },
    Xori {
        rd: u8,
        rs1: u8,
        imm: u32,
    },
    Ori {
        rd: u8,
        rs1: u8,
        imm: u32,
    },
    Andi {
        rd: u8,
        rs1: u8,
        imm: u32,
    },
    Slli {
        rd: u8,
        rs1: u8,
        shamt: u8,
    },
    Srli {
        rd: u8,
        rs1: u8,
        shamt: u8,
    },
    Srai {
        rd: u8,
        rs1: u8,
        shamt: u8,
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

#[allow(unused)]
mod opcode {
    pub const LOAD: u8 = 0b0_00_000_11;
    pub const STORE: u8 = 0b0_01_000_11;
    pub const MADD: u8 = 0b0_10_000_11;
    pub const BRANCH: u8 = 0b0_11_000_11;

    pub const LOAD_FP: u8 = 0b0_00_001_11;
    pub const STORE_FP: u8 = 0b0_01_001_11;
    pub const MSUB: u8 = 0b0_10_001_11;
    pub const JALR: u8 = 0b0_11_001_11;

    pub const CUSTOM_0: u8 = 0b0_00_010_11;
    pub const CUSTOM_1: u8 = 0b0_01_010_11;
    pub const NMSUB: u8 = 0b0_10_010_11;
    pub const RESERVED: u8 = 0b0_11_010_11;

    pub const MISC_MEM: u8 = 0b0_00_011_11;
    pub const AMO: u8 = 0b0_01_011_11;
    pub const NMADD: u8 = 0b0_10_011_11;
    pub const JAL: u8 = 0b0_11_011_11;

    pub const OP_IMM: u8 = 0b0_00_100_11;
    pub const OP: u8 = 0b0_01_100_11;
    pub const OP_FP: u8 = 0b0_10_100_11;
    pub const SYSTEM: u8 = 0b0_11_100_11;

    pub const AUIPC: u8 = 0b0_00_101_11;
    pub const LUI: u8 = 0b0_01_101_11;
    pub const OP_V: u8 = 0b0_10_101_11;
    pub const OP_VE: u8 = 0b0_11_101_11;

    pub const OP_IMM_32: u8 = 0b0_00_110_11;
    pub const OP_32: u8 = 0b0_01_110_11;
    pub const CUSTOM_2: u8 = 0b0_10_110_11;
    pub const CUSTOM_3: u8 = 0b0_11_110_11;
}

mod fields {
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

    #[inline]
    pub fn i_type_imm(x: u32) -> u32 {
        ((x as i32) >> 20) as u32
    }

    #[inline]
    pub fn s_type_imm(x: u32) -> u32 {
        let hi = ((x as i32) >> 25) as u32; // sign-extended imm[11:5]
        let lo = (x >> 7) & 0x1F; // imm[4:0]
        (hi << 5) | lo
    }

    #[inline]
    pub fn b_type_imm(x: u32) -> u32 {
        let hi = ((x as i32) >> 31) as u32; // sign-extended imm[12]
        let b11 = ((x >> 7) & 0x1) << 11; // imm[11]
        let b10_5 = ((x >> 25) & 0x3F) << 5; // imm[10:5]
        let b4_1 = ((x >> 8) & 0xF) << 1; // imm[4:1]

        (hi << 12) | b11 | b10_5 | b4_1
    }

    #[inline]
    pub fn j_type_imm(x: u32) -> u32 {
        let hi = ((x as i32) >> 31) as u32; // sign-extended imm[20]
        let b19_12 = ((x >> 12) & 0xFF) << 12; // imm[19:12]
        let b11 = ((x >> 20) & 0x1) << 11; // imm[11]
        let b10_1 = ((x >> 21) & 0x3FF) << 1; // imm[10:1]

        (hi << 20) | b19_12 | b11 | b10_1
    }

    #[inline]
    pub fn u_type_imm(x: u32) -> u32 {
        x & 0xFFFFF000
    }
}

impl From<u32> for Instruction {
    fn from(value: u32) -> Self {
        let opcode = (value & 0x7f) as u8;
        let rd = fields::rd(value);
        let rs1 = fields::rs1(value);
        let rs2 = fields::rs2(value);
        let f3 = fields::funct3(value);
        let f7 = fields::funct7(value);
        match opcode {
            opcode::LOAD => {
                let imm = fields::i_type_imm(value);
                match f3 {
                    0x0 => Instruction::Lb { rd, rs1, imm },
                    0x1 => Instruction::Lh { rd, rs1, imm },
                    0x2 => Instruction::Lw { rd, rs1, imm },
                    0x4 => Instruction::Lbu { rd, rs1, imm },
                    0x5 => Instruction::Lhu { rd, rs1, imm },
                    _ => panic!("illegal load"),
                }
            }
            opcode::STORE => {
                let imm = fields::s_type_imm(value);
                match f3 {
                    0x0 => Instruction::Sb { rs1, rs2, imm },
                    0x1 => Instruction::Sh { rs1, rs2, imm },
                    0x2 => Instruction::Sw { rs1, rs2, imm },
                    _ => panic!("illegal store"),
                }
            }
            opcode::BRANCH => {
                let imm = fields::b_type_imm(value);
                match f3 {
                    0x0 => Instruction::Beq { rs1, rs2, imm },
                    0x1 => Instruction::Bne { rs1, rs2, imm },
                    0x4 => Instruction::Blt { rs1, rs2, imm },
                    0x5 => Instruction::Bge { rs1, rs2, imm },
                    0x6 => Instruction::Bltu { rs1, rs2, imm },
                    0x7 => Instruction::Bgeu { rs1, rs2, imm },
                    _ => panic!("illegal branch"),
                }
            }
            opcode::JALR => Instruction::Jalr {
                rd,
                rs1,
                imm: fields::i_type_imm(value),
            },
            opcode::MISC_MEM => {
                if f3 == 0 {
                    if rd == 0 && rs1 == 0 {
                        // PAUSE encoding check (standard pattern)
                        if fields::i_type_imm(value) == 0x0010 {
                            return Instruction::Pause;
                        }
                    }
                }
                todo!();
            }
            opcode::JAL => Instruction::Jal {
                rd,
                imm: fields::j_type_imm(value),
            },
            opcode::OP_IMM => {
                let imm = fields::i_type_imm(value);
                match f3 {
                    0x0 => Instruction::Addi { rd, rs1, imm },
                    0x2 => Instruction::Slti { rd, rs1, imm },
                    0x3 => Instruction::Sltiu { rd, rs1, imm },
                    0x4 => Instruction::Xori { rd, rs1, imm },
                    0x6 => Instruction::Ori { rd, rs1, imm },
                    0x7 => Instruction::Andi { rd, rs1, imm },
                    0x1 => Instruction::Slli {
                        rd,
                        rs1,
                        shamt: rs2,
                    },
                    0x5 => match f7 {
                        0x00 => Instruction::Srli {
                            rd,
                            rs1,
                            shamt: rs2,
                        },
                        0x20 => Instruction::Srai {
                            rd,
                            rs1,
                            shamt: rs2,
                        },
                        _ => panic!("illegal shift imm"),
                    },
                    _ => panic!("illegal I-type"),
                }
            }
            opcode::OP => match (f3, f7) {
                (0x0, 0x00) => Instruction::Add { rd, rs1, rs2 },
                (0x0, 0x20) => Instruction::Sub { rd, rs1, rs2 },
                (0x2, 0x00) => Instruction::Slt { rd, rs1, rs2 },
                (0x3, 0x00) => Instruction::Sltu { rd, rs1, rs2 },
                (0x4, 0x00) => Instruction::Xor { rd, rs1, rs2 },
                (0x6, 0x00) => Instruction::Or { rd, rs1, rs2 },
                (0x7, 0x00) => Instruction::And { rd, rs1, rs2 },
                (0x1, 0x00) => Instruction::Sll { rd, rs1, rs2 },
                (0x5, 0x00) => Instruction::Srl { rd, rs1, rs2 },
                (0x5, 0x20) => Instruction::Sra { rd, rs1, rs2 },
                _ => panic!("illegal R-type"),
            },
            opcode::SYSTEM => Instruction::Add {
                rd: 0,
                rs1: 0,
                rs2: 0,
            }, //TODO: temporary NOP
            opcode::AUIPC => Instruction::Auipc {
                rd,
                imm: fields::u_type_imm(value),
            },
            opcode::LUI => Instruction::Lui {
                rd,
                imm: fields::u_type_imm(value),
            },

            _ => todo!(),
        }
    }
}

impl From<Instruction> for u32 {
    fn from(_value: Instruction) -> Self {
        todo!()
    }
}
