use op::*;
use util::*;

pub fn decode(insn: &u32) -> Box<dyn Op> {
    let opcode  = pick(insn, 0, 7);
    let rd      = pick(insn, 7, 5) as usize;
    let funct3  = pick(insn, 12, 3);
    let rs1     = pick(insn, 15, 5) as usize;
    let _rs2     = pick(insn, 20, 5) as usize;
    let _funct7  = pick(insn, 25, 7);

    match opcode {
        0b0110111 => {
            let imm = pick(insn, 12, 20) << 12;
            Box::new(LUI{ rd: rd, imm: imm })
        },
        0b0010111 => {
            let imm = pick(insn, 12, 20) << 12;
            Box::new(AUIPC{ rd: rd, imm: imm })
        },
        0b1101111 => {
            let imm =
                pick(insn, 31, 1) << 20 |
                pick(insn, 21, 10) << 1 |
                pick(insn, 20, 1) << 12 |
                pick(insn, 12, 8) << 12;
            Box::new(JAL{ rd: rd, imm: imm })
        },
        0b1100111 => {
            let imm = pick(insn, 20, 12);
            match funct3 {
                0b000 => Box::new(JALR{ rd: rd, rs1: rs1, imm: imm }),
                _ => Box::new(UnknownOp{}),
            }
        },
        _ => Box::new(UnknownOp{}),
    }
}
