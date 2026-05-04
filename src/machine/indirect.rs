use crate::machine::{Instr, Machine, Op};

pub fn run(machine: &mut Machine) {
    let instr = machine.instrs[0];
    dispatch(machine, instr);
}

fn dispatch(machine: &mut Machine, instr: Instr) {
    match instr.op() {
        Op::Halt => become hlt(machine, instr),
        Op::Add  => become add(machine, instr),
        Op::Sub  => become sub(machine, instr),
        Op::Mul  => become mul(machine, instr),
        Op::Div  => become div(machine, instr),
        Op::Bgt  => become bgt(machine, instr),
        Op::Bleq => become blq(machine, instr),
    }    
}

fn add(machine: &mut Machine, instr: Instr) {
    let rd  = instr.rd() as usize;
    let r1  = instr.r1() as usize;
    let imm = instr.imm();
    let r1v = machine.regs[r1];
    let rdv = machine.regs[rd];
    machine.regs[rd] = rdv.wrapping_add(r1v.wrapping_add(imm));
    machine.ip += 1;

    let instr = machine.instrs[machine.ip];
    become dispatch(machine, instr);
}
fn sub(machine: &mut Machine, instr: Instr) {
    let rd  = instr.rd() as usize;
    let r1  = instr.r1() as usize;
    let imm = instr.imm();
    let r1v = machine.regs[r1];
    let rdv = machine.regs[rd];
    machine.regs[rd] = rdv.wrapping_sub(r1v.wrapping_add(imm));
    machine.ip += 1;

    let instr = machine.instrs[machine.ip];
    become dispatch(machine, instr);
    
}
fn mul(machine: &mut Machine, instr: Instr) {
    let rd  = instr.rd() as usize;
    let r1  = instr.r1() as usize;
    let imm = instr.imm();
    let r1v = machine.regs[r1];
    let rdv = machine.regs[rd];
    machine.regs[rd] = rdv.wrapping_mul(r1v.wrapping_add(imm));
    machine.ip += 1;

    let instr = machine.instrs[machine.ip];
    become dispatch(machine, instr);
    
}
fn div(machine: &mut Machine, instr: Instr) {
    let rd  = instr.rd() as usize;
    let r1  = instr.r1() as usize;
    let imm = instr.imm();
    let r1v = machine.regs[r1];
    let rdv = machine.regs[rd];
    machine.regs[rd] = rdv.wrapping_div(r1v.wrapping_add(imm));
    machine.ip += 1;

    let instr = machine.instrs[machine.ip];
    become dispatch(machine, instr);
    
}
fn bgt(machine: &mut Machine, instr: Instr) {
    let rd  = instr.rd() as usize;
    let r1  = instr.r1() as usize;
    let imm = instr.imm();
    let r1v = machine.regs[r1];
    let rdv = machine.regs[rd];
    if rdv > r1v { machine.ip = imm as usize } else { machine.ip += 1 }

    let instr = machine.instrs[machine.ip];
    become dispatch(machine, instr);
}
fn blq(machine: &mut Machine, instr: Instr) {
    let rd  = instr.rd() as usize;
    let r1  = instr.r1() as usize;
    let imm = instr.imm();
    let r1v = machine.regs[r1];
    let rdv = machine.regs[rd];
    if rdv <= r1v { machine.ip = imm as usize } else { machine.ip += 1 }

    let instr = machine.instrs[machine.ip];
    become dispatch(machine, instr);
}
fn hlt(_machine: &mut Machine, _instr: Instr) {
    return
}
