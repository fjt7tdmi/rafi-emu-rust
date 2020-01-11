extern crate byteorder;

mod bus;
mod core;
mod csr;
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
    let initial_pc = 0x8000_0000;

    let mut memory = Memory::new();
    memory.load_file(path);

    let mut bus = Bus::new(&mut memory);
    let mut core = Core::new(&mut bus);

    core.pc = initial_pc;

    for _i in 0..max_cycle {
        let insn = core.fetch();
        let op = decode(&insn);

        // Currently, 2-byte ops are not supported
        core.next_pc = core.pc + 4;

        println!("0x{:x}: {}", core.pc, op.to_string());

        op.execute(&mut core);

        core.pc = core.next_pc;
    }

    println!("HostIo: {}", memory.read_u32(host_io_addr));
}

fn main() {
    let path = "rafi-prebuilt-binary/riscv-tests/isa/rv32ui-p-add.bin";
    emulate(path.to_string());
}
