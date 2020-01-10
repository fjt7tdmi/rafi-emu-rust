use op::*;
use util::*;

pub fn decode(insn: &u32) -> Box<dyn Op> {
    let opcode  = pick(insn, 0, 7);
    let rd      = pick(insn, 7, 5) as usize;
    let funct3  = pick(insn, 12, 3);
    let rs1     = pick(insn, 15, 5) as usize;
    let rs2     = pick(insn, 20, 5) as usize;
    let funct7  = pick(insn, 25, 7);

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
        0b1100011 => {
            let imm = sign_extend(13,
                pick(insn, 31, 1) << 12 |
                pick(insn, 7, 1) << 11 |
                pick(insn, 25, 6) << 5 |
                pick(insn, 8, 4) << 1);
            match funct3 {
                0b000 => Box::new(BEQ  { imm: imm, rs1: rs1, rs2: rs2 }),
                0b001 => Box::new(BNE  { imm: imm, rs1: rs1, rs2: rs2 }),
                0b100 => Box::new(BLT  { imm: imm, rs1: rs1, rs2: rs2 }),
                0b101 => Box::new(BGE  { imm: imm, rs1: rs1, rs2: rs2 }),
                0b110 => Box::new(BLTU { imm: imm, rs1: rs1, rs2: rs2 }),
                0b111 => Box::new(BGEU { imm: imm, rs1: rs1, rs2: rs2 }),
                _ => Box::new(UnknownOp{}),
            }
        },
        0b0000011 => {
            let imm = sign_extend(12, pick(insn, 20, 12));
            match funct3 {
                0b000 => Box::new(LB { imm: imm, rd: rd, rs1: rs1 }),
                0b001 => Box::new(LH { imm: imm, rd: rd, rs1: rs1 }),
                0b010 => Box::new(LW { imm: imm, rd: rd, rs1: rs1 }),
                0b100 => Box::new(LBU{ imm: imm, rd: rd, rs1: rs1 }),
                0b101 => Box::new(LHU{ imm: imm, rd: rd, rs1: rs1 }),
                _ => Box::new(UnknownOp{}),
            }
        },
        0b0100011 => {
            let imm = sign_extend(12, pick(insn, 25, 7) << 5 | pick(insn, 7, 5));
            match funct3 {
                0b000 => Box::new(SB{ imm: imm, rs1: rs1, rs2: rs2 }),
                0b001 => Box::new(SH{ imm: imm, rs1: rs1, rs2: rs2 }),
                0b010 => Box::new(SW{ imm: imm, rs1: rs1, rs2: rs2 }),
                _ => Box::new(UnknownOp{}),
            }
        },
        0b0010011 => {
            let imm = sign_extend(12, pick(insn, 20, 12));
            let shamt = pick(insn, 20, 5);
            match (funct3, funct7) {
                (0b000, _) => Box::new(ADDI { imm: imm, rd: rd, rs1: rs1 }),
                (0b010, _) => Box::new(SLTI { imm: imm, rd: rd, rs1: rs1 }),
                (0b011, _) => Box::new(SLTIU{ imm: imm, rd: rd, rs1: rs1 }),
                (0b100, _) => Box::new(XORI { imm: imm, rd: rd, rs1: rs1 }),
                (0b110, _) => Box::new(ORI  { imm: imm, rd: rd, rs1: rs1 }),
                (0b111, _) => Box::new(ANDI { imm: imm, rd: rd, rs1: rs1 }),
                (0b001, 0b0000000) => Box::new(SLLI{ rd: rd, rs1: rs1, shamt: shamt }),
                (0b101, 0b0000000) => Box::new(SRLI{ rd: rd, rs1: rs1, shamt: shamt }),
                (0b101, 0b0000001) => Box::new(SRAI{ rd: rd, rs1: rs1, shamt: shamt }),
                _ => Box::new(UnknownOp{}),
            }
        },
        0b0110011 => {
            match (funct3, funct7) {
                (0b000, 0b0000000) => Box::new(ADD { rd: rd, rs1: rs1, rs2: rs2 }),
                (0b000, 0b0100000) => Box::new(SUB { rd: rd, rs1: rs1, rs2: rs2 }),
                (0b001, 0b0000000) => Box::new(SLL { rd: rd, rs1: rs1, rs2: rs2 }),
                (0b010, 0b0000000) => Box::new(SLT { rd: rd, rs1: rs1, rs2: rs2 }),
                (0b011, 0b0000000) => Box::new(SLTU{ rd: rd, rs1: rs1, rs2: rs2 }),
                (0b100, 0b0000000) => Box::new(XOR { rd: rd, rs1: rs1, rs2: rs2 }),
                (0b101, 0b0000000) => Box::new(SRL { rd: rd, rs1: rs1, rs2: rs2 }),
                (0b101, 0b0100000) => Box::new(SRA { rd: rd, rs1: rs1, rs2: rs2 }),
                (0b110, 0b0000000) => Box::new(OR  { rd: rd, rs1: rs1, rs2: rs2 }),
                (0b111, 0b0000000) => Box::new(AND { rd: rd, rs1: rs1, rs2: rs2 }),
                _ => Box::new(UnknownOp{}),
            }
        },
        0b0001111 => {
            let head = pick(insn, 28, 4);
            let pred = pick(insn, 28, 4);
            let succ = pick(insn, 28, 4);
            match (funct3, rs1, rd, head, pred, succ) {
                (0b000, 0b00000, 0b00000, 0b0000, _, _) => Box::new(FENCE { pred: pred, succ: succ }),
                (0b001, 0b00000, 0b00000, 0b0000, 0b00000, 0b00000) => Box::new(FENCEI { }),
                _ => Box::new(UnknownOp{}),
            }
        },
        0b1110011 => {
            let csr = pick(insn, 20, 12) as usize;
            let zimm = pick(insn, 15, 5);
            match(funct3, funct7, rs2, rs1, rd) {
                (0b000, 0b0000000, 0b00000, 0b00000, 0b00000) => Box::new(ECALL{}),
                (0b000, 0b0000000, 0b00001, 0b00000, 0b00000) => Box::new(EBREAK{}),
                (0b000, 0b0000000, 0b00010, 0b00000, 0b00000) => Box::new(URET{}),
                (0b000, 0b0001000, 0b00010, 0b00000, 0b00000) => Box::new(SRET{}),
                (0b000, 0b0011000, 0b00010, 0b00000, 0b00000) => Box::new(MRET{}),
                (0b000, 0b0001000, 0b00101, 0b00000, 0b00000) => Box::new(WFI{}),
                (0b000, 0b0001001, _, _, 0b00000) => Box::new(SFENCEVMA{ }),
                (0b001, _, _, _, _) => Box::new(CSRRW { csr: csr, rd: rd, rs1: rs1 }),
                (0b010, _, _, _, _) => Box::new(CSRRS { csr: csr, rd: rd, rs1: rs1 }),
                (0b011, _, _, _, _) => Box::new(CSRRC { csr: csr, rd: rd, rs1: rs1 }),
                (0b101, _, _, _, _) => Box::new(CSRRWI{ csr: csr, rd: rd, zimm: zimm }),
                (0b110, _, _, _, _) => Box::new(CSRRSI{ csr: csr, rd: rd, zimm: zimm }),
                (0b111, _, _, _, _) => Box::new(CSRRCI{ csr: csr, rd: rd, zimm: zimm }),
                _ => Box::new(UnknownOp{}),
            }
        },
        _ => Box::new(UnknownOp{}),
    }
}
