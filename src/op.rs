use core::*;

trait Op {
    fn execute(&self, core: &mut Core);
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
fn test_op() {
    let mut core = Core::new();
    let op = LUI { rd: 1, imm: 0x12340000 };

    op.execute(&mut core);

    assert_eq!(core.int_reg.read(1), 0x12340000);
}