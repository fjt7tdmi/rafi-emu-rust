extern crate byteorder;

mod bus;
mod core;
mod decoder;
mod memory;
mod op;
mod util;

use bus::*;
use core::*;
use decoder::*;
use memory::*;

fn emulate(path: String) {
    let max_cycle = 1000;
    let host_io_addr = 0x1000;

    let mut memory = Memory::new();
    memory.load_file(path);

    let mut bus = Bus::new(&mut memory);
    let mut core = Core::new(&mut bus);

    for _i in 0..max_cycle {
        let insn = core.fetch();
        let op = decode(&insn);
        
        println!("{}", op.to_string());

        op.execute(&mut core);
    }

    println!("HostIo: {}", memory.read_u32(host_io_addr));
}

fn main() {
    let path = "rafi-prebuilt-binary/riscv-tests/isa/rv32ui-p-add.bin";
    emulate(path.to_string());
}
