use crate::machine::{DirectMachine as Machine, Instr, Instrs, Pc, Regs};

pub fn run(machine: Machine) -> (Machine, Pc) {
    let instr = machine.instrs[0];
    HANDLERS[instr.op() as usize](machine.regs, machine.instrs, 0, instr)
}

type Handler = extern "rust-preserve-none" fn(Regs, Instrs, Pc, Instr) -> (Machine, Pc);
const EXCEPTION_INDEX: usize = 7;

static HANDLERS: [Handler; 7] = [
    halt, // Halt  = 0b0000,
    add,  // Add   = 0b0001,
    sub,  // Sub   = 0b0010,
    mul,  // Mul   = 0b0011,
    div,  // Div   = 0b0100,
    bgt,  // Bgt   = 0b0101,
    blq,  // Bleq  = 0b0110,
    // exception,
];

extern "rust-preserve-none" fn add(regs: Regs, instrs: Instrs, pc: Pc, instr: Instr) -> (Machine, Pc) {
    let instr = instrs[pc as usize];
    let rd = instr.rd();
    let r1 = instr.r1();
    let imm = instr.imm();
    let rdv = regs[rd as usize];
    let r1v = regs[r1 as usize];
    
    let mut machine = Machine { regs, instrs } ;
    let pc = machine.add(pc, rd, imm, r1v, rdv);

    let instr = instrs[pc as usize];
    become HANDLERS[instr.op() as usize](machine.regs, machine.instrs, pc, instr)
}

extern "rust-preserve-none" fn sub(regs: Regs, instrs: Instrs, pc: Pc, instr: Instr) -> (Machine, Pc) {
    let instr = instrs[pc as usize];
    let rd = instr.rd();
    let r1 = instr.r1();
    let imm = instr.imm();
    let rdv = regs[rd as usize];
    let r1v = regs[r1 as usize];
    
    let mut machine = Machine { regs, instrs } ;
    let pc = machine.sub(pc, rd, imm, r1v, rdv);

    let instr = instrs[pc as usize];
    become HANDLERS[instr.op() as usize](machine.regs, machine.instrs, pc, instr)
}

extern "rust-preserve-none" fn mul(regs: Regs, instrs: Instrs, pc: Pc, instr: Instr) -> (Machine, Pc) {
    let instr = instrs[pc as usize];
    let rd = instr.rd();
    let r1 = instr.r1();
    let imm = instr.imm();
    let rdv = regs[rd as usize];
    let r1v = regs[r1 as usize];
    
    let mut machine = Machine { regs, instrs } ;
    let pc = machine.mul(pc, rd, imm, r1v, rdv);

    let instr = instrs[pc as usize];
    become HANDLERS[instr.op() as usize](machine.regs, machine.instrs, pc, instr)
}

extern "rust-preserve-none" fn div(regs: Regs, instrs: Instrs, pc: Pc, instr: Instr) -> (Machine, Pc) {
    let instr = instrs[pc as usize];
    let rd = instr.rd();
    let r1 = instr.r1();
    let imm = instr.imm();
    let rdv = regs[rd as usize];
    let r1v = regs[r1 as usize];
    
    let mut machine = Machine { regs, instrs } ;
    let pc = machine.div(pc, rd, imm, r1v, rdv);

    let instr = instrs[pc as usize];
    become HANDLERS[instr.op() as usize](machine.regs, machine.instrs, pc, instr)
}

extern "rust-preserve-none" fn bgt(regs: Regs, instrs: Instrs, pc: Pc, instr: Instr) -> (Machine, Pc) {
    let instr = instrs[pc as usize];
    let rd = instr.rd();
    let r1 = instr.r1();
    let imm = instr.imm();
    let rdv = regs[rd as usize];
    let r1v = regs[r1 as usize];
    
    let mut machine = Machine { regs, instrs } ;
    let pc = machine.bgt(pc, rd, imm, r1v, rdv);

    let instr = instrs[pc as usize];
    become HANDLERS[instr.op() as usize](machine.regs, machine.instrs, pc, instr)
}

extern "rust-preserve-none" fn blq(regs: Regs, instrs: Instrs, pc: Pc, instr: Instr) -> (Machine, Pc) {
    let instr = instrs[pc as usize];
    let rd = instr.rd();
    let r1 = instr.r1();
    let imm = instr.imm();
    let rdv = regs[rd as usize];
    let r1v = regs[r1 as usize];
    
    let mut machine = Machine { regs, instrs } ;
    let pc = machine.blq(pc, rd, imm, r1v, rdv);

    let instr = instrs[pc as usize];
    become HANDLERS[instr.op() as usize](machine.regs, machine.instrs, pc, instr)
}

extern "rust-preserve-none" fn halt(regs: Regs, instrs: Instrs, pc: Pc, _instr: Instr) -> (Machine, Pc){
    return (Machine { regs, instrs }, pc)
}

// Zero-cost if never called.
extern "rust-preserve-none" fn exception(regs: Regs, instrs: Instrs, pc: Pc, instr: Instr) -> (Machine, Pc){
    // save data
    // 

    // TODO: Exception handling.
    println!("Handling exceptions");

    // Assume we decide to not retry
    // 

    become HANDLERS[instr.op() as usize](regs, instrs, pc, instr)
}

