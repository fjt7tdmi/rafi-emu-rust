use byteorder::{LittleEndian, ReadBytesExt};
use std::io::prelude::*;
use std::io::Cursor;
use std::fs::File;

pub struct Memory {
    pub body: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        let memory_size = 65536;

        Memory { body: vec![0xff; memory_size] }
    }

    pub fn load_file(&mut self, path: String) {
        let mut f = File::open(path).unwrap();

        f.read(&mut self.body[..]).unwrap();    
    }

    #[allow(dead_code)]
    pub fn read_u8(&self, addr: u64) -> u8 {
        let mut cursor = Cursor::new(&self.body);
        Cursor::set_position(&mut cursor, addr);
    
        cursor.read_u8().unwrap()
    }

    #[allow(dead_code)]
    pub fn read_u16(&self, addr: u64) -> u16 {
        let mut cursor = Cursor::new(&self.body);
        Cursor::set_position(&mut cursor, addr);
    
        cursor.read_u16::<LittleEndian>().unwrap()
    }

    pub fn read_u32(&self, addr: u64) -> u32 {
        let mut cursor = Cursor::new(&self.body);
        Cursor::set_position(&mut cursor, addr);
    
        cursor.read_u32::<LittleEndian>().unwrap()
    }
}
