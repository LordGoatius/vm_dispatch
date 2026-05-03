use crate::machine::{Instr, Machine, Op, Reg};

struct DirectInstr<'a>(Instr, &'a mut Machine);
struct DirectOp(Op);

trait Opr: FnMut(Reg, Reg, u16, &mut Machine) {}

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

// Perhaps simply using real functions next time would suffice.
// Would be much easier.
static mut ADD:  &mut dyn Opr = &mut Add;
static mut SUB:  &mut dyn Opr = &mut Sub;
static mut MUL:  &mut dyn Opr = &mut Mul;
static mut DIV:  &mut dyn Opr = &mut Div;
static mut BGT:  &mut dyn Opr = &mut Bgt;
static mut BLEQ: &mut dyn Opr = &mut Bleq;
static mut HALT: &mut dyn Opr = &mut Halt;

pub fn begin(machine: &mut Machine) {
    let instr = machine.instrs[machine.ip];
    let op  = DirectOp(instr.op()).to_thread();
    let rd  = instr.rd();
    let r1  = instr.r1();
    let imm = instr.imm();
    op(rd, r1, imm, machine);
}

impl DirectOp {
    // This is the best way I can think of doing it
    #[inline(always)]
    fn to_thread(self) -> &'static mut &'static mut dyn Opr {
        // Oh yeah. Let's do some UB.
        // Optimization performed on the idea
        // that each mutable reference is unique is still valid I
        // believe because this is an extremely weird program?
        // Maybe? So it's UB but unlikely to cause issues I think.
        // for now. Maybe.
        unsafe {
            match self.0 {
                Op::Halt => &mut HALT,
                Op::Add  => &mut ADD,
                Op::Sub  => &mut SUB,
                Op::Mul  => &mut MUL,
                Op::Div  => &mut DIV,
                Op::Bgt  => &mut BGT,
                Op::Bleq => &mut BLEQ,
            }
        }
    }
}

impl FnOnce<()> for DirectInstr<'_> {
    type Output = ();

    #[inline(always)]
    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        let instr = self.0;
        let op  = DirectOp(instr.op()).to_thread();
        let rd  = instr.rd();
        let r1  = instr.r1();
        let imm = instr.imm();
        op(rd, r1, imm, self.1);
    }
}

impl FnOnce<(Reg, Reg, u16, &mut Machine)> for Add {
    type Output = ();

    extern "rust-call" fn call_once(self, _args: (Reg, Reg, u16, &mut Machine)) -> Self::Output {
        todo!()
    }
}

impl FnMut<(Reg, Reg, u16, &mut Machine)> for Add {
    extern "rust-call" fn call_mut(&mut self, args: (Reg, Reg, u16, &mut Machine)) -> Self::Output {
        let (rd, r1, imm, machine) = args;
        let rd = rd as usize;
        let r1 = r1 as usize;
        let r1v = machine.regs[r1];
        let rdv = machine.regs[rd];
        machine.regs[rd] = rdv + (r1v + imm);
        machine.ip += 1;
        DirectInstr(machine.instrs[machine.ip], machine)()
    }
}

impl FnOnce<(Reg, Reg, u16, &mut Machine)> for Sub {
    type Output = ();

    extern "rust-call" fn call_once(self, _args: (Reg, Reg, u16, &mut Machine)) -> Self::Output {
        todo!()
    }
}

impl FnMut<(Reg, Reg, u16, &mut Machine)> for Sub {
    extern "rust-call" fn call_mut(&mut self, args: (Reg, Reg, u16, &mut Machine)) -> Self::Output {
        let (rd, r1, imm, machine) = args;
        let rd = rd as usize;
        let r1 = r1 as usize;
        let r1v = machine.regs[r1];
        let rdv = machine.regs[rd];
        machine.regs[rd] = rdv - (r1v + imm);
        machine.ip += 1;
        DirectInstr(machine.instrs[machine.ip], machine)()
    }
}

impl FnOnce<(Reg, Reg, u16, &mut Machine)> for Mul {
    type Output = ();

    extern "rust-call" fn call_once(self, _args: (Reg, Reg, u16, &mut Machine)) -> Self::Output {
        todo!()
    }
}

impl FnMut<(Reg, Reg, u16, &mut Machine)> for Mul {
    extern "rust-call" fn call_mut(&mut self, args: (Reg, Reg, u16, &mut Machine)) -> Self::Output {
        let (rd, r1, imm, machine) = args;
        let rd = rd as usize;
        let r1 = r1 as usize;
        let r1v = machine.regs[r1];
        let rdv = machine.regs[rd];
        machine.regs[rd] = rdv * (r1v + imm);
        machine.ip += 1;
        DirectInstr(machine.instrs[machine.ip], machine)()
    }
}

impl FnOnce<(Reg, Reg, u16, &mut Machine)> for Div {
    type Output = ();

    extern "rust-call" fn call_once(self, _args: (Reg, Reg, u16, &mut Machine)) -> Self::Output {
        todo!()
    }
}

impl FnMut<(Reg, Reg, u16, &mut Machine)> for Div {
    extern "rust-call" fn call_mut(&mut self, args: (Reg, Reg, u16, &mut Machine)) -> Self::Output {
        let (rd, r1, imm, machine) = args;
        let rd = rd as usize;
        let r1 = r1 as usize;
        let r1v = machine.regs[r1];
        let rdv = machine.regs[rd];
        machine.regs[rd] = rdv / (r1v + imm);
        machine.ip += 1;
        DirectInstr(machine.instrs[machine.ip], machine)()
    }
}

impl FnOnce<(Reg, Reg, u16, &mut Machine)> for Bgt {
    type Output = ();

    extern "rust-call" fn call_once(self, _args: (Reg, Reg, u16, &mut Machine)) -> Self::Output {
        todo!()
    }
}

impl FnMut<(Reg, Reg, u16, &mut Machine)> for Bgt {
    extern "rust-call" fn call_mut(&mut self, args: (Reg, Reg, u16, &mut Machine)) -> Self::Output {
        let (rd, r1, imm, machine) = args;
        let rd = rd as usize;
        let r1 = r1 as usize;
        let r1v = machine.regs[r1];
        let rdv = machine.regs[rd];
        if rdv > r1v {
            machine.ip = imm as usize;
        } else {
            machine.ip += 1;
        }
        DirectInstr(machine.instrs[machine.ip], machine)()
    }
}

impl FnOnce<(Reg, Reg, u16, &mut Machine)> for Bleq {
    type Output = ();

    extern "rust-call" fn call_once(self, _args: (Reg, Reg, u16, &mut Machine)) -> Self::Output {
        todo!()
    }
}

impl FnMut<(Reg, Reg, u16, &mut Machine)> for Bleq {
    extern "rust-call" fn call_mut(&mut self, args: (Reg, Reg, u16, &mut Machine)) -> Self::Output {
        let (rd, r1, imm, machine) = args;
        let rd = rd as usize;
        let r1 = r1 as usize;
        let r1v = machine.regs[r1];
        let rdv = machine.regs[rd];
        if rdv <= r1v {
            machine.ip = imm as usize;
        } else {
            machine.ip += 1;
        }
        DirectInstr(machine.instrs[machine.ip], machine)()
    }
}

impl FnOnce<(Reg, Reg, u16, &mut Machine)> for Halt {
    type Output = ();

    extern "rust-call" fn call_once(self, _args: (Reg, Reg, u16, &mut Machine)) -> Self::Output {
        todo!()
    }
}

impl FnMut<(Reg, Reg, u16, &mut Machine)> for Halt {
    extern "rust-call" fn call_mut(&mut self, _args: (Reg, Reg, u16, &mut Machine)) -> Self::Output {
        return;
    }
}

// I really don't know if this is what's best? Or directly even
// translates to direct threading. This may be more indirect.
// NOTE: Save for indirect threading.
// impl FnOnce<(Reg, Reg, u16, &mut Machine)> for DirectOp {
//     type Output = ();

//     extern "rust-call" fn call_once(self, args: (Reg, Reg, u16, &mut Machine)) -> Self::Output {
//         let (rd, r1, imm, machine) = args;
//         let rd = rd as usize;
//         let r1 = r1 as usize;
//         let r1v = machine.regs[r1];
//         let rdv = machine.regs[rd];
//         let ip = match self.0 {
//             Op::Add => {
//                 machine.regs[rd] = rdv + (r1v + imm);
//                 machine.ip + 1
//             },
//             Op::Sub => {
//                 machine.regs[rd] = rdv - (r1v + imm);
//                 machine.ip + 1
//             },
//             Op::Mul => {
//                 machine.regs[rd] = rdv * (r1v + imm);
//                 machine.ip + 1
//             },
//             Op::Div => {
//                 machine.regs[rd] = rdv / (r1v + imm);
//                 machine.ip + 1
//             },
//             Op::Bgt => {
//                 if rdv > r1v {
//                     imm as usize
//                 } else {
//                     machine.ip + 1
//                 }
//             },
//             Op::Bleq => {
//                 if rdv <= r1v {
//                     imm as usize
//                 } else {
//                     machine.ip + 1
//                 }
//             },
//             Op::Halt => todo!(),
//         };
//     }
// }
//
// impl FnMut<(Reg, Reg, u16, &mut Machine)> for DirectOp {
//     extern "rust-call" fn call_mut(&mut self, args: (Reg, Reg, u16, &mut Machine)) -> Self::Output {
//         todo!()
//     }
// }
