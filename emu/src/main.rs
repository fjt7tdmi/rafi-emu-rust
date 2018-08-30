extern crate byteorder;

use std::io::prelude::*;
use std::io::Cursor;
use std::fs::File;
use byteorder::{LittleEndian, ReadBytesExt};

type CsrId = u16;
type RegId = u8;
type Imm = u32;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Debug)]
enum RvOp {
    UNKNOWN { },
    // RV32I
    LUI     { imm: Imm, rd: RegId },
    AUIPC   { imm: Imm, rd: RegId },
    JAL     { imm: Imm, rd: RegId },
    JALR    { imm: Imm, rd: RegId },
    BEQ     { imm: Imm, rd: RegId, rs1: RegId, rs2: RegId },
    BNE     { imm: Imm, rd: RegId, rs1: RegId, rs2: RegId },
    BLT     { imm: Imm, rd: RegId, rs1: RegId, rs2: RegId },
    BGE     { imm: Imm, rd: RegId, rs1: RegId, rs2: RegId },
    BLTU    { imm: Imm, rd: RegId, rs1: RegId, rs2: RegId },
    BGEU    { imm: Imm, rd: RegId, rs1: RegId, rs2: RegId },
    LB      { imm: Imm, rd: RegId, rs1: RegId },
    LH      { imm: Imm, rd: RegId, rs1: RegId },
    LW      { imm: Imm, rd: RegId, rs1: RegId },
    LBU     { imm: Imm, rd: RegId, rs1: RegId },
    LHU     { imm: Imm, rd: RegId, rs1: RegId },
    SB      { imm: Imm, rs1: RegId, rs2: RegId },
    SH      { imm: Imm, rs1: RegId, rs2: RegId },
    SW      { imm: Imm, rs1: RegId, rs2: RegId },
    ADDI    { imm: Imm, rd: RegId, rs1: RegId },
    SLTI    { imm: Imm, rd: RegId, rs1: RegId },
    SLTIU   { imm: Imm, rd: RegId, rs1: RegId },
    XORI    { imm: Imm, rd: RegId, rs1: RegId },
    ORI     { imm: Imm, rd: RegId, rs1: RegId },
    ANDI    { imm: Imm, rd: RegId, rs1: RegId },
    SLLI    { rd: RegId, rs1: RegId, shamt: Imm },
    SRLI    { rd: RegId, rs1: RegId, shamt: Imm },
    SRAI    { rd: RegId, rs1: RegId, shamt: Imm },
    ADD     { rd: RegId, rs1: RegId, rs2: RegId },
    SUB     { rd: RegId, rs1: RegId, rs2: RegId },
    SLL     { rd: RegId, rs1: RegId, rs2: RegId },
    SLT     { rd: RegId, rs1: RegId, rs2: RegId },
    SLTU    { rd: RegId, rs1: RegId, rs2: RegId },
    XOR     { rd: RegId, rs1: RegId, rs2: RegId },
    SRL     { rd: RegId, rs1: RegId, rs2: RegId },
    SRA     { rd: RegId, rs1: RegId, rs2: RegId },
    OR      { rd: RegId, rs1: RegId, rs2: RegId },
    AND     { rd: RegId, rs1: RegId, rs2: RegId },
    FENCE   { pred: Imm, succ: Imm },
    FENCE_I { },
    ECALL   { },
    EBRAK   { },
    CSRRW   { csr: CsrId, rd: RegId, rs1: RegId },
    CSRRS   { csr: CsrId, rd: RegId, rs1: RegId },
    CSRRC   { csr: CsrId, rd: RegId, rs1: RegId },
    CSRRWI  { csr: CsrId, imm: Imm, rd: RegId },
    CSRRSI  { csr: CsrId, imm: Imm, rd: RegId },
    CSRRCI  { csr: CsrId, imm: Imm, rd: RegId },
}

fn take_bit(value: &u32, lsb: usize, width: usize) -> u32 {
    (value >> lsb) & ((1 << width) - 1)
}

fn fetch(main_memory: &Vec<u8>, addr: u64) -> u32 {
    let mut cursor = Cursor::new(main_memory);
    Cursor::set_position(&mut cursor, addr);

    cursor.read_u32::<LittleEndian>().unwrap()
}

fn decode(insn: &u32) -> RvOp {
    let opcode  = take_bit(insn, 0, 7);
    let rd      = take_bit(insn, 7, 5);

    match opcode {
        0b0110111 =>
            RvOp::LUI {
                imm: take_bit(insn, 12, 20) << 12,
                rd: rd as RegId,
            },
        _ => RvOp::UNKNOWN { }
    }
}

fn display(main_memory: &Vec<u8>) {
    let count = 16 as u64;

    for i in 0..count {
        let insn = fetch(main_memory, i * 4);
        let op = decode(&insn);
        println!("{:08x}", insn);
        println!("{:?}", op);
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
