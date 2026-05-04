use crate::machine::{Machine, Op};

pub fn run(machine: &mut Machine) {
    loop {
        let instr = machine.instrs[machine.ip];
        let rd = instr.rd() as usize;
        let r1 = instr.r1() as usize;
        let rdv = machine.regs[rd];
        let r1v = machine.regs[r1];
        let imm = instr.imm();
        machine.ip = std::hint::black_box(match instr.op() {
            Op::Halt => return,
            Op::Add => {
                machine.regs[rd] = rdv + (r1v + imm);
                machine.ip + 1
            },
            Op::Sub => {
                machine.regs[rd] = rdv - (r1v + imm);
                machine.ip + 1
            },
            Op::Mul => {
                machine.regs[rd] = rdv * (r1v + imm);
                machine.ip + 1
            },
            Op::Div => {
                machine.regs[rd] = rdv / (r1v + imm);
                machine.ip + 1
            },
            Op::Bgt => {
                if rdv > r1v {
                    imm as usize
                } else {
                    machine.ip + 1
                }
            },
            Op::Bleq => {
                if rdv <= r1v {
                    imm as usize
                } else {
                    machine.ip + 1
                }
            },
        });
    }
}
