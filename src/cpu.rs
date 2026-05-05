use crate::hooks::{BranchKind, CpuHooks};
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

    pub fn run<H: CpuHooks>(&mut self, pc: u32, mem: &mut impl MemoryDevice, hooks: &mut H) {
        self.pc = pc;
        println!("starting @: {:08x}", self.pc);

        loop {
            let instr: Instruction = mem.load(self.pc, AccessSize::Word).into();
            let mut pc_delta = 4;
            let mut must_break = false;

            match instr {
                Instruction::Lui { rd, imm } => {
                    self.regs[rd as usize] = imm;
                }

                Instruction::Auipc { rd, imm } => {
                    let result = imm.wrapping_add(self.pc);

                    self.regs[rd as usize] = result;
                }

                Instruction::Jal { rd, imm } => {
                    let ret = self.pc.wrapping_add(4);
                    let target = self.pc.wrapping_add(imm);

                    self.regs[rd as usize] = ret;
                    pc_delta = imm;
                    hooks.on_branch(
                        self.pc,
                        self.pc.wrapping_add(pc_delta),
                        true,
                        BranchKind::TODO,
                    );
                }

                Instruction::Jalr { rd, rs1, imm } => {
                    let base = self.regs[rs1 as usize];
                    let ret = self.pc.wrapping_add(4);
                    let target = (base.wrapping_add(imm)) & !1;

                    self.regs[rd as usize] = ret;
                    pc_delta = target.wrapping_sub(self.pc);
                    hooks.on_branch(
                        self.pc,
                        self.pc.wrapping_add(pc_delta),
                        true,
                        BranchKind::TODO,
                    );
                }

                Instruction::Beq { rs1, rs2, imm } => {
                    let a = self.regs[rs1 as usize];
                    let b = self.regs[rs2 as usize];
                    let taken = a == b;

                    if taken {
                        pc_delta = imm;
                    }
                    hooks.on_branch(
                        self.pc,
                        self.pc.wrapping_add(pc_delta),
                        taken,
                        BranchKind::TODO,
                    );
                }

                Instruction::Bne { rs1, rs2, imm } => {
                    let a = self.regs[rs1 as usize];
                    let b = self.regs[rs2 as usize];
                    let taken = a != b;

                    if taken {
                        pc_delta = imm;
                    }
                    hooks.on_branch(
                        self.pc,
                        self.pc.wrapping_add(pc_delta),
                        taken,
                        BranchKind::TODO,
                    );
                }

                Instruction::Blt { rs1, rs2, imm } => {
                    let a = self.regs[rs1 as usize];
                    let b = self.regs[rs2 as usize];
                    let taken = (a as i32) < (b as i32);

                    if taken {
                        pc_delta = imm;
                    }
                    hooks.on_branch(
                        self.pc,
                        self.pc.wrapping_add(pc_delta),
                        taken,
                        BranchKind::TODO,
                    );
                }

                Instruction::Bge { rs1, rs2, imm } => {
                    let a = self.regs[rs1 as usize];
                    let b = self.regs[rs2 as usize];
                    let taken = (a as i32) >= (b as i32);

                    if taken {
                        pc_delta = imm;
                    }
                    hooks.on_branch(
                        self.pc,
                        self.pc.wrapping_add(pc_delta),
                        taken,
                        BranchKind::TODO,
                    );
                }

                Instruction::Bltu { rs1, rs2, imm } => {
                    let a = self.regs[rs1 as usize];
                    let b = self.regs[rs2 as usize];
                    let taken = a < b;

                    if taken {
                        pc_delta = imm;
                    }
                    hooks.on_branch(
                        self.pc,
                        self.pc.wrapping_add(pc_delta),
                        taken,
                        BranchKind::TODO,
                    );
                }

                Instruction::Bgeu { rs1, rs2, imm } => {
                    let a = self.regs[rs1 as usize];
                    let b = self.regs[rs2 as usize];
                    let taken = a >= b;

                    if taken {
                        pc_delta = imm;
                    }

                    hooks.on_branch(
                        self.pc,
                        self.pc.wrapping_add(pc_delta),
                        taken,
                        BranchKind::TODO,
                    );
                }

                Instruction::Lb { rd, rs1, imm } => {
                    let addr = self.regs[rs1 as usize].wrapping_add(imm);
                    let value = Self::u8_sign_extend(mem.load(addr, AccessSize::Byte) as u8);

                    self.regs[rd as usize] = value;
                }

                Instruction::Lh { rd, rs1, imm } => {
                    let addr = self.regs[rs1 as usize].wrapping_add(imm);
                    let value = Self::u16_sign_extend(mem.load(addr, AccessSize::Half) as u16);

                    self.regs[rd as usize] = value;
                }

                Instruction::Lw { rd, rs1, imm } => {
                    let addr = self.regs[rs1 as usize].wrapping_add(imm);
                    let value = mem.load(addr, AccessSize::Word);

                    self.regs[rd as usize] = value;
                }

                Instruction::Lbu { rd, rs1, imm } => {
                    let addr = self.regs[rs1 as usize].wrapping_add(imm);
                    let value = mem.load(addr, AccessSize::Byte) as u8 as u32;

                    self.regs[rd as usize] = value;
                }

                Instruction::Lhu { rd, rs1, imm } => {
                    let addr = self.regs[rs1 as usize].wrapping_add(imm);
                    let value = mem.load(addr, AccessSize::Half) as u16 as u32;

                    self.regs[rd as usize] = value;
                }

                Instruction::Sb { rs1, rs2, imm } => {
                    let addr = self.regs[rs1 as usize].wrapping_add(imm);
                    let value = self.regs[rs2 as usize];

                    mem.store(addr, AccessSize::Byte, value);
                }

                Instruction::Sh { rs1, rs2, imm } => {
                    let addr = self.regs[rs1 as usize].wrapping_add(imm);
                    let value = self.regs[rs2 as usize];

                    mem.store(addr, AccessSize::Half, value);
                }

                Instruction::Sw { rs1, rs2, imm } => {
                    let addr = self.regs[rs1 as usize].wrapping_add(imm);
                    let value = self.regs[rs2 as usize];

                    mem.store(addr, AccessSize::Word, value);
                }

                Instruction::Addi { rd, rs1, imm } => {
                    let a = self.regs[rs1 as usize];
                    let result = a.wrapping_add(imm);

                    self.regs[rd as usize] = result;
                }

                Instruction::Slti { rd, rs1, imm } => {
                    let a = self.regs[rs1 as usize];
                    let result = ((a as i32) < (imm as i32)) as u32;

                    self.regs[rd as usize] = result;
                }

                Instruction::Sltiu { rd, rs1, imm } => {
                    let a = self.regs[rs1 as usize];
                    let result = (a < imm) as u32;

                    self.regs[rd as usize] = result;
                }

                Instruction::Xori { rd, rs1, imm } => {
                    let result = self.regs[rs1 as usize] ^ imm;

                    self.regs[rd as usize] = result;
                }

                Instruction::Ori { rd, rs1, imm } => {
                    let result = self.regs[rs1 as usize] | imm;

                    self.regs[rd as usize] = result;
                }

                Instruction::Andi { rd, rs1, imm } => {
                    let result = self.regs[rs1 as usize] & imm;

                    self.regs[rd as usize] = result;
                }

                Instruction::Slli { rd, rs1, shamt } => {
                    let result = self.regs[rs1 as usize] << shamt;

                    self.regs[rd as usize] = result;
                }

                Instruction::Srli { rd, rs1, shamt } => {
                    let result = self.regs[rs1 as usize] >> shamt;

                    self.regs[rd as usize] = result;
                }

                Instruction::Srai { rd, rs1, shamt } => {
                    let result = ((self.regs[rs1 as usize] as i32) >> shamt) as u32;

                    self.regs[rd as usize] = result;
                }

                Instruction::Add { rd, rs1, rs2 } => {
                    let result = self.regs[rs1 as usize].wrapping_add(self.regs[rs2 as usize]);

                    self.regs[rd as usize] = result;
                }

                Instruction::Sub { rd, rs1, rs2 } => {
                    let result = self.regs[rs1 as usize].wrapping_sub(self.regs[rs2 as usize]);

                    self.regs[rd as usize] = result;
                }

                Instruction::Sll { rd, rs1, rs2 } => {
                    let result = self.regs[rs1 as usize] << (self.regs[rs2 as usize] & 0x1f);

                    self.regs[rd as usize] = result;
                }

                Instruction::Slt { rd, rs1, rs2 } => {
                    let result = ((self.regs[rs1 as usize] as i32)
                        < (self.regs[rs2 as usize] as i32)) as u32;

                    self.regs[rd as usize] = result;
                }

                Instruction::Sltu { rd, rs1, rs2 } => {
                    let result = (self.regs[rs1 as usize] < self.regs[rs2 as usize]) as u32;

                    self.regs[rd as usize] = result;
                }

                Instruction::Xor { rd, rs1, rs2 } => {
                    let result = self.regs[rs1 as usize] ^ self.regs[rs2 as usize];

                    self.regs[rd as usize] = result;
                }

                Instruction::Srl { rd, rs1, rs2 } => {
                    let result = self.regs[rs1 as usize] >> (self.regs[rs2 as usize] & 0x1f);

                    self.regs[rd as usize] = result;
                }

                Instruction::Sra { rd, rs1, rs2 } => {
                    let result = ((self.regs[rs1 as usize] as i32)
                        >> (self.regs[rs2 as usize] & 0x1f))
                        as u32;

                    self.regs[rd as usize] = result;
                }

                Instruction::Or { rd, rs1, rs2 } => {
                    let result = self.regs[rs1 as usize] | self.regs[rs2 as usize];

                    self.regs[rd as usize] = result;
                }

                Instruction::And { rd, rs1, rs2 } => {
                    let result = self.regs[rs1 as usize] & self.regs[rs2 as usize];

                    self.regs[rd as usize] = result;
                }

                Instruction::Fence { .. } => {}

                Instruction::FenceTso => {}

                Instruction::Pause => {
                    must_break = true;
                }

                Instruction::Ebreak => {
                    must_break = true;
                }

                Instruction::Ecall => {
                    must_break = true;
                }
            }

            self.regs[0] = 0;
            self.pc = self.pc.wrapping_add(pc_delta);

            hooks.on_instruction(self.pc, &instr);

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
