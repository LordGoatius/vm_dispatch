use bitstruct::{bitstruct, FromRaw};

// TODO: Implement baseline register machine using match

pub mod direct;

pub struct Machine {
    regs: [u16; 16],
    ip: usize,
    instrs: Vec<u16>,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Op {
    Add =  0b0000,
    Sub =  0b0001,
    Mul =  0b0010,
    Div =  0b0011,
    Bgt =  0b0100,
    Bleq = 0b0101,
}

impl FromRaw<u8, Op> for Instr {
    fn from_raw(raw: u8) -> Op {
        use Op::*;
        match raw {
            0b0000 => Add,
            0b0001 => Sub,
            0b0010 => Mul,
            0b0011 => Div,
            0b0100 => Bgt,
            0b0101 => Bleq,
            _ => panic!("Unknown Op")
        }
        
    }
}

impl From<Op> for u8 {
    fn from(value: Op) -> Self {
        value as u8
    }
}

bitstruct! {
    #[derive(Clone, Copy)]
    pub struct Instr(u16) {
        op: Op = 0..4;
        rd: u8 = 4..8;
        r1: u8 = 8..12;
        imm: u16 = 12..16;
    }
}

pub type Reg = u8;

pub fn instr(op: Op, rd: Reg, r1: Reg, imm: u16) -> Instr {
    let mask = 0xf;
    let op = op as u16 & mask;
    let rd = (rd as u16 & mask) << 4;
    let r1 = (r1 as u16 & mask) << 8;
    let imm = (imm & mask) << 12;
    Instr(op | rd | r1 | imm)
}

#[cfg(test)]
pub mod tests {
    use crate::machine::{Op, instr};

    #[test]
    fn inst_test() {
        let op = Op::Add;
        let rd = 0xf;
        let r1 = 0x5;
        let imm = 0xf;
        let inst = instr(op, rd, r1, imm);

        assert_eq!(inst.op(), op);
        assert_eq!(inst.rd(), rd);
        assert_eq!(inst.r1(), r1);
        assert_eq!(inst.imm(), imm);
    }
}
