// Num of CSRs
const NUM_CSR: usize = 0x1000;

// CSR Index definitions
const CSR_INDEX_MSTATUS : usize = 0x300;
const CSR_INDEX_MTVEC   : usize = 0x305;
const CSR_INDEX_MEPC    : usize = 0x341;
const CSR_INDEX_MCAUSE  : usize = 0x342;
const CSR_INDEX_MTVAL   : usize = 0x343;

// register definitions
bitfield! {
    pub struct MSTATUS(u32);
    impl Debug;
    pub sd,     set_sd:     31, 31;
    pub tsr,    set_tsr:    22, 22;
    pub tw,     set_tw:     21, 21;
    pub tvm,    set_tvm:    20, 20;
    pub mxr,    set_mxr:    19, 19;
    pub sum,    set_sum:    18, 18;
    pub mprv,   set_mprv:   17, 17;
    pub xs,     set_xs:     16, 15;
    pub fs,     set_fs:     14, 13;
    pub mpp,    set_mpp:    12, 11;
    pub spp,    set_spp:     8,  8;
    pub mpie,   set_mpie:    7,  7;
    pub spie,   set_spie:    5,  5;
    pub upie,   set_upie:    4,  4;
    pub mie,    set_mie:     3,  3;
    pub sie,    set_sie:     1,  1;
    pub uie,    set_uei:     0,  0;
}

bitfield! {
    pub struct MTVEC(u32);
    impl Debug;
    pub base,   set_base:   31,  2;
    pub mode,   set_mode:    1,  0;
}

// CSR struct definition
pub struct Csr {
    values: [u32; NUM_CSR],
}

#[allow(dead_code)]
impl Csr {
    pub fn new() -> Csr {
        Csr { values: [0; NUM_CSR] }
    }

    pub fn read(&self, index: usize) -> u32 {
        self.values[index]
    }

    pub fn write(&mut self, index: usize, value: u32) {
        self.values[index] = value
    }

    pub fn read_mstatus(&self) -> MSTATUS {
        MSTATUS(self.read(CSR_INDEX_MSTATUS))
    }

    pub fn write_mstatus(&mut self, value: MSTATUS) {
        self.write(CSR_INDEX_MSTATUS, value.0)
    }

    pub fn read_mtvec(&self) -> MTVEC {
        MTVEC(self.read(CSR_INDEX_MTVEC))
    }

    pub fn write_mtvec(&mut self, value: MTVEC) {
        self.write(CSR_INDEX_MTVEC, value.0)
    }

    pub fn read_mepc(&self) -> u32 {
        self.read(CSR_INDEX_MEPC)
    }

    pub fn write_mepc(&mut self, value: u32) {
        self.write(CSR_INDEX_MEPC, value)
    }

    pub fn read_mcause(&self) -> u32 {
        self.read(CSR_INDEX_MCAUSE)
    }

    pub fn write_mcause(&mut self, value: u32) {
        self.write(CSR_INDEX_MCAUSE, value)
    }

    pub fn read_mtval(&self) -> u32 {
        self.read(CSR_INDEX_MTVAL)
    }

    pub fn write_mtval(&mut self, value: u32) {
        self.write(CSR_INDEX_MTVAL, value)
    }
}
