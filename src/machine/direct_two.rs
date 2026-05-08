use crate::machine::{Instr, Machine};

pub fn run(machine: &mut Machine) {
    let instr = machine.instrs[0];
    HANDLERS[instr.op() as usize](machine, instr)
}

type Handler = extern "rust-preserve-none" fn(&mut Machine, instr: Instr);
const EXCEPTION_INDEX: usize = 7;

static HANDLERS: [Handler; 8] = [
    halt, // Halt  = 0b0000,
    add,  // Add   = 0b0001,
    sub,  // Sub   = 0b0010,
    mul,  // Mul   = 0b0011,
    div,  // Div   = 0b0100,
    bgt,  // Bgt   = 0b0101,
    blq,  // Bleq  = 0b0110,
    exception,
];

extern "rust-preserve-none" fn add(machine: &mut Machine, instr: Instr) {
    let rd  = instr.rd() as usize;
    let r1  = instr.r1() as usize;
    let imm = instr.imm();
    let r1v = machine.regs[r1];
    let rdv = machine.regs[rd];
    machine.regs[rd] = rdv.wrapping_add(r1v.wrapping_add(imm));
    machine.ip += 1;

    let instr = machine.instrs[machine.ip];
    become HANDLERS[instr.op() as usize](machine, instr)
}

extern "rust-preserve-none" fn sub(machine: &mut Machine, instr: Instr) {
    let rd  = instr.rd() as usize;
    let r1  = instr.r1() as usize;
    let imm = instr.imm();
    let r1v = machine.regs[r1];
    let rdv = machine.regs[rd];
    machine.regs[rd] = rdv.wrapping_sub(r1v.wrapping_add(imm));
    machine.ip += 1;

    let instr = machine.instrs[machine.ip];
    become HANDLERS[instr.op() as usize](machine, instr)
}

extern "rust-preserve-none" fn mul(machine: &mut Machine, instr: Instr) {
    let rd  = instr.rd() as usize;
    let r1  = instr.r1() as usize;
    let imm = instr.imm();
    let r1v = machine.regs[r1];
    let rdv = machine.regs[rd];
    machine.regs[rd] = rdv.wrapping_mul(r1v.wrapping_add(imm));
    machine.ip += 1;

    let instr = machine.instrs[machine.ip];
    become HANDLERS[instr.op() as usize](machine, instr)
}

extern "rust-preserve-none" fn div(machine: &mut Machine, instr: Instr) {
    let rd  = instr.rd() as usize;
    let r1  = instr.r1() as usize;
    let imm = instr.imm();
    let r1v = machine.regs[r1];
    let rdv = machine.regs[rd];

    let val = rdv
        .checked_div_euclid(r1v.wrapping_add(imm));

    let (instr, next) = match val {
        Some(val) => {
            machine.regs[rd] = val;
            machine.ip += 1;
            let instr = machine.instrs[machine.ip];
            (instr, instr.op() as usize)
        },
        None => {
            (instr, EXCEPTION_INDEX)
        },
    };


    become HANDLERS[next](machine, instr)
}

extern "rust-preserve-none" fn bgt(machine: &mut Machine, instr: Instr) {
    let rd  = instr.rd() as usize;
    let r1  = instr.r1() as usize;
    let imm = instr.imm();
    let r1v = machine.regs[r1];
    let rdv = machine.regs[rd];
    if rdv > r1v {
        machine.ip = imm as usize
    } else {
        machine.ip += 1
    }

    let instr = machine.instrs[machine.ip];
    become HANDLERS[instr.op() as usize](machine, instr)
}

extern "rust-preserve-none" fn blq(machine: &mut Machine, instr: Instr) {
    let rd  = instr.rd() as usize;
    let r1  = instr.r1() as usize;
    let imm = instr.imm();
    let r1v = machine.regs[r1];
    let rdv = machine.regs[rd];
    if rdv <= r1v {
        machine.ip = imm as usize
    } else {
        machine.ip += 1
    }

    let instr = machine.instrs[machine.ip];
    become HANDLERS[instr.op() as usize](machine, instr)
}

extern "rust-preserve-none" fn halt(_machine: &mut Machine, _instr: Instr) {
    return;
}

// Zero-cost if never called.
extern "rust-preserve-none" fn exception(machine: &mut Machine, instr: Instr) {
    // save data
    let ip = machine.ip;

    // TODO: Exception handling.
    println!("Handling exceptions");

    // Assume we decide to not retry
    let instr = machine.instrs[ip + 1];

    become HANDLERS[instr.op() as usize](machine, instr)
}
