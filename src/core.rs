use bus::*;
use csr::*;

pub struct IntReg {
    values: [u32; 32],
}

impl IntReg {
    pub fn new() -> IntReg {
        IntReg { values: [0; 32] }
    }

    pub fn read(&self, index: usize) -> u32 {
        self.values[index]
    }

    pub fn write(&mut self, index: usize, value: u32) {
        if index != 0 {
            self.values[index] = value
        }
    }
}

#[test]
fn test_int_reg() {
    let mut reg = IntReg::new();
    reg.write(0, 100);
    reg.write(1, 200);
    assert_eq!(reg.read(0), 0);
    assert_eq!(reg.read(1), 200);
}

pub struct Core<'a> {
    pub csr: Csr,
    pub int_reg: IntReg,
    pub pc: u32,
    pub next_pc: u32,
    pub bus: &'a mut Bus<'a>,
    pub host_io_addr: u32,
}

impl<'a> Core<'a> {
    pub fn new(bus: &'a mut Bus<'a>) -> Core<'a> {
        Core { csr: Csr::new(), int_reg: IntReg::new(), pc: 0, next_pc: 0, bus: bus, host_io_addr: 0 }
    }

    pub fn fetch(&self) -> u32 {
        self.bus.read_u32(self.pc)
    }

    pub fn read_host_io(&self) -> u32 {
        self.bus.read_u32(self.host_io_addr)
    }
}
