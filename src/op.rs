use core::*;
use util::*;

use std::string::ToString;

pub trait Op : ToString {
    fn execute(&self, core: &mut Core);
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

#[allow(dead_code)]
pub struct JAL {
    pub rd: usize,
    pub imm: u32,
}

#[allow(dead_code)]
impl Op for JAL {
    fn execute(&self, core: &mut Core) {
        let next_pc = core.next_pc;

        core.next_pc = core.pc.wrapping_add(self.imm);
        core.int_reg.write(self.rd, next_pc);
    }
}

#[allow(dead_code)]
impl ToString for JAL {
    fn to_string(&self) -> String {
        match self.rd {
            0 => format!("j #{}", self.imm),
            _ => format!("jal {},{}", get_int_reg_name(self.rd), self.imm),
        }
    }
}

#[allow(dead_code)]
pub struct JALR {
    pub rd: usize,
    pub rs1: usize,
    pub imm: u32,
}

#[allow(dead_code)]
impl Op for JALR {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1);
        let next_pc = core.next_pc;

        core.next_pc = src1.wrapping_add(self.imm);
        core.int_reg.write(self.rd, next_pc);
    }
}

#[allow(dead_code)]
impl ToString for JALR {
    fn to_string(&self) -> String {
        match self.rd {
            0 => format!("jr {},{}", get_int_reg_name(self.rs1), self.imm),
            _ => format!("jalr {},{},{}", get_int_reg_name(self.rd), get_int_reg_name(self.rs1), self.imm),
        }
    }
}

#[allow(dead_code)]
pub struct BEQ {
    pub rs1: usize,
    pub rs2: usize,
    pub imm: u32,
}

#[allow(dead_code)]
impl Op for BEQ {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1);
        let src2 = core.int_reg.read(self.rs2);

        if src1 == src2 {
            core.next_pc = core.pc.wrapping_add(self.imm);
        }
    }
}

#[allow(dead_code)]
impl ToString for BEQ {
    fn to_string(&self) -> String {
        match (self.rs1, self.rs2) {
            (0, _) => format!("beqz {}, #{}", get_int_reg_name(self.rs2), self.imm),
            (_, 0) => format!("beqz {}, #{}", get_int_reg_name(self.rs1), self.imm),
            (_, _) => format!("beq {},{},{}", get_int_reg_name(self.rs1), get_int_reg_name(self.rs2), self.imm),
        }
    }
}

#[allow(dead_code)]
pub struct BNE {
    pub rs1: usize,
    pub rs2: usize,
    pub imm: u32,
}

#[allow(dead_code)]
impl Op for BNE {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1);
        let src2 = core.int_reg.read(self.rs2);

        if src1 != src2 {
            core.next_pc = core.pc.wrapping_add(self.imm);
        }
    }
}

#[allow(dead_code)]
impl ToString for BNE {
    fn to_string(&self) -> String {
        match (self.rs1, self.rs2) {
            (0, _) => format!("bnez {}, #{}", get_int_reg_name(self.rs2), self.imm),
            (_, 0) => format!("bnez {}, #{}", get_int_reg_name(self.rs1), self.imm),
            (_, _) => format!("bne {},{},{}", get_int_reg_name(self.rs1), get_int_reg_name(self.rs2), self.imm),
        }
    }
}

#[allow(dead_code)]
pub struct BLT {
    pub rs1: usize,
    pub rs2: usize,
    pub imm: u32,
}

#[allow(dead_code)]
impl Op for BLT {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1) as i32;
        let src2 = core.int_reg.read(self.rs2) as i32;

        if src1 < src2 {
            core.next_pc = core.pc.wrapping_add(self.imm);
        }
    }
}

#[allow(dead_code)]
impl ToString for BLT {
    fn to_string(&self) -> String {
        match (self.rs1, self.rs2) {
            (0, _) => format!("bltz {}, #{}", get_int_reg_name(self.rs2), self.imm),
            (_, 0) => format!("bltz {}, #{}", get_int_reg_name(self.rs1), self.imm),
            (_, _) => format!("blt {},{},{}", get_int_reg_name(self.rs1), get_int_reg_name(self.rs2), self.imm),
        }
    }
}

#[allow(dead_code)]
pub struct BGE {
    pub rs1: usize,
    pub rs2: usize,
    pub imm: u32,
}

#[allow(dead_code)]
impl Op for BGE {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1) as i32;
        let src2 = core.int_reg.read(self.rs2) as i32;

        if src1 >= src2 {
            core.next_pc = core.pc.wrapping_add(self.imm);
        }
    }
}

#[allow(dead_code)]
impl ToString for BGE {
    fn to_string(&self) -> String {
        match (self.rs1, self.rs2) {
            (0, _) => format!("bgez {}, #{}", get_int_reg_name(self.rs2), self.imm),
            (_, 0) => format!("bgez {}, #{}", get_int_reg_name(self.rs1), self.imm),
            (_, _) => format!("bge {},{},{}", get_int_reg_name(self.rs1), get_int_reg_name(self.rs2), self.imm),
        }
    }
}

#[allow(dead_code)]
pub struct BLTU {
    pub rs1: usize,
    pub rs2: usize,
    pub imm: u32,
}

#[allow(dead_code)]
impl Op for BLTU {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1);
        let src2 = core.int_reg.read(self.rs2);

        if src1 < src2 {
            core.next_pc = core.pc.wrapping_add(self.imm);
        }
    }
}

#[allow(dead_code)]
impl ToString for BLTU {
    fn to_string(&self) -> String {
        format!("bltu {},{},{}", get_int_reg_name(self.rs1), get_int_reg_name(self.rs2), self.imm)
    }
}

#[allow(dead_code)]
pub struct BGEU {
    pub rs1: usize,
    pub rs2: usize,
    pub imm: u32,
}

#[allow(dead_code)]
impl Op for BGEU {
    fn execute(&self, core: &mut Core) {
        let src1 = core.int_reg.read(self.rs1);
        let src2 = core.int_reg.read(self.rs2);

        if src1 >= src2 {
            core.next_pc = core.pc.wrapping_add(self.imm);
        }
    }
}

#[allow(dead_code)]
impl ToString for BGEU {
    fn to_string(&self) -> String {
        format!("bgeu {},{},{}", get_int_reg_name(self.rs1), get_int_reg_name(self.rs2), self.imm)
    }
}

#[allow(dead_code)]
pub struct LB {
    pub rd: usize,
    pub rs1: usize,
    pub imm: u32,
}

#[allow(dead_code)]
impl Op for LB {
    fn execute(&self, core: &mut Core) {
        let addr = core.int_reg.read(self.rs1) + self.imm;
        let value = sign_extend(8, core.bus.read_u8(addr) as u32);

        core.int_reg.write(self.rd, value);
    }
}

#[allow(dead_code)]
impl ToString for LB {
    fn to_string(&self) -> String {
        format!("lb {},{}({})", get_int_reg_name(self.rd), self.imm, get_int_reg_name(self.rs1))
    }
}

#[allow(dead_code)]
pub struct LH {
    pub rd: usize,
    pub rs1: usize,
    pub imm: u32,
}

#[allow(dead_code)]
impl Op for LH {
    fn execute(&self, core: &mut Core) {
        let addr = core.int_reg.read(self.rs1) + self.imm;
        let value = sign_extend(16, core.bus.read_u16(addr) as u32);

        core.int_reg.write(self.rd, value);
    }
}

#[allow(dead_code)]
impl ToString for LH {
    fn to_string(&self) -> String {
        format!("lh {},{}({})", get_int_reg_name(self.rd), self.imm, get_int_reg_name(self.rs1))
    }
}

#[allow(dead_code)]
pub struct LW {
    pub rd: usize,
    pub rs1: usize,
    pub imm: u32,
}

#[allow(dead_code)]
impl Op for LW {
    fn execute(&self, core: &mut Core) {
        let addr = core.int_reg.read(self.rs1) + self.imm;
        let value = core.bus.read_u32(addr);

        core.int_reg.write(self.rd, value);
    }
}

#[allow(dead_code)]
impl ToString for LW {
    fn to_string(&self) -> String {
        format!("lw {},{}({})", get_int_reg_name(self.rd), self.imm, get_int_reg_name(self.rs1))
    }
}

#[allow(dead_code)]
pub struct LBU {
    pub rd: usize,
    pub rs1: usize,
    pub imm: u32,
}

#[allow(dead_code)]
impl Op for LBU {
    fn execute(&self, core: &mut Core) {
        let addr = core.int_reg.read(self.rs1) + self.imm;
        let value = core.bus.read_u8(addr) as u32;

        core.int_reg.write(self.rd, value);
    }
}

#[allow(dead_code)]
impl ToString for LBU {
    fn to_string(&self) -> String {
        format!("lbu {},{}({})", get_int_reg_name(self.rd), self.imm, get_int_reg_name(self.rs1))
    }
}

#[allow(dead_code)]
pub struct LHU {
    pub rd: usize,
    pub rs1: usize,
    pub imm: u32,
}

#[allow(dead_code)]
impl Op for LHU {
    fn execute(&self, core: &mut Core) {
        let addr = core.int_reg.read(self.rs1) + self.imm;
        let value = core.bus.read_u16(addr) as u32;

        core.int_reg.write(self.rd, value);
    }
}

#[allow(dead_code)]
impl ToString for LHU {
    fn to_string(&self) -> String {
        format!("lhu {},{}({})", get_int_reg_name(self.rd), self.imm, get_int_reg_name(self.rs1))
    }
}
