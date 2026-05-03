use bitstruct::{bitstruct, FromRaw};

// TODO: Implement baseline register machine using match

pub mod direct;
pub mod direct_better;
pub mod indirect;
pub mod switch;

pub struct Machine {
    regs: [u16; 16],
    ip: usize,
    instrs: [Instr; 256],
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            regs: [0; 16],
            ip: 0,
            instrs: INSTRS,
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Op {
    Halt  = 0b0000,
    Add   = 0b0001,
    Sub   = 0b0010,
    Mul   = 0b0011,
    Div   = 0b0100,
    Bgt   = 0b0101,
    Bleq  = 0b0110,
}

impl FromRaw<u8, Op> for Instr {
    fn from_raw(raw: u8) -> Op {
        use Op::*;
        match raw {
            0b0000 => Halt,
            0b0001 => Add ,
            0b0010 => Sub ,
            0b0011 => Mul ,
            0b0100 => Div ,
            0b0101 => Bgt ,
            0b0110 => Bleq,
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
    #[derive(Clone, Copy, Default)]
    pub struct Instr(u16) {
        op: Op = 0..4;
        rd: u8 = 4..8;
        r1: u8 = 8..12;
        imm: u16 = 12..16;
    }
}

pub type Reg = u8;

pub const fn instr(op: Op, rd: Reg, r1: Reg, imm: u16) -> Instr {
    let mask = 0xf;
    let op = op as u16 & mask;
    let rd = (rd as u16 & mask) << 4;
    let r1 = (r1 as u16 & mask) << 8;
    let imm = (imm & mask) << 12;
    Instr(op | rd | r1 | imm)
}

const INSTRS: [Instr; 256] = {
    let mut instrs = [instr(Op::Halt, 0, 0, 0); 256];
    let n = 6;
    // 0 add r1, r1, 1
    // 1 add r2, r2, n
    // 2 add r3, r3, 1
    // 3 add r4, r4, 0
    //   loop:
    // 4   bleq r2, r1, hlt
    // 5   mul r3, r2, 0
    // 6   sub r2, r1, 0
    // 7   bleq r4, r4, loop
    //   loop_15:
    // 8   add r5, r5, 15
    //   cont:
    // 9   bleq r5, r4, hlt
    // 10  sub r5, r4, 1
    // 11  bleq r4, r4, cont
    //   hlt:
    // 12   hlt
    instrs[0] = instr(Op::Add, 1, 1, 1);
    instrs[1] = instr(Op::Add, 2, 2, n);
    instrs[2] = instr(Op::Add, 3, 3, 1);
    instrs[3] = instr(Op::Add, 4, 4, 0);
    instrs[4] = instr(Op::Bleq, 2, 1, 8);
    instrs[5] = instr(Op::Mul, 3, 2, 0);
    instrs[6] = instr(Op::Sub, 2, 1, 0);
    instrs[7] = instr(Op::Bleq, 4, 4, 4);

    instrs[8] = instr(Op::Add, 5, 5, 15);
    instrs[9] = instr(Op::Bleq, 5, 4, 12);
    instrs[10] = instr(Op::Sub, 5, 1, 0);
    instrs[11] = instr(Op::Bleq, 4, 4, 9);
    instrs[12] = instr(Op::Add, 1, 2, 3);
    instrs[13] = instr(Op::Add, 1, 2, 3);
    instrs[14] = instr(Op::Add, 1, 2, 3);
    instrs[15] = instr(Op::Add, 1, 2, 3);
    instrs[16] = instr(Op::Add, 1, 2, 3);
    instrs[17] = instr(Op::Add, 1, 2, 3);
    instrs[18] = instr(Op::Add, 1, 2, 3);
    instrs[19] = instr(Op::Halt, 0, 0, 0);

    instrs
};

#[cfg(test)]
pub mod tests {
    use crate::machine::{Machine, direct, direct_better, switch};
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

    #[test]
    fn direct_machine() {
        let mut machine = Machine::default();
        direct::begin(&mut machine);
        assert_eq!(machine.regs[3], 720);
    }

    #[test]
    fn direct_machine_2() {
        let mut machine = Machine::default();
        direct_better::run(&mut machine);
        assert_eq!(machine.regs[3], 720);
        panic!()
    }

    #[test]
    fn switch_machine() {
        let mut machine = Machine::default();
        switch::begin(&mut machine);
        assert_eq!(machine.regs[3], 720);
    }
}

#[cfg(test)]
pub mod bench {
    extern crate test;
    use test::Bencher;

    use crate::machine::Machine;
    
    #[bench]
    fn direct_machine(b: &mut Bencher) {
        for _ in 0..20 {
            let mut machine = Machine::default();
            super::direct::begin(&mut machine)
        }
        b.iter(|| {
            let mut machine = Machine::default();
            super::direct::begin(&mut machine)
        });
    }

    #[bench]
    fn direct_machine_2(b: &mut Bencher) {
        for _ in 0..20 {
            let mut machine = Machine::default();
            super::direct_better::run(&mut machine)
        }
        b.iter(|| {
            let mut machine = Machine::default();
            super::direct_better::run(&mut machine)
        });
    }

    #[bench]
    fn switch_machine(b: &mut Bencher) {
        for _ in 0..20 {
            let mut machine = Machine::default();
            super::switch::begin(&mut machine)
        }
        b.iter(|| {
            let mut machine = Machine::default();
            super::switch::begin(&mut machine)
        });
    }
}

