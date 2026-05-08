use bitstruct::{bitstruct, FromRaw};

// TODO: Implement baseline register machine using match

pub mod direct;
pub mod direct_two;
pub mod direct_full;
pub mod indirect;
pub mod subroutine;
pub mod switch;

pub type Pc = u8;
pub type Regs = [u16; 16];
pub type Instrs = [Instr; 256];

pub struct Machine {
    regs: [u16; 16],
    instrs: [Instr; 256],
    ip: usize,
}

#[derive(Debug)]
pub struct DirectMachine {
    regs: [u16; 16],
    instrs: [Instr; 256],
}

impl DirectMachine {
    #[inline(always)]
    pub fn add(&mut self, pc: Pc, rd: Reg, imm: u16, r1v: u16, rdv: u16) -> Pc {
        self.regs[rd as usize] = rdv.wrapping_add(r1v.wrapping_add(imm));
        pc + 1
    }

    #[inline(always)]
    pub fn sub(&mut self, pc: Pc, rd: Reg, imm: u16, r1v: u16, rdv: u16) -> Pc {
        self.regs[rd as usize] = rdv.wrapping_sub(r1v.wrapping_add(imm));
        pc + 1
    }

    #[inline(always)]
    pub fn mul(&mut self, pc: Pc, rd: Reg, imm: u16, r1v: u16, rdv: u16) -> Pc {
        self.regs[rd as usize] = rdv.wrapping_mul(r1v.wrapping_add(imm));
        pc + 1
    }

    #[inline(always)]
    pub fn div(&mut self, pc: Pc, rd: Reg, imm: u16, r1v: u16, rdv: u16) -> Pc {
        self.regs[rd as usize] = rdv.wrapping_div(r1v.wrapping_add(imm));
        pc + 1
    }

    #[inline(always)]
    pub fn bgt(&mut self, pc: Pc, rd: Reg, imm: u16, r1v: u16, rdv: u16) -> Pc {
        if rdv > r1v {
            imm as u8
        } else {
            pc + 1
        }
    }

    #[inline(always)]
    pub fn blq(&mut self, pc: Pc, rd: Reg, imm: u16, r1v: u16, rdv: u16) -> Pc {
        if rdv <= r1v {
            imm as u8
        } else {
            pc + 1
        }
    }
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

impl Default for DirectMachine {
    fn default() -> Self {
        Self {
            regs: [0; 16],
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
    #[derive(Clone, Copy, Default, Debug)]
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
    instrs[12] = instr(Op::Halt, 0, 0, 0);

    instrs
};

#[cfg(test)]
pub mod tests {
    use std::arch::x86_64;

    use crate::machine::{
        DirectMachine, Machine, Op, direct, direct_full, direct_two, indirect, instr, subroutine, switch
    };

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
        direct_two::run(&mut machine);
        assert_eq!(machine.regs[3], 720);
    }

    #[test]
    fn direct_machine_full() {
        let machine = DirectMachine::default();
        let (machine, _) = direct_full::run(machine);
        assert_eq!(machine.regs[3], 720);
    }

    #[test]
    fn switch_machine() {
        let mut machine = Machine::default();
        switch::run(&mut machine);
        assert_eq!(machine.regs[3], 720);
    }

    #[test]
    fn indirect_machine() {
        let mut machine = Machine::default();
        indirect::run(&mut machine);
        assert_eq!(machine.regs[3], 720);
    }

    #[test]
    fn subroutine_machine() {
        let mut machine = Machine::default();
        subroutine::run(&mut machine);
        assert_eq!(machine.regs[3], 720);
    }

    #[test]
    fn readcycle() {
        let begin;
        let end;
        let mut machine = Machine::default();
        begin = unsafe { x86_64::_rdtsc() };
        super::switch::run(&mut machine);
        end = unsafe { x86_64::_rdtsc() };
        println!("{}", end-begin);

        let begin;
        let end;
        let mut machine = Machine::default();
        begin = unsafe { x86_64::_rdtsc() };
        super::direct_two::run(&mut machine);
        end = unsafe { x86_64::_rdtsc() };
        println!("{}", end-begin);
    }

    #[test]
    fn direct_machine_exception() {
        let mut machine = Machine::default();
        machine.instrs[0..2].copy_from_slice(&[instr(Op::Div, 1, 1, 0), instr(Op::Halt, 0, 0, 0)]);
        direct_two::run(&mut machine);
    }

}

#[cfg(test)]
pub mod bench {
    extern crate test;
    use test::Bencher;

    use crate::machine::{DirectMachine, Machine};
    
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
            super::direct_two::run(&mut machine)
        }
        b.iter(|| {
            let mut machine = Machine::default();
            super::direct_two::run(&mut machine)
        });
    }

    #[bench]
    fn direct_machine_full(b: &mut Bencher) {
        for _ in 0..20 {
            let machine = DirectMachine::default();
            let (machine, _) = super::direct_full::run(machine);
        }
        b.iter(|| {
            let machine = DirectMachine::default();
            let (machine, _) = super::direct_full::run(machine);
        });
    }

    #[bench]
    fn indirect_machine(b: &mut Bencher) {
        for _ in 0..20 {
            let mut machine = Machine::default();
            super::indirect::run(&mut machine)
        }
        b.iter(|| {
            let mut machine = Machine::default();
            super::indirect::run(&mut machine)
        });
    }

    #[bench]
    fn subroutine_machine(b: &mut Bencher) {
        for _ in 0..20 {
            let mut machine = Machine::default();
            super::subroutine::run(&mut machine)
        }
        b.iter(|| {
            let mut machine = Machine::default();
            super::subroutine::run(&mut machine)
        });
    }

    #[bench]
    fn switch_machine(b: &mut Bencher) {
        for _ in 0..20 {
            let mut machine = Machine::default();
            super::switch::run(&mut machine)
        }
        b.iter(|| {
            let mut machine = Machine::default();
            super::switch::run(&mut machine)
        });
    }
}

