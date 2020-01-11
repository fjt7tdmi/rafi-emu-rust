use core::*;
use trap::*;
use util::*;

use std::string::ToString;

pub trait Op : ToString {
    fn execute(&self, core: &mut Core);

    fn post_check_trap(&self, _core: &mut Core) -> Option<Trap>
    {
        None
    }
}

pub struct UnknownOp {
}

impl Op for UnknownOp {
    fn execute(&self, _core: &mut Core) {
        panic!("execute unknown op.");
    }
}

impl ToString for UnknownOp {
    fn to_string(&self) -> String {
        "unknown".to_string()
    }
}

pub struct LUI {
    pub rd: usize,
    pub imm: u32,
}

impl Op for LUI {
    fn execute(&self, core: &mut Core) {
        core.int_reg.write(self.rd, self.imm);
    }
}

impl ToString for LUI {
    fn to_string(&self) -> String {
        format!("lui {},0x{:x}", get_int_reg_name(self.rd), self.imm)
    }
}

#[test]
fn test_lui() {
    use bus::*;
    use memory::*;

    let mut memory = Memory::new();
    let mut bus = Bus::new(&mut memory);
    let mut core = Core::new(&mut bus);

    let op = LUI { rd: 1, imm: 0x12340000 };
    assert_eq!(op.to_string(), "lui ra,0x12340000");

    op.execute(&mut core);
    assert_eq!(core.int_reg.read(1), 0x12340000);
}

pub struct AUIPC {
    pub rd: usize,
    pub imm: u32,
}

impl Op for AUIPC {
    fn execute(&self, core: &mut Core) {
        let value = core.pc.wrapping_add(self.imm);

        core.int_reg.write(self.rd, value);
    }
}

impl ToString for AUIPC {
    fn to_string(&self) -> String {
        format!("auipc {},0x{:x}", get_int_reg_name(self.rd), self.imm)
    }
}

#[test]
fn test_auipc() {
    use bus::*;
    use memory::*;

    let mut memory = Memory::new();
    let mut bus = Bus::new(&mut memory);
    let mut core = Core::new(&mut bus);

    let op = AUIPC { rd: 1, imm: 0x80000000 };
    assert_eq!(op.to_string(), "auipc ra,0x80000000");

    core.pc = 0x40000000;
    op.execute(&mut core);
    assert_eq!(core.int_reg.read(1), 0xc0000000);

    core.pc = 0x80000000;
    op.execute(&mut core);
    assert_eq!(core.int_reg.read(1), 0x00000000);
}

pub struct JAL {
    pub rd: usize,
    pub imm: u32,
}

impl Op for JAL {
    fn execute(&self, core: &mut Core) {
        let next_pc = core.next_pc;

        core.next_pc = core.pc.wrapping_add(self.imm);
        core.int_reg.write(self.rd, next_pc);
    }
}

impl ToString for JAL {
    fn to_string(&self) -> String {
        match self.rd {
            0 => format!("j #{}", self.imm),
            _ => format!("jal {},{}", get_int_reg_name(self.rd), self.imm),
        }
    }
}

pub struct JALR {
    pub rd: usize,
    pub rs1: usize,
    pub imm: u32,
}

impl Op for JALR {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1);
        let next_pc = core.next_pc;

        core.next_pc = src1.wrapping_add(self.imm);
        core.int_reg.write(self.rd, next_pc);
    }
}

impl ToString for JALR {
    fn to_string(&self) -> String {
        match self.rd {
            0 => format!("jr {},{}", get_int_reg_name(self.rs1), self.imm),
            _ => format!("jalr {},{},{}", get_int_reg_name(self.rd), get_int_reg_name(self.rs1), self.imm),
        }
    }
}

pub struct BEQ {
    pub rs1: usize,
    pub rs2: usize,
    pub imm: u32,
}

impl Op for BEQ {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1);
        let src2 = core.int_reg.read(self.rs2);

        if src1 == src2 {
            core.next_pc = core.pc.wrapping_add(self.imm);
        }
    }
}

impl ToString for BEQ {
    fn to_string(&self) -> String {
        match (self.rs1, self.rs2) {
            (0, _) => format!("beqz {}, #{}", get_int_reg_name(self.rs2), self.imm),
            (_, 0) => format!("beqz {}, #{}", get_int_reg_name(self.rs1), self.imm),
            (_, _) => format!("beq {},{},{}", get_int_reg_name(self.rs1), get_int_reg_name(self.rs2), self.imm),
        }
    }
}

pub struct BNE {
    pub rs1: usize,
    pub rs2: usize,
    pub imm: u32,
}

impl Op for BNE {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1);
        let src2 = core.int_reg.read(self.rs2);

        if src1 != src2 {
            core.next_pc = core.pc.wrapping_add(self.imm);
        }
    }
}

impl ToString for BNE {
    fn to_string(&self) -> String {
        match (self.rs1, self.rs2) {
            (0, _) => format!("bnez {}, #{}", get_int_reg_name(self.rs2), self.imm),
            (_, 0) => format!("bnez {}, #{}", get_int_reg_name(self.rs1), self.imm),
            (_, _) => format!("bne {},{},{}", get_int_reg_name(self.rs1), get_int_reg_name(self.rs2), self.imm),
        }
    }
}

pub struct BLT {
    pub rs1: usize,
    pub rs2: usize,
    pub imm: u32,
}

impl Op for BLT {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1) as i32;
        let src2 = core.int_reg.read(self.rs2) as i32;

        if src1 < src2 {
            core.next_pc = core.pc.wrapping_add(self.imm);
        }
    }
}

impl ToString for BLT {
    fn to_string(&self) -> String {
        match (self.rs1, self.rs2) {
            (0, _) => format!("bltz {}, #{}", get_int_reg_name(self.rs2), self.imm),
            (_, 0) => format!("bltz {}, #{}", get_int_reg_name(self.rs1), self.imm),
            (_, _) => format!("blt {},{},{}", get_int_reg_name(self.rs1), get_int_reg_name(self.rs2), self.imm),
        }
    }
}

pub struct BGE {
    pub rs1: usize,
    pub rs2: usize,
    pub imm: u32,
}

impl Op for BGE {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1) as i32;
        let src2 = core.int_reg.read(self.rs2) as i32;

        if src1 >= src2 {
            core.next_pc = core.pc.wrapping_add(self.imm);
        }
    }
}

impl ToString for BGE {
    fn to_string(&self) -> String {
        match (self.rs1, self.rs2) {
            (0, _) => format!("bgez {}, #{}", get_int_reg_name(self.rs2), self.imm),
            (_, 0) => format!("bgez {}, #{}", get_int_reg_name(self.rs1), self.imm),
            (_, _) => format!("bge {},{},{}", get_int_reg_name(self.rs1), get_int_reg_name(self.rs2), self.imm),
        }
    }
}

pub struct BLTU {
    pub rs1: usize,
    pub rs2: usize,
    pub imm: u32,
}

impl Op for BLTU {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1);
        let src2 = core.int_reg.read(self.rs2);

        if src1 < src2 {
            core.next_pc = core.pc.wrapping_add(self.imm);
        }
    }
}

impl ToString for BLTU {
    fn to_string(&self) -> String {
        format!("bltu {},{},{}", get_int_reg_name(self.rs1), get_int_reg_name(self.rs2), self.imm)
    }
}

pub struct BGEU {
    pub rs1: usize,
    pub rs2: usize,
    pub imm: u32,
}

impl Op for BGEU {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1);
        let src2 = core.int_reg.read(self.rs2);

        if src1 >= src2 {
            core.next_pc = core.pc.wrapping_add(self.imm);
        }
    }
}

impl ToString for BGEU {
    fn to_string(&self) -> String {
        format!("bgeu {},{},{}", get_int_reg_name(self.rs1), get_int_reg_name(self.rs2), self.imm)
    }
}

pub struct LB {
    pub rd: usize,
    pub rs1: usize,
    pub imm: u32,
}

impl Op for LB {
    fn execute(&self, core: &mut Core) {
        let addr = core.int_reg.read(self.rs1).wrapping_add(self.imm);
        let value = sign_extend(8, core.bus.read_u8(addr) as u32);

        core.int_reg.write(self.rd, value);
    }
}

impl ToString for LB {
    fn to_string(&self) -> String {
        format!("lb {},{}({})", get_int_reg_name(self.rd), self.imm, get_int_reg_name(self.rs1))
    }
}

pub struct LH {
    pub rd: usize,
    pub rs1: usize,
    pub imm: u32,
}

impl Op for LH {
    fn execute(&self, core: &mut Core) {
        let addr = core.int_reg.read(self.rs1).wrapping_add(self.imm);
        let value = sign_extend(16, core.bus.read_u16(addr) as u32);

        core.int_reg.write(self.rd, value);
    }
}

impl ToString for LH {
    fn to_string(&self) -> String {
        format!("lh {},{}({})", get_int_reg_name(self.rd), self.imm, get_int_reg_name(self.rs1))
    }
}

pub struct LW {
    pub rd: usize,
    pub rs1: usize,
    pub imm: u32,
}

impl Op for LW {
    fn execute(&self, core: &mut Core) {
        let addr = core.int_reg.read(self.rs1).wrapping_add(self.imm);
        let value = core.bus.read_u32(addr);

        core.int_reg.write(self.rd, value);
    }
}

impl ToString for LW {
    fn to_string(&self) -> String {
        format!("lw {},{}({})", get_int_reg_name(self.rd), self.imm, get_int_reg_name(self.rs1))
    }
}

pub struct LBU {
    pub rd: usize,
    pub rs1: usize,
    pub imm: u32,
}

impl Op for LBU {
    fn execute(&self, core: &mut Core) {
        let addr = core.int_reg.read(self.rs1).wrapping_add(self.imm);
        let value = core.bus.read_u8(addr) as u32;

        core.int_reg.write(self.rd, value);
    }
}

impl ToString for LBU {
    fn to_string(&self) -> String {
        format!("lbu {},{}({})", get_int_reg_name(self.rd), self.imm, get_int_reg_name(self.rs1))
    }
}

pub struct LHU {
    pub rd: usize,
    pub rs1: usize,
    pub imm: u32,
}

impl Op for LHU {
    fn execute(&self, core: &mut Core) {
        let addr = core.int_reg.read(self.rs1).wrapping_add(self.imm);
        let value = core.bus.read_u16(addr) as u32;

        core.int_reg.write(self.rd, value);
    }
}

impl ToString for LHU {
    fn to_string(&self) -> String {
        format!("lhu {},{}({})", get_int_reg_name(self.rd), self.imm, get_int_reg_name(self.rs1))
    }
}

pub struct SB {
    pub rs1: usize,
    pub rs2: usize,
    pub imm: u32,
}

impl Op for SB {
    fn execute(&self, core: &mut Core) {
        let addr = core.int_reg.read(self.rs1).wrapping_add(self.imm);
        let value = core.int_reg.read(self.rs2) as u8;

        core.bus.write_u8(addr, value);
    }
}

impl ToString for SB {
    fn to_string(&self) -> String {
        format!("sb {},{}({})", get_int_reg_name(self.rs2), self.imm, get_int_reg_name(self.rs1))
    }
}

pub struct SH {
    pub rs1: usize,
    pub rs2: usize,
    pub imm: u32,
}

impl Op for SH {
    fn execute(&self, core: &mut Core) {
        let addr = core.int_reg.read(self.rs1).wrapping_add(self.imm);
        let value = core.int_reg.read(self.rs2) as u16;

        core.bus.write_u16(addr, value);
    }
}

impl ToString for SH {
    fn to_string(&self) -> String {
        format!("sh {},{}({})", get_int_reg_name(self.rs2), self.imm, get_int_reg_name(self.rs1))
    }
}

pub struct SW {
    pub rs1: usize,
    pub rs2: usize,
    pub imm: u32,
}

impl Op for SW {
    fn execute(&self, core: &mut Core) {
        let addr = core.int_reg.read(self.rs1).wrapping_add(self.imm);
        let value = core.int_reg.read(self.rs2) as u32;

        core.bus.write_u32(addr, value);
    }
}

impl ToString for SW {
    fn to_string(&self) -> String {
        format!("sw {},{}({})", get_int_reg_name(self.rs2), self.imm, get_int_reg_name(self.rs1))
    }
}

pub struct ADDI {
    pub rd: usize,
    pub rs1: usize,
    pub imm: u32,
}

impl Op for ADDI {
    fn execute(&self, core: &mut Core) {
        let value = core.int_reg.read(self.rs1).wrapping_add(self.imm);

        core.int_reg.write(self.rd, value);
    }
}

impl ToString for ADDI {
    fn to_string(&self) -> String {
        format!("addi {},{},{}", get_int_reg_name(self.rd), get_int_reg_name(self.rs1), self.imm)
    }
}

pub struct SLTI {
    pub rd: usize,
    pub rs1: usize,
    pub imm: u32,
}

impl Op for SLTI {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1);
        let value = if (src1 as i32) < (self.imm as i32) { 1 } else { 0 };

        core.int_reg.write(self.rd, value);
    }
}

impl ToString for SLTI {
    fn to_string(&self) -> String {
        format!("slti {},{},{}", get_int_reg_name(self.rd), get_int_reg_name(self.rs1), self.imm)
    }
}

pub struct SLTIU {
    pub rd: usize,
    pub rs1: usize,
    pub imm: u32,
}

impl Op for SLTIU {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1);
        let value = if src1 < self.imm { 1 } else { 0 };

        core.int_reg.write(self.rd, value);
    }
}

impl ToString for SLTIU {
    fn to_string(&self) -> String {
        format!("sltiu {},{},{}", get_int_reg_name(self.rd), get_int_reg_name(self.rs1), self.imm)
    }
}

pub struct XORI {
    pub rd: usize,
    pub rs1: usize,
    pub imm: u32,
}

impl Op for XORI {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1);
        let value = src1 ^ self.imm;

        core.int_reg.write(self.rd, value);
    }
}

impl ToString for XORI {
    fn to_string(&self) -> String {
        format!("xori {},{},{}", get_int_reg_name(self.rd), get_int_reg_name(self.rs1), self.imm)
    }
}

pub struct ORI {
    pub rd: usize,
    pub rs1: usize,
    pub imm: u32,
}

impl Op for ORI {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1);
        let value = src1 | self.imm;

        core.int_reg.write(self.rd, value);
    }
}

impl ToString for ORI {
    fn to_string(&self) -> String {
        format!("ori {},{},{}", get_int_reg_name(self.rd), get_int_reg_name(self.rs1), self.imm)
    }
}

pub struct ANDI {
    pub rd: usize,
    pub rs1: usize,
    pub imm: u32,
}

impl Op for ANDI {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1);
        let value = src1 & self.imm;

        core.int_reg.write(self.rd, value);
    }
}

impl ToString for ANDI {
    fn to_string(&self) -> String {
        format!("andi {},{},{}", get_int_reg_name(self.rd), get_int_reg_name(self.rs1), self.imm)
    }
}

pub struct SLLI {
    pub rd: usize,
    pub rs1: usize,
    pub shamt: u32,
}

impl Op for SLLI {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1);
        let value = src1.wrapping_shl(self.shamt);

        core.int_reg.write(self.rd, value);
    }
}

impl ToString for SLLI {
    fn to_string(&self) -> String {
        format!("slli {},{},{}", get_int_reg_name(self.rd), get_int_reg_name(self.rs1), self.shamt)
    }
}

pub struct SRLI {
    pub rd: usize,
    pub rs1: usize,
    pub shamt: u32,
}

impl Op for SRLI {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1);
        let value = src1.wrapping_shr(self.shamt);

        core.int_reg.write(self.rd, value);
    }
}

impl ToString for SRLI {
    fn to_string(&self) -> String {
        format!("srli {},{},{}", get_int_reg_name(self.rd), get_int_reg_name(self.rs1), self.shamt)
    }
}

pub struct SRAI {
    pub rd: usize,
    pub rs1: usize,
    pub shamt: u32,
}

impl Op for SRAI {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1) as i32;
        let value = src1.wrapping_shr(self.shamt);

        core.int_reg.write(self.rd, value as u32);
    }
}

impl ToString for SRAI {
    fn to_string(&self) -> String {
        format!("srai {},{},{}", get_int_reg_name(self.rd), get_int_reg_name(self.rs1), self.shamt)
    }
}

pub struct ADD {
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,
}

impl Op for ADD {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1);
        let src2 = core.int_reg.read(self.rs2);
        let value = src1.wrapping_add(src2);

        core.int_reg.write(self.rd, value);
    }
}

impl ToString for ADD {
    fn to_string(&self) -> String {
        format!("add {},{},{}", get_int_reg_name(self.rd), get_int_reg_name(self.rs1), get_int_reg_name(self.rs2))
    }
}

pub struct SUB {
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,
}

impl Op for SUB {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1);
        let src2 = core.int_reg.read(self.rs2);
        let value = src1.wrapping_sub(src2);

        core.int_reg.write(self.rd, value);
    }
}

impl ToString for SUB {
    fn to_string(&self) -> String {
        format!("sub {},{},{}", get_int_reg_name(self.rd), get_int_reg_name(self.rs1), get_int_reg_name(self.rs2))
    }
}

pub struct SLL {
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,
}

impl Op for SLL {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1);
        let src2 = core.int_reg.read(self.rs2);
        let value = src1.wrapping_shl(src2);

        core.int_reg.write(self.rd, value);
    }
}

impl ToString for SLL {
    fn to_string(&self) -> String {
        format!("sll {},{},{}", get_int_reg_name(self.rd), get_int_reg_name(self.rs1), get_int_reg_name(self.rs2))
    }
}

pub struct SLT {
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,
}

impl Op for SLT {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1);
        let src2 = core.int_reg.read(self.rs2);
        let value = if (src1 as i32) < (src2 as i32) { 1 } else { 0 };

        core.int_reg.write(self.rd, value);
    }
}

impl ToString for SLT {
    fn to_string(&self) -> String {
        format!("slt {},{},{}", get_int_reg_name(self.rd), get_int_reg_name(self.rs1), get_int_reg_name(self.rs2))
    }
}

pub struct SLTU {
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,
}

impl Op for SLTU {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1);
        let src2 = core.int_reg.read(self.rs2);
        let value = if src1 < src2 { 1 } else { 0 };

        core.int_reg.write(self.rd, value);
    }
}

impl ToString for SLTU {
    fn to_string(&self) -> String {
        format!("sltu {},{},{}", get_int_reg_name(self.rd), get_int_reg_name(self.rs1), get_int_reg_name(self.rs2))
    }
}

pub struct XOR {
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,
}

impl Op for XOR {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1);
        let src2 = core.int_reg.read(self.rs2);
        let value = src1 ^ src2;

        core.int_reg.write(self.rd, value);
    }
}

impl ToString for XOR {
    fn to_string(&self) -> String {
        format!("xor {},{},{}", get_int_reg_name(self.rd), get_int_reg_name(self.rs1), get_int_reg_name(self.rs2))
    }
}

pub struct SRL {
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,
}

impl Op for SRL {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1);
        let src2 = core.int_reg.read(self.rs2);
        let value = src1.wrapping_shr(src2);

        core.int_reg.write(self.rd, value);
    }
}

impl ToString for SRL {
    fn to_string(&self) -> String {
        format!("srl {},{},{}", get_int_reg_name(self.rd), get_int_reg_name(self.rs1), get_int_reg_name(self.rs2))
    }
}

pub struct SRA {
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,
}

impl Op for SRA {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1);
        let src2 = core.int_reg.read(self.rs2);
        let value = (src1 as i32).wrapping_shr(src2);

        core.int_reg.write(self.rd, value as u32);
    }
}

impl ToString for SRA {
    fn to_string(&self) -> String {
        format!("sra {},{},{}", get_int_reg_name(self.rd), get_int_reg_name(self.rs1), get_int_reg_name(self.rs2))
    }
}

pub struct OR {
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,
}

impl Op for OR {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1);
        let src2 = core.int_reg.read(self.rs2);
        let value = src1 | src2;

        core.int_reg.write(self.rd, value);
    }
}

impl ToString for OR {
    fn to_string(&self) -> String {
        format!("or {},{},{}", get_int_reg_name(self.rd), get_int_reg_name(self.rs1), get_int_reg_name(self.rs2))
    }
}

pub struct AND {
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,
}

impl Op for AND {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1);
        let src2 = core.int_reg.read(self.rs2);
        let value = src1 & src2;

        core.int_reg.write(self.rd, value);
    }
}

impl ToString for AND {
    fn to_string(&self) -> String {
        format!("and {},{},{}", get_int_reg_name(self.rd), get_int_reg_name(self.rs1), get_int_reg_name(self.rs2))
    }
}

pub struct FENCE {
    pub pred: u32,
    pub succ: u32,
}

impl Op for FENCE {
    fn execute(&self, _core: &mut Core) {
    }
}

impl ToString for FENCE {
    fn to_string(&self) -> String {
        format!("fence")
    }
}

pub struct FENCEI {
}

impl Op for FENCEI {
    fn execute(&self, _core: &mut Core) {
    }
}

impl ToString for FENCEI {
    fn to_string(&self) -> String {
        format!("fence.i")
    }
}

pub struct ECALL {
}

impl Op for ECALL {
    fn execute(&self, _core: &mut Core) {
    }

    fn post_check_trap(&self, core: &mut Core) -> Option<Trap>  {
        Some(Trap::new_ecall_from_machine(core.pc))
    }
}

impl ToString for ECALL {
    fn to_string(&self) -> String {
        format!("ecall")
    }
}

pub struct EBREAK {
}

impl Op for EBREAK {
    fn execute(&self, _core: &mut Core) {
    }

    fn post_check_trap(&self, core: &mut Core) -> Option<Trap> {
        Some(Trap::new_ebreak(core.pc))
    }
}

impl ToString for EBREAK {
    fn to_string(&self) -> String {
        format!("ebreak")
    }
}

pub struct CSRRW {
    pub csr: usize,
    pub rd: usize,
    pub rs1: usize,
}

impl Op for CSRRW {
    fn execute(&self, core: &mut Core) {
        let org = core.csr.read(self.csr);
        let value = core.int_reg.read(self.rs1);

        core.csr.write(self.csr, value);
        core.int_reg.write(self.rd, org);
    }
}

impl ToString for CSRRW {
    fn to_string(&self) -> String {
        match self.rd {
            0 => format!("csrw {},{}", get_csr_name(self.csr), get_int_reg_name(self.rs1)),
            _ => format!("csrrw {},{},{}", get_int_reg_name(self.rd), get_csr_name(self.csr), get_int_reg_name(self.rs1)),
        }
    }
}

pub struct CSRRS {
    pub csr: usize,
    pub rd: usize,
    pub rs1: usize,
}

impl Op for CSRRS {
    fn execute(&self, core: &mut Core) {
        let org = core.csr.read(self.csr);
        let value = org | core.int_reg.read(self.rs1);

        core.csr.write(self.csr, value);
        core.int_reg.write(self.rd, org);
    }
}

impl ToString for CSRRS {
    fn to_string(&self) -> String {
        match (self.rd, self.rs1) {
            (_, 0) => format!("csrr {},{}", get_int_reg_name(self.rd), get_csr_name(self.csr)),
            (0, _) => format!("csrr {},{}", get_csr_name(self.csr), get_int_reg_name(self.rs1)),
            (_, _) => format!("csrrs {},{},{}", get_int_reg_name(self.rd), get_csr_name(self.csr), get_int_reg_name(self.rs1)),
        }
    }
}

pub struct CSRRC {
    pub csr: usize,
    pub rd: usize,
    pub rs1: usize,
}

impl Op for CSRRC {
    fn execute(&self, core: &mut Core) {
        let org = core.csr.read(self.csr);
        let value = org & !core.int_reg.read(self.rs1);

        core.csr.write(self.csr, value);
        core.int_reg.write(self.rd, org);
    }
}

impl ToString for CSRRC {
    fn to_string(&self) -> String {
        match self.rd {
            0 => format!("csrc {},{}", get_csr_name(self.csr), get_int_reg_name(self.rs1)),
            _ => format!("csrrc {},{},{}", get_int_reg_name(self.rd), get_csr_name(self.csr), get_int_reg_name(self.rs1)),
        }
    }
}

pub struct CSRRWI {
    pub csr: usize,
    pub rd: usize,
    pub zimm: u32,
}

impl Op for CSRRWI {
    fn execute(&self, core: &mut Core) {
        let org = core.csr.read(self.csr);
        let value = self.zimm;

        core.csr.write(self.csr, value);
        core.int_reg.write(self.rd, org);
    }
}

impl ToString for CSRRWI {
    fn to_string(&self) -> String {
        match self.rd {
            0 => format!("csrwi {},{}", get_csr_name(self.csr), self.zimm),
            _ => format!("csrrwi {},{},{}", get_int_reg_name(self.rd), get_csr_name(self.csr), self.zimm),
        }
    }
}

pub struct CSRRSI {
    pub csr: usize,
    pub rd: usize,
    pub zimm: u32,
}

impl Op for CSRRSI {
    fn execute(&self, core: &mut Core) {
        let org = core.csr.read(self.csr);
        let value = org | self.zimm;

        core.csr.write(self.csr, value);
        core.int_reg.write(self.rd, org);
    }
}

impl ToString for CSRRSI {
    fn to_string(&self) -> String {
        match self.rd {
            0 => format!("csrsi {},{}", get_csr_name(self.csr), self.zimm),
            _ => format!("csrrsi {},{},{}", get_int_reg_name(self.rd), get_csr_name(self.csr), self.zimm),
        }
    }
}

pub struct CSRRCI {
    pub csr: usize,
    pub rd: usize,
    pub zimm: u32,
}

impl Op for CSRRCI {
    fn execute(&self, core: &mut Core) {
        let org = core.csr.read(self.csr);
        let value = org & !self.zimm;

        core.csr.write(self.csr, value);
        core.int_reg.write(self.rd, org);
    }
}

impl ToString for CSRRCI {
    fn to_string(&self) -> String {
        match self.rd {
            0 => format!("csrci {},{}", get_csr_name(self.csr), self.zimm),
            _ => format!("csrrci {},{},{}", get_int_reg_name(self.rd), get_csr_name(self.csr), self.zimm),
        }
    }
}

pub struct URET {
}

impl Op for URET {
    fn execute(&self, _core: &mut Core) {
    }

    fn post_check_trap(&self, core: &mut Core) -> Option<Trap> {
        Some(Trap::new_trap_return(core.pc))
    }
}

impl ToString for URET {
    fn to_string(&self) -> String {
        format!("uret")
    }
}

pub struct SRET {
}

impl Op for SRET {
    fn execute(&self, _core: &mut Core) {
    }

    fn post_check_trap(&self, core: &mut Core) -> Option<Trap> {
        Some(Trap::new_trap_return(core.pc))
    }
}

impl ToString for SRET {
    fn to_string(&self) -> String {
        format!("sret")
    }
}

pub struct MRET {
}

impl Op for MRET {
    fn execute(&self, _core: &mut Core) {
    }

    fn post_check_trap(&self, core: &mut Core) -> Option<Trap> {
        Some(Trap::new_trap_return(core.pc))
    }
}

impl ToString for MRET {
    fn to_string(&self) -> String {
        format!("mret")
    }
}

pub struct WFI {
}

impl Op for WFI {
    fn execute(&self, _core: &mut Core) {
    }
}

impl ToString for WFI {
    fn to_string(&self) -> String {
        format!("wfi")
    }
}

pub struct SFENCEVMA {
}

impl Op for SFENCEVMA {
    fn execute(&self, _core: &mut Core) {
    }
}

impl ToString for SFENCEVMA {
    fn to_string(&self) -> String {
        format!("sfence.vma")
    }
}
