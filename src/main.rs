extern crate byteorder;

use byteorder::{LittleEndian, ReadBytesExt};
use std::io::prelude::*;
use std::io::Cursor;
use std::fs::File;

mod core;
mod decoder;
mod op;
mod util;

use core::*;
use decoder::*;

fn fetch(memory: &Vec<u8>, addr: u64) -> u32 {
    let mut cursor = Cursor::new(memory);
    Cursor::set_position(&mut cursor, addr);

    cursor.read_u32::<LittleEndian>().unwrap()
}

fn emulate(path: String) {
    let memory_size = 65536;
    let max_cycle = 1000;

    let mut core = Core::new();
    let mut memory: Vec<u8> = vec![0xff; memory_size];

    println!("Read {}", path);
    let mut f = File::open(path).unwrap();

    f.read(&mut memory[..]).unwrap();

    for i in 0..max_cycle {
        let insn = fetch(&memory, i * 4);
        let op = decode(&insn);
        
        op.execute(&mut core);
    }

    println!("HostIo: {}", fetch(&memory, 0x1000));
}

fn main() {
    let path = "rafi-prebuilt-binary/riscv-tests/isa/rv32ui-p-add.bin";
    emulate(path.to_string());
}
