const N_CSR: usize = 0x1000;

pub struct Csr {
    values: [u32; N_CSR],
}

impl Csr {
    pub fn new() -> Csr {
        Csr { values: [0; N_CSR] }
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
