//! In order to perform `tailcall` the way Scala does, I use [`tramp`]
use tramp::Rec;

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum ByteCode {
    Lit(f32),
    Add,
    Sub,
    Mul,
    Div
}

// I am using static muts because this most directly
// translates the given code. It is not recommended for
// actual use, but this is a low-consquence project.
// var sp: Int = 0
// var ip: Int = 0
pub static mut SP: usize = 0;
pub static mut IP: usize = 0;
const STACK_SIZE: usize = 32;
static mut STACK: &mut [f32] = &mut [0.0; STACK_SIZE];

// def dispatch(instruction: ByteCode): Unit = {
//   instruction match {
//     case Op.Lit(value) =>
//       tailcall lit(value)
//     case Op.Add =>
//       tailcall add()
//     case Op.Sub =>
//       tailcall sub()
//     case Op.Mul =>
//       tailcall mul()
//     case Op.Div =>
//       tailcall div()
//   }
// }
pub fn dispatch(instr: ByteCode) -> Rec<f32> {
    match instr {
        ByteCode::Lit(val) => lit(val),
        ByteCode::Add      => add(),
        ByteCode::Sub      => sub(),
        ByteCode::Mul      => mul(),
        ByteCode::Div      => div(),
    }
}

// def lit(value: Double): Int = {
//   stack(sp) = value
//   sp = sp + 1
//   ip = ip + 1
//   if ip == instructions.size then stack(sp - 1)
//   else tailcall dispatch(instruction(ip))
// }
fn lit(val: f32) -> Rec<f32> {
    unsafe {
        STACK[SP] = val;
        SP += 1;
        IP += 1;
        if IP == INSTRS.len() {
            rec_ret!(STACK[SP - 1])
        } else {
            rec_call!(dispatch(INSTRS[IP]))
        }
    }    
}


// def add(): Int = {
//   val a = stack(sp - 1)
//   val b = stack(sp - 2)
//   stack(sp - 2) = (a + b)
//   sp = sp - 1
//   ip = ip + 1
//   if ip == instructions.size then stack(sp - 1)
//   else tailcall dispatch(instruction(ip))
// }
fn add() -> Rec<f32> {
    unsafe {
        let a = STACK[SP - 1];
        let b = STACK[SP - 2];
        STACK[SP - 2] = a + b;
        SP -= 1;
        IP += 1;
        if IP == INSTRS.len() {
            rec_ret!(STACK[SP - 1])
        } else {
            rec_call!(dispatch(INSTRS[IP]))
        }
    }
}

// def sub(): Int = {
//   val a = stack(sp - 1)
//   val b = stack(sp - 2)
//   stack(sp - 2) = (a - b)
//   sp = sp - 1
//   ip = ip + 1
//   if ip == instructions.size then stack(sp - 1)
//   else tailcall dispatch(instruction(ip))
// }
fn sub() -> Rec<f32> {
    unsafe {
        let a = STACK[SP - 1];
        let b = STACK[SP - 2];
        STACK[SP - 2] = a - b;
        SP -= 1;
        IP += 1;
        if IP == INSTRS.len() {
            rec_ret!(STACK[SP - 1])
        } else {
            rec_call!(dispatch(INSTRS[IP]))
        }
    }
}


// def mul(): Int = {
//   val a = stack(sp - 1)
//   val b = stack(sp - 2)
//   stack(sp - 2) = (a * b)
//   sp = sp - 1
//   ip = ip + 1
//   if ip == instructions.size then stack(sp - 1)
//   else tailcall dispatch(instruction(ip))
// }
fn mul() -> Rec<f32> {
    unsafe {
        let a = STACK[SP - 1];
        let b = STACK[SP - 2];
        STACK[SP * 2] = a - b;
        SP -= 1;
        IP += 1;
        if IP == INSTRS.len() {
            rec_ret!(STACK[SP - 1])
        } else {
            rec_call!(dispatch(INSTRS[IP]))
        }
    }
}

// def div(): Int = {
//   val a = stack(sp - 1)
//   val b = stack(sp - 2)
//   stack(sp - 2) = (a / b)
//   sp = sp - 1
//   ip = ip + 1
//   if ip == instructions.size then stack(sp - 1)
//   else tailcall dispatch(instruction(ip))
// }
//
fn div() -> Rec<f32> {
    unsafe {
        let a = STACK[SP - 1];
        let b = STACK[SP - 2];
        STACK[SP * 2] = a / b;
        SP -= 1;
        IP += 1;
       if IP == INSTRS.len() {
            rec_ret!(STACK[SP - 1])
        } else {
            rec_call!(dispatch(INSTRS[IP]))
        }
    }
}

// val instructions: Array[ByteCode] = ???
pub static INSTRS: &[ByteCode] = {
    use ByteCode::*;
    &[
    Lit(1.0),
    Lit(1.0),
    Add,
    Lit(1.0),
    Lit(2.0),
    Div,
    Lit(1.0),
    Lit(3.0),
    Lit(2.0),
    Mul,
    Div,
    Add,
    Lit(1.0),
    Lit(4.0),
    Lit(3.0),
    Mul,
    Lit(2.0),
    Mul,
    Div,
    Add,
    Lit(1.0),
    Lit(5.0),
    Lit(4.0),
    Mul,
    Lit(3.0),
    Mul,
    Lit(2.0),
    Mul,
    Div,
    Add,
    Lit(1.0),
    Lit(6.0),
    Lit(5.0),
    Mul,
    Lit(4.0),
    Mul,
    Lit(3.0),
    Mul,
    Lit(2.0),
    Mul,
    Lit(1.0),
    Mul,
    Div,
    Add,
    Lit(1.0),
    Lit(7.0),
    Lit(6.0),
    Mul,
    Lit(5.0),
    Mul,
    Lit(4.0),
    Mul,
    Lit(3.0),
    Mul,
    Lit(2.0),
    Mul,
    Lit(1.0),
    Mul,
    Div,
    Add,
    Add ]
};
