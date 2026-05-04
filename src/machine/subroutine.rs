use crate::machine::{Instr, Machine, Op};

pub fn dispatch(machine: &mut Machine) {
    let instr = machine.instrs[machine.ip];
    if matches!(instr.op(), Op::Halt) {
        return;
    }
    machine.ip = HANDLERS[instr.op() as usize](machine, instr);
    become dispatch(machine);
}

pub fn run(machine: &mut Machine) {
    dispatch(machine);
}

type Handler = fn(&mut Machine, instr: Instr) -> usize;

static HANDLERS: [Handler; 7] = [
    halt, // Halt  = 0b0000,
    add,  // Add   = 0b0001,
    sub,  // Sub   = 0b0010,
    mul,  // Mul   = 0b0011,
    div,  // Div   = 0b0100,
    bgt,  // Bgt   = 0b0101,
    blq,  // Bleq  = 0b0110,
];

fn add(machine: &mut Machine, instr: Instr) -> usize {
    let rd  = instr.rd() as usize;
    let r1  = instr.r1() as usize;
    let imm = instr.imm();
    let r1v = machine.regs[r1];
    let rdv = machine.regs[rd];
    machine.regs[rd] = rdv.wrapping_add(r1v.wrapping_add(imm));
    machine.ip + 1
}

fn sub(machine: &mut Machine, instr: Instr) -> usize {
    let rd  = instr.rd() as usize;
    let r1  = instr.r1() as usize;
    let imm = instr.imm();
    let r1v = machine.regs[r1];
    let rdv = machine.regs[rd];
    machine.regs[rd] = rdv.wrapping_sub(r1v.wrapping_add(imm));
    machine.ip + 1
}

fn mul(machine: &mut Machine, instr: Instr) -> usize {
    let rd  = instr.rd() as usize;
    let r1  = instr.r1() as usize;
    let imm = instr.imm();
    let r1v = machine.regs[r1];
    let rdv = machine.regs[rd];
    machine.regs[rd] = rdv.wrapping_mul(r1v.wrapping_add(imm));
    machine.ip + 1
}

fn div(machine: &mut Machine, instr: Instr) -> usize {
    let rd  = instr.rd() as usize;
    let r1  = instr.r1() as usize;
    let imm = instr.imm();
    let r1v = machine.regs[r1];
    let rdv = machine.regs[rd];
    machine.regs[rd] = rdv.wrapping_div(r1v.wrapping_add(imm));
    machine.ip + 1
}

fn bgt(machine: &mut Machine, instr: Instr) -> usize {
    let rd  = instr.rd() as usize;
    let r1  = instr.r1() as usize;
    let imm = instr.imm();
    let r1v = machine.regs[r1];
    let rdv = machine.regs[rd];
    if rdv > r1v {
        imm as usize
    } else {
        machine.ip + 1
    }
    
}

fn blq(machine: &mut Machine, instr: Instr) -> usize {
    let rd  = instr.rd() as usize;
    let r1  = instr.r1() as usize;
    let imm = instr.imm();
    let r1v = machine.regs[r1];
    let rdv = machine.regs[rd];
    if rdv <= r1v {
        imm as usize
    } else {
        machine.ip + 1
    }
    
}

fn halt(_machine: &mut Machine, _instr: Instr) -> usize {
    return 0;
}


