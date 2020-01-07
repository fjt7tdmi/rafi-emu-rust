extern crate byteorder;

mod core;
mod decoder;
mod memory;
mod op;
mod util;

use core::*;
use decoder::*;
use memory::*;

fn emulate(path: String) {
    let max_cycle = 1000;
    let host_io_addr = 0x1000;

    let mut memory = Memory::new();
    let mut core = Core::new();

    println!("Read {}", path);
    memory.load_file(path);

    for i in 0..max_cycle {
        let insn = memory.read_u32(i * 4);
        let op = decode(&insn);
        
        op.execute(&mut core);
    }

    println!("HostIo: {}", memory.read_u32(host_io_addr));
}

fn main() {
    let path = "rafi-prebuilt-binary/riscv-tests/isa/rv32ui-p-add.bin";
    emulate(path.to_string());
}
