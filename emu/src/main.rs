extern crate byteorder;

use std::io::prelude::*;
use std::io::Cursor;
use std::fs::File;
use byteorder::{LittleEndian, ReadBytesExt};

type RegId = u8;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
enum RvInsn {
    li { rd: RegId },
}

fn fetch(main_memory: &Vec<u8>, addr: u64) -> u32 {
    let mut cursor = Cursor::new(main_memory);
    Cursor::set_position(&mut cursor, addr);

    cursor.read_u32::<LittleEndian>().unwrap()
}

fn display(main_memory: &Vec<u8>) {
    let count = 16 as u64;

    for i in 0..count {
        let value = fetch(main_memory, i * 4);
        println!("{:08x}", value);
    }
}

fn main() {
    let main_memory_size = 65536;

    let path = "../../rafi-emu-rust-data/rv32ui-p-add.bin";
    println!("Read {}", path);

    let mut f = File::open(path).unwrap();
    let mut main_memory: Vec<u8> = vec![0xff; main_memory_size];

    f.read(&mut main_memory[..]).unwrap();

    display(&main_memory);
}
