use crate::instructions::Instruction;
use crate::memory_device::*;
pub struct Cpu32 {
    regs: [u32; 32],
    pc: u32,
}

impl Cpu32 {
    pub fn new() -> Self {
        Self {
            pc: 0,
            regs: [0; 32],
        }
    }

    pub fn run(&mut self, pc: u32, mem: &mut impl MemoryDevice) {
        self.pc = pc;
        println!("starting @: {:08x}", self.pc);
        const ENABLE_TRACE: bool = false;
        loop {
            let instr: Instruction = mem.load(self.pc, AccessSize::Word).into();
            let mut pc_delta = 4;
            let mut must_break = false;

            macro_rules! trace {
                ($self:expr, $fmt:expr $(, $args:expr)*) => {
                    if ENABLE_TRACE {
                        println!(
                            "[PC=0x{:08x}] {}",
                            $self.pc,
                            format!($fmt $(, $args)*)
                        );
                   }
                };
            }

            match instr {
                Instruction::Lui { rd, imm } => {
                    trace!(
                        self,
                        "LUI   x{}, 0x{:08x} -> x{}=0x{:08x}", rd, imm, rd, imm
                    );
                    self.regs[rd as usize] = imm;
                }

                Instruction::Auipc { rd, imm } => {
                    let result = imm.wrapping_add(self.pc);
                    trace!(
                        self,
                        "AUIPC x{}, 0x{:08x} -> 0x{:08x}+0x{:08x}=0x{:08x}",
                        rd,
                        imm,
                        self.pc,
                        imm,
                        result
                    );
                    self.regs[rd as usize] = result;
                }

                Instruction::Jal { rd, imm } => {
                    let ret = self.pc.wrapping_add(4);
                    let target = self.pc.wrapping_add(imm);
                    trace!(
                        self,
                        "JAL   x{}, 0x{:08x} -> x{}=0x{:08x}, target=0x{:08x}",
                        rd,
                        imm,
                        rd,
                        ret,
                        target
                    );
                    self.regs[rd as usize] = ret;
                    pc_delta = imm;
                }

                Instruction::Jalr { rd, rs1, imm } => {
                    let base = self.regs[rs1 as usize];
                    let ret = self.pc.wrapping_add(4);
                    let target = (base.wrapping_add(imm)) & !1;

                    trace!(
                        self,
                        "JALR  x{}, x{}=0x{:08x}, 0x{:08x} -> x{}=0x{:08x}, target=0x{:08x}",
                        rd,
                        rs1,
                        base,
                        imm,
                        rd,
                        ret,
                        target
                    );

                    self.regs[rd as usize] = ret;
                    pc_delta = target.wrapping_sub(self.pc);
                }

                Instruction::Beq { rs1, rs2, imm } => {
                    let a = self.regs[rs1 as usize];
                    let b = self.regs[rs2 as usize];
                    let taken = a == b;

                    trace!(
                        self,
                        "BEQ   x{}=0x{:08x}, x{}=0x{:08x} -> {} target=0x{:08x}",
                        rs1,
                        a,
                        rs2,
                        b,
                        taken,
                        self.pc.wrapping_add(imm)
                    );

                    if taken {
                        pc_delta = imm;
                    }
                }

                Instruction::Bne { rs1, rs2, imm } => {
                    let a = self.regs[rs1 as usize];
                    let b = self.regs[rs2 as usize];
                    let taken = a != b;

                    trace!(
                        self,
                        "BNE   x{}=0x{:08x}, x{}=0x{:08x} -> {} target=0x{:08x}",
                        rs1,
                        a,
                        rs2,
                        b,
                        taken,
                        self.pc.wrapping_add(imm)
                    );

                    if taken {
                        pc_delta = imm;
                    }
                }

                Instruction::Blt { rs1, rs2, imm } => {
                    let a = self.regs[rs1 as usize];
                    let b = self.regs[rs2 as usize];
                    let taken = (a as i32) < (b as i32);

                    trace!(
                        self,
                        "BLT   x{}={}, x{}={} -> {} target=0x{:08x}",
                        rs1,
                        a as i32,
                        rs2,
                        b as i32,
                        taken,
                        self.pc.wrapping_add(imm)
                    );

                    if taken {
                        pc_delta = imm;
                    }
                }

                Instruction::Bge { rs1, rs2, imm } => {
                    let a = self.regs[rs1 as usize];
                    let b = self.regs[rs2 as usize];
                    let taken = (a as i32) >= (b as i32);

                    trace!(
                        self,
                        "BGE   x{}={}, x{}={} -> {} target=0x{:08x}",
                        rs1,
                        a as i32,
                        rs2,
                        b as i32,
                        taken,
                        self.pc.wrapping_add(imm)
                    );

                    if taken {
                        pc_delta = imm;
                    }
                }

                Instruction::Bltu { rs1, rs2, imm } => {
                    let a = self.regs[rs1 as usize];
                    let b = self.regs[rs2 as usize];
                    let taken = a < b;

                    trace!(
                        self,
                        "BLTU  x{}={}, x{}={} -> {} target=0x{:08x}",
                        rs1,
                        a,
                        rs2,
                        b,
                        taken,
                        self.pc.wrapping_add(imm)
                    );

                    if taken {
                        pc_delta = imm;
                    }
                }

                Instruction::Bgeu { rs1, rs2, imm } => {
                    let a = self.regs[rs1 as usize];
                    let b = self.regs[rs2 as usize];
                    let taken = a >= b;

                    trace!(
                        self,
                        "BGEU  x{}={}, x{}={} -> {} target=0x{:08x}",
                        rs1,
                        a,
                        rs2,
                        b,
                        taken,
                        self.pc.wrapping_add(imm)
                    );

                    if taken {
                        pc_delta = imm;
                    }
                }

                Instruction::Lb { rd, rs1, imm } => {
                    let addr = self.regs[rs1 as usize].wrapping_add(imm);
                    let value = Self::u8_sign_extend(mem.load(addr, AccessSize::Byte) as u8);

                    trace!(self, "LB    x{}, [0x{:08x}] -> 0x{:08x}", rd, addr, value);

                    self.regs[rd as usize] = value;
                }

                Instruction::Lh { rd, rs1, imm } => {
                    let addr = self.regs[rs1 as usize].wrapping_add(imm);
                    let value = Self::u16_sign_extend(mem.load(addr, AccessSize::Half) as u16);

                    trace!(self, "LH    x{}, [0x{:08x}] -> 0x{:08x}", rd, addr, value);

                    self.regs[rd as usize] = value;
                }

                Instruction::Lw { rd, rs1, imm } => {
                    let addr = self.regs[rs1 as usize].wrapping_add(imm);
                    let value = mem.load(addr, AccessSize::Word);

                    trace!(self, "LW    x{}, [0x{:08x}] -> 0x{:08x}", rd, addr, value);

                    self.regs[rd as usize] = value;
                }

                Instruction::Lbu { rd, rs1, imm } => {
                    let addr = self.regs[rs1 as usize].wrapping_add(imm);
                    let value = mem.load(addr, AccessSize::Byte) as u8 as u32;

                    trace!(self, "LBU   x{}, [0x{:08x}] -> 0x{:08x}", rd, addr, value);

                    self.regs[rd as usize] = value;
                }

                Instruction::Lhu { rd, rs1, imm } => {
                    let addr = self.regs[rs1 as usize].wrapping_add(imm);
                    let value = mem.load(addr, AccessSize::Half) as u16 as u32;

                    trace!(self, "LHU   x{}, [0x{:08x}] -> 0x{:08x}", rd, addr, value);

                    self.regs[rd as usize] = value;
                }

                Instruction::Sb { rs1, rs2, imm } => {
                    let addr = self.regs[rs1 as usize].wrapping_add(imm);
                    let value = self.regs[rs2 as usize];

                    trace!(self, "SB    [0x{:08x}] <- x{}=0x{:08x}", addr, rs2, value);

                    mem.store(addr, AccessSize::Byte, value);
                }

                Instruction::Sh { rs1, rs2, imm } => {
                    let addr = self.regs[rs1 as usize].wrapping_add(imm);
                    let value = self.regs[rs2 as usize];

                    trace!(self, "SH    [0x{:08x}] <- x{}=0x{:08x}", addr, rs2, value);

                    mem.store(addr, AccessSize::Half, value);
                }

                Instruction::Sw { rs1, rs2, imm } => {
                    let addr = self.regs[rs1 as usize].wrapping_add(imm);
                    let value = self.regs[rs2 as usize];

                    trace!(self, "SW    [0x{:08x}] <- x{}=0x{:08x}", addr, rs2, value);

                    mem.store(addr, AccessSize::Word, value);
                }

                Instruction::Addi { rd, rs1, imm } => {
                    let a = self.regs[rs1 as usize];
                    let result = a.wrapping_add(imm);

                    trace!(
                        self,
                        "ADDI  x{}, x{}=0x{:08x}, 0x{:08x} -> 0x{:08x}", rd, rs1, a, imm, result
                    );

                    self.regs[rd as usize] = result;
                }

                Instruction::Slti { rd, rs1, imm } => {
                    let a = self.regs[rs1 as usize];
                    let result = ((a as i32) < (imm as i32)) as u32;

                    trace!(self, "SLTI  -> x{}={}", rd, result);

                    self.regs[rd as usize] = result;
                }

                Instruction::Sltiu { rd, rs1, imm } => {
                    let a = self.regs[rs1 as usize];
                    let result = (a < imm) as u32;

                    trace!(self, "SLTIU -> x{}={}", rd, result);

                    self.regs[rd as usize] = result;
                }

                Instruction::Xori { rd, rs1, imm } => {
                    let result = self.regs[rs1 as usize] ^ imm;
                    trace!(self, "XORI  -> x{}=0x{:08x}", rd, result);
                    self.regs[rd as usize] = result;
                }

                Instruction::Ori { rd, rs1, imm } => {
                    let result = self.regs[rs1 as usize] | imm;
                    trace!(self, "ORI   -> x{}=0x{:08x}", rd, result);
                    self.regs[rd as usize] = result;
                }

                Instruction::Andi { rd, rs1, imm } => {
                    let result = self.regs[rs1 as usize] & imm;
                    trace!(self, "ANDI  -> x{}=0x{:08x}", rd, result);
                    self.regs[rd as usize] = result;
                }

                Instruction::Slli { rd, rs1, shamt } => {
                    let result = self.regs[rs1 as usize] << shamt;
                    trace!(self, "SLLI  -> x{}=0x{:08x}", rd, result);
                    self.regs[rd as usize] = result;
                }

                Instruction::Srli { rd, rs1, shamt } => {
                    let result = self.regs[rs1 as usize] >> shamt;
                    trace!(self, "SRLI  -> x{}=0x{:08x}", rd, result);
                    self.regs[rd as usize] = result;
                }

                Instruction::Srai { rd, rs1, shamt } => {
                    let result = ((self.regs[rs1 as usize] as i32) >> shamt) as u32;
                    trace!(self, "SRAI  -> x{}=0x{:08x}", rd, result);
                    self.regs[rd as usize] = result;
                }

                Instruction::Add { rd, rs1, rs2 } => {
                    let result = self.regs[rs1 as usize].wrapping_add(self.regs[rs2 as usize]);

                    trace!(self, "ADD   -> x{}=0x{:08x}", rd, result);

                    self.regs[rd as usize] = result;
                }

                Instruction::Sub { rd, rs1, rs2 } => {
                    let result = self.regs[rs1 as usize].wrapping_sub(self.regs[rs2 as usize]);

                    trace!(self, "SUB   -> x{}=0x{:08x}", rd, result);

                    self.regs[rd as usize] = result;
                }

                Instruction::Sll { rd, rs1, rs2 } => {
                    let result = self.regs[rs1 as usize] << (self.regs[rs2 as usize] & 0x1f);

                    trace!(self, "SLL   -> x{}=0x{:08x}", rd, result);

                    self.regs[rd as usize] = result;
                }

                Instruction::Slt { rd, rs1, rs2 } => {
                    let result = ((self.regs[rs1 as usize] as i32)
                        < (self.regs[rs2 as usize] as i32)) as u32;

                    trace!(self, "SLT   -> x{}={}", rd, result);

                    self.regs[rd as usize] = result;
                }

                Instruction::Sltu { rd, rs1, rs2 } => {
                    let result = (self.regs[rs1 as usize] < self.regs[rs2 as usize]) as u32;

                    trace!(self, "SLTU  -> x{}={}", rd, result);

                    self.regs[rd as usize] = result;
                }

                Instruction::Xor { rd, rs1, rs2 } => {
                    let result = self.regs[rs1 as usize] ^ self.regs[rs2 as usize];

                    trace!(self, "XOR   -> x{}=0x{:08x}", rd, result);

                    self.regs[rd as usize] = result;
                }

                Instruction::Srl { rd, rs1, rs2 } => {
                    let result = self.regs[rs1 as usize] >> (self.regs[rs2 as usize] & 0x1f);

                    trace!(self, "SRL   -> x{}=0x{:08x}", rd, result);

                    self.regs[rd as usize] = result;
                }

                Instruction::Sra { rd, rs1, rs2 } => {
                    let result = ((self.regs[rs1 as usize] as i32)
                        >> (self.regs[rs2 as usize] & 0x1f))
                        as u32;

                    trace!(self, "SRA   -> x{}=0x{:08x}", rd, result);

                    self.regs[rd as usize] = result;
                }

                Instruction::Or { rd, rs1, rs2 } => {
                    let result = self.regs[rs1 as usize] | self.regs[rs2 as usize];

                    trace!(self, "OR    -> x{}=0x{:08x}", rd, result);

                    self.regs[rd as usize] = result;
                }

                Instruction::And { rd, rs1, rs2 } => {
                    let result = self.regs[rs1 as usize] & self.regs[rs2 as usize];

                    trace!(self, "AND   -> x{}=0x{:08x}", rd, result);

                    self.regs[rd as usize] = result;
                }

                Instruction::Fence { .. } => {
                    trace!(self, "FENCE");
                }

                Instruction::FenceTso => {
                    trace!(self, "FENCE.TSO");
                }

                Instruction::Pause => {
                    trace!(self, "PAUSE");
                    must_break = true;
                }

                Instruction::Ebreak => {
                    trace!(self, "EBREAK");
                    must_break = true;
                }

                Instruction::Ecall => {
                    trace!(self, "ECALL");
                    must_break = true;
                }
            }

            self.regs[0] = 0;
            self.pc = self.pc.wrapping_add(pc_delta);

            if must_break {
                return;
            }
        }
    }

    fn u8_sign_extend(value: u8) -> u32 {
        value as u8 as i8 as i32 as u32
    }
    fn u16_sign_extend(value: u16) -> u32 {
        value as u16 as i16 as i32 as u32
    }
}
