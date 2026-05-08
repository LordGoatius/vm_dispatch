use crate::machine::{Instr, Machine, Op, Reg};

trait Opr: for <'a> Fn(Instr, &'a mut Machine) + Sync {}

static HANDLERS: [&dyn Opr; 7] = [
    &Halt,
    &Add,
    &Sub,
    &Mul,
    &Div,
    &Bgt,
    &Bleq,
];

struct Add;
struct Sub;
struct Mul;
struct Div;
struct Bgt;
struct Bleq;
struct Halt;

impl Opr for Add {}
impl Opr for Sub {}
impl Opr for Mul {}
impl Opr for Div {}
impl Opr for Bgt {}
impl Opr for Bleq {}
impl Opr for Halt {}

pub fn begin(machine: &mut Machine) {
    let instr = machine.instrs[0];
    let op = instr.op();
    HANDLERS[op as usize](instr, machine);
}

impl FnOnce<(Instr, &mut Machine)> for Add {
    type Output = ();

    extern "rust-call" fn call_once(self, _args: (Instr, &mut Machine)) -> Self::Output {
        todo!()
    }
}

impl FnMut<(Instr, &mut Machine)> for Add {
    extern "rust-call" fn call_mut(&mut self, args: (Instr, &mut Machine)) -> Self::Output {
        todo!()
    }
}

impl Fn<(Instr, &mut Machine)> for Add {
    extern "rust-call" fn call(&self, args: (Instr, &mut Machine)) -> Self::Output {
        let (instr, machine) = args;
        let rd = instr.rd() as usize;
        let r1 = instr.r1() as usize;
        let imm = instr.imm();
        let r1v = machine.regs[r1];
        let rdv = machine.regs[rd];
        machine.regs[rd] = rdv + (r1v + imm);
        machine.ip += 1;

        let instr = machine.instrs[machine.ip];
        HANDLERS[instr.op() as usize](instr, machine)
    }
}

impl FnOnce<(Instr, &mut Machine)> for Sub {
    type Output = ();

    extern "rust-call" fn call_once(self, _args: (Instr, &mut Machine)) -> Self::Output {
        todo!()
    }
}

impl FnMut<(Instr, &mut Machine)> for Sub {
    extern "rust-call" fn call_mut(&mut self, _args: (Instr, &mut Machine)) -> Self::Output {
        todo!()
    }
}

impl Fn<(Instr, &mut Machine)> for Sub {
    extern "rust-call" fn call(&self, args: (Instr, &mut Machine)) -> Self::Output {
        let (instr, machine) = args;
        let rd = instr.rd() as usize;
        let r1 = instr.r1() as usize;
        let imm = instr.imm();
        let r1v = machine.regs[r1];
        let rdv = machine.regs[rd];
        machine.regs[rd] = rdv - (r1v + imm);
        machine.ip += 1;

        let instr = machine.instrs[machine.ip];
        HANDLERS[instr.op() as usize](instr, machine)
    }
}

impl FnOnce<(Instr, &mut Machine)> for Mul {
    type Output = ();

    extern "rust-call" fn call_once(self, _args: (Instr, &mut Machine)) -> Self::Output {
        todo!()
    }
}

impl FnMut<(Instr, &mut Machine)> for Mul {
    extern "rust-call" fn call_mut(&mut self, _args: (Instr, &mut Machine)) -> Self::Output {
        todo!()
    }
}

impl Fn<(Instr, &mut Machine)> for Mul {
    extern "rust-call" fn call(&self, args: (Instr, &mut Machine)) -> Self::Output {
        let (instr, machine) = args;
        let rd = instr.rd() as usize;
        let r1 = instr.r1() as usize;
        let imm = instr.imm();
        let r1v = machine.regs[r1];
        let rdv = machine.regs[rd];
        machine.regs[rd] = rdv * (r1v + imm);
        machine.ip += 1;

        let instr = machine.instrs[machine.ip];
        HANDLERS[instr.op() as usize](instr, machine)
    }
}

impl FnOnce<(Instr, &mut Machine)> for Div {
    type Output = ();

    extern "rust-call" fn call_once(self, _args: (Instr, &mut Machine)) -> Self::Output {
        todo!()
    }
}

impl FnMut<(Instr, &mut Machine)> for Div {
    extern "rust-call" fn call_mut(&mut self, _args: (Instr, &mut Machine)) -> Self::Output {
        todo!()
    }
}

impl Fn<(Instr, &mut Machine)> for Div {
    extern "rust-call" fn call(&self, args: (Instr, &mut Machine)) -> Self::Output {
        let (instr, machine) = args;
        let rd = instr.rd() as usize;
        let r1 = instr.r1() as usize;
        let imm = instr.imm();
        let r1v = machine.regs[r1];
        let rdv = machine.regs[rd];
        machine.regs[rd] = rdv / (r1v + imm);
        machine.ip += 1;

        let instr = machine.instrs[machine.ip];
        HANDLERS[instr.op() as usize](instr, machine)
    }
}

impl FnOnce<(Instr, &mut Machine)> for Bgt {
    type Output = ();

    extern "rust-call" fn call_once(self, args: (Instr, &mut Machine)) -> Self::Output {
        todo!()
    }
}

impl FnMut<(Instr, &mut Machine)> for Bgt {
    extern "rust-call" fn call_mut(&mut self, args: (Instr, &mut Machine)) -> Self::Output {
        todo!()
    }
}

impl Fn<(Instr, &mut Machine)> for Bgt {
    extern "rust-call" fn call(&self, args: (Instr, &mut Machine)) -> Self::Output {
        let (instr, machine) = args;
        let rd = instr.rd() as usize;
        let r1 = instr.r1() as usize;
        let imm = instr.imm();
        let r1v = machine.regs[r1];
        let rdv = machine.regs[rd];
        if rdv > r1v {
            machine.ip = imm as usize;
        } else {
            machine.ip += 1;
        }
        let instr = machine.instrs[machine.ip];
        HANDLERS[instr.op() as usize](instr, machine)
    }
}

impl FnOnce<(Instr, &mut Machine)> for Bleq {
    type Output = ();

    extern "rust-call" fn call_once(self, args: (Instr, &mut Machine)) -> Self::Output {
        todo!()
    }
}

impl FnMut<(Instr, &mut Machine)> for Bleq {
    extern "rust-call" fn call_mut(&mut self, args: (Instr, &mut Machine)) -> Self::Output {
        todo!()
    }
}

impl Fn<(Instr, &mut Machine)> for Bleq {
    extern "rust-call" fn call(&self, args: (Instr, &mut Machine)) -> Self::Output {
        let (instr, machine) = args;
        let rd = instr.rd() as usize;
        let r1 = instr.r1() as usize;
        let imm = instr.imm();
        let r1v = machine.regs[r1];
        let rdv = machine.regs[rd];
        if rdv <= r1v {
            machine.ip = imm as usize;
        } else {
            machine.ip += 1;
        }
        let instr = machine.instrs[machine.ip];
        HANDLERS[instr.op() as usize](instr, machine)
    }
}

impl FnOnce<(Instr, &mut Machine)> for Halt {
    type Output = ();

    extern "rust-call" fn call_once(self, _args: (Instr, &mut Machine)) -> Self::Output {
        todo!()
    }
}

impl FnMut<(Instr, &mut Machine)> for Halt {
    extern "rust-call" fn call_mut(&mut self, _args: (Instr, &mut Machine)) -> Self::Output {
        todo!()
    }
}
impl Fn<(Instr, &mut Machine)> for Halt {
    extern "rust-call" fn call(&self, _args: (Instr, &mut Machine)) -> Self::Output {
        return;
    }
}
