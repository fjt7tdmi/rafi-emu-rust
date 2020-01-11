#[macro_use]
extern crate bitfield;

extern crate byteorder;
extern crate serde;
extern crate serde_json;

mod bus;
mod core;
mod csr;
mod decoder;
mod memory;
mod op;
mod trap;
mod util;

use bus::*;
use core::*;
use decoder::*;
use memory::*;
use trap::*;

use std::fs::File;
use std::io::BufReader;
use std::process::exit;

fn emulate(path: String) -> u32 {
    const MAX_CYCLE: u32 = 1000;
    const HOST_IO_ADDR: u32 = 0x80001000;
    const INITIAL_PC: u32 = 0x8000_0000;

    let mut memory = Memory::new();
    memory.load_file(path);

    let mut bus = Bus::new(&mut memory);
    let mut core = Core::new(&mut bus);

    core.host_io_addr = HOST_IO_ADDR;
    core.pc = INITIAL_PC;

    for _i in 0..MAX_CYCLE {
        if core.read_host_io() != 0 {
            break
        }

        let insn = core.fetch();
        let op = decode(&insn);

        // Currently, 2-byte ops are not supported
        core.next_pc = core.pc + 4;

        op.execute(&mut core);

        match op.post_check_trap(&mut core) {
            Some(trap) => process_trap(&mut core, &trap),
            None => (),
        }

        core.pc = core.next_pc;
    }

    println!("HostIo: {}", core.read_host_io());
    core.read_host_io()
}


fn get_tests() -> Vec<String> {
    let file = File::open("riscv_tests.json").unwrap();
    let reader = BufReader::new(file);
    let tests: Vec<String> = serde_json::from_reader(reader).unwrap();
    
    tests
}

fn main() {
    let mut failed_test_num: i32 = 0;

    let tests = get_tests();
    for test in tests {
        let path = format!("rafi-prebuilt-binary/riscv-tests/isa/{}.bin", test);
        println!("{}", path);

        if emulate(path.to_string()) != 1 {
            failed_test_num += 1;
        }
    }

    exit(failed_test_num)
}
