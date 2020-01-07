use memory::*;

#[allow(dead_code)]
pub struct Bus<'a> {
    pub memory: &'a Memory,
}

#[allow(dead_code)]
impl Bus<'_> {
    pub fn new(memory: &mut Memory) -> Bus {
        Bus { memory: memory }
    }

    pub fn read_u8(&self, addr: u32) -> u8 {
        self.memory.read_u8(addr.wrapping_sub(0x8000_0000) as u64)
    }

    pub fn read_u16(&self, addr: u32) -> u16 {
        self.memory.read_u16(addr.wrapping_sub(0x8000_0000) as u64)
    }

    pub fn read_u32(&self, addr: u32) -> u32 {
        self.memory.read_u32(addr.wrapping_sub(0x8000_0000) as u64)
    }
}
