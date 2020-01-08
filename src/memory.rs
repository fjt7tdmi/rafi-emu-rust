use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
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

    pub fn read_u8(&self, addr: u64) -> u8 {
        let mut cursor = Cursor::new(&self.body);
        Cursor::set_position(&mut cursor, addr);
    
        cursor.read_u8().unwrap()
    }

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

    pub fn write_u8(&mut self, addr: u64, value: u8) {
        let mut cursor = Cursor::new(&mut self.body);
        Cursor::set_position(&mut cursor, addr);
    
        cursor.write_u8(value).unwrap();
    }

    pub fn write_u16(&mut self, addr: u64, value: u16) {
        let mut cursor = Cursor::new(&mut self.body);
        Cursor::set_position(&mut cursor, addr);
    
        cursor.write_u16::<LittleEndian>(value).unwrap();
    }

    pub fn write_u32(&mut self, addr: u64, value: u32) {
        let mut cursor = Cursor::new(&mut self.body);
        Cursor::set_position(&mut cursor, addr);
    
        cursor.write_u32::<LittleEndian>(value).unwrap();
    }
}
