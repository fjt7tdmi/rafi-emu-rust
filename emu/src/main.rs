extern crate byteorder;

mod util;

use byteorder::{LittleEndian, ReadBytesExt};
use std::io::prelude::*;
use std::io::Cursor;
use std::fs::File;
use util::*;

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
    JALR    { imm: Imm, rd: RegId, rs1: RegId },
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
    EBREAK  { },
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
    let rd      = take_bit(insn, 7, 5) as RegId;
    let funct3  = take_bit(insn, 12, 3);
    let rs1     = take_bit(insn, 15, 5) as RegId;
    let rs2     = take_bit(insn, 20, 5) as RegId;
    let funct7  = take_bit(insn, 25, 7);

    // TODO: sext
    match opcode {
        0b0110111 => {
            let imm = take_bit(insn, 12, 20) << 12;
            RvOp::LUI { imm: imm, rd: rd }
        },
        0b0010111 => RvOp::AUIPC {
            imm: take_bit(insn, 12, 20) << 12,
            rd: rd,
        },
        0b1101111 => {
            let imm = take_bit(insn, 31, 1) << 20 | take_bit(insn, 21, 10) << 1 | take_bit(insn, 20, 1) << 12 | take_bit(insn, 12, 8) << 12;
            RvOp::JAL { imm: imm, rd: rd, }
        }
        0b1100111 => {
            let imm = take_bit(insn, 20, 12);
            match funct3 {
                0b000 => RvOp::JALR { imm: imm, rd: rd, rs1: rs1 },
                _ => RvOp::UNKNOWN { },
            }
        },
        0b1100011 => {
            let imm = sext(12, take_bit(insn, 20, 12));
            match funct3 {
                0b000 => RvOp::LB{ imm: imm, rd: rd, rs1: rs1 },
                0b001 => RvOp::LH{ imm: imm, rd: rd, rs1: rs1 },
                0b010 => RvOp::LW{ imm: imm, rd: rd, rs1: rs1 },
                0b100 => RvOp::LBU{ imm: imm, rd: rd, rs1: rs1 },
                0b101 => RvOp::LHU{ imm: imm, rd: rd, rs1: rs1 },
                _ => RvOp::UNKNOWN { },
            }
        },
        0b0100011 => {
            let imm = sext(12, take_bit(insn, 25, 7) << 5 | take_bit(insn, 7, 5));
            match funct3 {
                0b000 => RvOp::SB{ imm: imm, rs1: rs1, rs2: rs2 },
                0b001 => RvOp::SH{ imm: imm, rs1: rs1, rs2: rs2 },
                0b010 => RvOp::SW{ imm: imm, rs1: rs1, rs2: rs2 },
                _ => RvOp::UNKNOWN { },
            }
        },
        0b0010011 => {
            let imm = sext(12, take_bit(insn, 20, 12));
            let shamt = take_bit(insn, 20, 5);
            match (funct3, funct7) {
                (0b000, _) => RvOp::ADDI { imm: imm, rd: rd, rs1: rs1 },
                (0b010, _) => RvOp::SLTI { imm: imm, rd: rd, rs1: rs1 },
                (0b011, _) => RvOp::SLTIU{ imm: imm, rd: rd, rs1: rs1 },
                (0b100, _) => RvOp::XORI { imm: imm, rd: rd, rs1: rs1 },
                (0b110, _) => RvOp::ORI  { imm: imm, rd: rd, rs1: rs1 },
                (0b111, _) => RvOp::ANDI { imm: imm, rd: rd, rs1: rs1 },
                (0b001, 0b0000000) => RvOp::SLLI { rd: rd, rs1: rs1, shamt: shamt },
                (0b101, 0b0000000) => RvOp::SRLI { rd: rd, rs1: rs1, shamt: shamt },
                (0b101, 0b0000001) => RvOp::SRAI { rd: rd, rs1: rs1, shamt: shamt },
                _ => RvOp::UNKNOWN { },
            }
        },
        0b0110011 => {
            match (funct3, funct7) {
                (0b000, 0b0000000) => RvOp::ADD { rd: rd, rs1: rs1, rs2: rs2 },
                (0b000, 0b0100000) => RvOp::SUB { rd: rd, rs1: rs1, rs2: rs2 },
                (0b001, 0b0000000) => RvOp::SLL { rd: rd, rs1: rs1, rs2: rs2 },
                (0b010, 0b0000000) => RvOp::SLT { rd: rd, rs1: rs1, rs2: rs2 },
                (0b011, 0b0000000) => RvOp::SLTU{ rd: rd, rs1: rs1, rs2: rs2 },
                (0b100, 0b0000000) => RvOp::XOR { rd: rd, rs1: rs1, rs2: rs2 },
                (0b101, 0b0000000) => RvOp::SRL { rd: rd, rs1: rs1, rs2: rs2 },
                (0b101, 0b0100000) => RvOp::SRA { rd: rd, rs1: rs1, rs2: rs2 },
                (0b110, 0b0000000) => RvOp::OR  { rd: rd, rs1: rs1, rs2: rs2 },
                (0b111, 0b0000000) => RvOp::AND { rd: rd, rs1: rs1, rs2: rs2 },
                _ => RvOp::UNKNOWN { },
            }
        },
        0b0001111 => {
            let head = take_bit(insn, 28, 4);
            let pred = take_bit(insn, 28, 4);
            let succ = take_bit(insn, 28, 4);
            match (funct3, rs1, rd, head, pred, succ) {
                (0b000, 0b00000, 0b00000, 0b0000, _, _) => RvOp::FENCE { pred: pred, succ: succ },
                (0b001, 0b00000, 0b00000, 0b0000, 0b00000, 0b00000) => RvOp::FENCE_I { },
                _ => RvOp::UNKNOWN { },
            }
        },
        0b1110011 => {
            let csr = take_bit(insn, 20, 12) as CsrId;
            let zimm = take_bit(insn, 15, 5);
            match(funct3, csr, rs1, rd) {
                (0b000, 0b000000000000, 0b00000, 0b00000) => RvOp::ECALL{ },
                (0b000, 0b000000000001, 0b00000, 0b00000) => RvOp::EBREAK{ },
                (0b001, _, _, _) => RvOp::CSRRW { csr: csr, rd: rd, rs1: rs1 },
                (0b010, _, _, _) => RvOp::CSRRS { csr: csr, rd: rd, rs1: rs1 },
                (0b011, _, _, _) => RvOp::CSRRC { csr: csr, rd: rd, rs1: rs1 },
                (0b101, _, _, _) => RvOp::CSRRWI{ csr: csr, rd: rd, imm: zimm },
                (0b110, _, _, _) => RvOp::CSRRSI{ csr: csr, rd: rd, imm: zimm },
                (0b111, _, _, _) => RvOp::CSRRCI{ csr: csr, rd: rd, imm: zimm },
                _ => RvOp::UNKNOWN { },
            }
        },
        // TODO: impl
        _ => RvOp::UNKNOWN { },
    }
}

fn display(main_memory: &Vec<u8>) {
    let count = 16 as u64;

    for i in 0..count {
        let insn = fetch(main_memory, i * 4);
        let op = decode(&insn);
        //println!("{:08x}", insn);
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
