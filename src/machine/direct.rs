use crate::machine::{Instr, Machine, Op};

pub struct DirectInstr<'a>(Instr, &'a mut Machine);

pub trait Opr: FnMut() {}

impl FnOnce<()> for DirectInstr<'_> {
    type Output = ();

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        todo!()
    }
}

impl FnMut<()> for DirectInstr<'_> {
    extern "rust-call" fn call_mut(&mut self, _args: ()) -> Self::Output {
        let instr = self.0;
        let op  = instr.op();
        let rd  = instr.rd() as usize;
        let r1  = instr.r1() as usize;
        let imm = instr.imm();
        // Correct me if I'm wrong but doesn't this defeat the whole point
        match op {
            Op::Add  => {
                let regs = &mut self.1.regs; 
                regs[rd] = regs[rd] + (regs[r1] + imm);
            },
            Op::Sub  => {
                let regs = &mut self.1.regs; 
                regs[rd] = regs[rd] - (regs[r1] + imm);
            },
            Op::Mul  => {
                let regs = &mut self.1.regs; 
                regs[rd] = regs[rd] * (regs[r1] + imm);
            },
            Op::Div  => {
                let regs = &mut self.1.regs; 
                regs[rd] = regs[rd] / (regs[r1] + imm);
            },
            Op::Bgt  => todo!(),
            Op::Bleq => todo!(),
        }
        DirectInstr(Instr(self.1.instrs[self.1.ip]), self.1)()
    }
}
