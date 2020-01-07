use core::*;

pub trait Op {
    fn execute(&self, core: &mut Core);
}

pub struct UnknownOp {
}

impl Op for UnknownOp {
    fn execute(&self, _core: &mut Core) {
        panic!("execute unknown op.");
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

#[test]
fn test_lui() {
    let mut core = Core::new();
    let op = LUI { rd: 1, imm: 0x12340000 };

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

#[test]
fn test_auipc() {
    let mut core = Core::new();
    let op = AUIPC { rd: 1, imm: 0x80000000 };

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
