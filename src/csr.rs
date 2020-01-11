// Num of CSRs
const NUM_CSR: usize = 0x1000;

// CSR Index definitions
const CSR_INDEX_MSTATUS : usize = 0x300;
const CSR_INDEX_MTVEC   : usize = 0x305;
const CSR_INDEX_MEPC    : usize = 0x341;
const CSR_INDEX_MCAUSE  : usize = 0x342;
const CSR_INDEX_MTVAL   : usize = 0x343;

// register definitions
bitflags! {
    pub struct MSTATUS: u32 {
        const SD    = 0b1000_0000_0000_0000_0000_0000_0000_0000;
        const TSR   = 0b0000_0000_0100_0000_0000_0000_0000_0000;
        const TW    = 0b0000_0000_0010_0000_0000_0000_0000_0000;
        const TVM   = 0b0000_0000_0001_0000_0000_0000_0000_0000;
        const MXR   = 0b0000_0000_0000_1000_0000_0000_0000_0000;
        const SUM   = 0b0000_0000_0000_0100_0000_0000_0000_0000;
        const MPRV  = 0b0000_0000_0000_0010_0000_0000_0000_0000;
        const XS    = 0b0000_0000_0000_0001_1000_0000_0000_0000;
        const FS    = 0b0000_0000_0000_0000_0110_0000_0000_0000;
        const MPP   = 0b0000_0000_0000_0000_0001_1000_0000_0000;
        const SPP   = 0b0000_0000_0000_0000_0000_0001_0000_0000;
        const MPIE  = 0b0000_0000_0000_0000_0000_0000_1000_0000;
        const SPIE  = 0b0000_0000_0000_0000_0000_0000_0010_0000;
        const UPIE  = 0b0000_0000_0000_0000_0000_0000_0001_0000;
        const MIE   = 0b0000_0000_0000_0000_0000_0000_0000_1000;
        const SIE   = 0b0000_0000_0000_0000_0000_0000_0000_0010;
        const UIE   = 0b0000_0000_0000_0000_0000_0000_0000_0001;
    }
}

bitflags! {
    pub struct MTVEC: u32 {
        const BASE = 0xffff_fffc;
        const MODE = 0x0000_0003;
    }
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
        MSTATUS::from_bits_truncate(self.read(CSR_INDEX_MSTATUS))
    }

    pub fn write_mstatus(&mut self, value: MTVEC) {
        self.write(CSR_INDEX_MSTATUS, value.bits())
    }

    pub fn read_mtvec(&self) -> MTVEC {
        MTVEC::from_bits_truncate(self.read(CSR_INDEX_MTVEC))
    }

    pub fn write_mtvec(&mut self, value: MTVEC) {
        self.write(CSR_INDEX_MTVEC, value.bits())
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
