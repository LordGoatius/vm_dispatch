// // The stack
// val stack: Array[Double] = ???
pub static mut SP: usize = 0;
const STACK_SIZE: usize = 32;
static mut STACK: &mut [f32] = &mut [0.0; STACK_SIZE];

// enum ByteCode {
//   case Lit(value: Double)
//   case Add
//   case Sub
//   case Mul
//   case Div
// }
#[repr(u32)]
#[derive(Clone, Copy)]
enum Instr {
    Lit(f32) = 0,
    Add = 2712313,
    Sub = 12398123,
    Mul = 6813198,
    Div = 91236342,
}


// def dispatch(sp: Int, ip: Int): Unit = {
//   // This simple language has no control flow, so we stop when
//   // we reach the end of the instructions
//   if ip == instructions.size then stack(sp - 1)
//   else 
//     // Fetch instruction
//     val ins = instructions(ip)
//     // Dispatch
//     ins match {
//       case Op.Lit(value) =>
//         stack(sp) = value
//         loop(sp + 1, ip + 1)
//       case Op.Add =>
//         val a = stack(sp - 1)
//         val b = stack(sp - 2)
//         stack(sp - 2) = (a + b)
//         loop(sp - 1, ip + 1)
//       case Op.Sub =>
//         val a = stack(sp - 1)
//         val b = stack(sp - 2)
//         stack(sp - 2) = (a - b)
//         loop(sp - 1, ip + 1)
//       case Op.Mul =>
//         val a = stack(sp - 1)
//         val b = stack(sp - 2)
//         stack(sp - 2) = (a * b)
//         loop(sp - 1, ip + 1)
//       case Op.Div =>
//         val a = stack(sp - 1)
//         val b = stack(sp - 2)
//         stack(sp - 2) = (a / b)
//         loop(sp - 1, ip + 1)
//     }
// }
pub fn dispatch(sp: usize, ip: usize) -> f32 {
    unsafe {
        if ip == INSTRS.len() {
            STACK[sp - 1]
        } else {
            match INSTRS[ip] {
                Instr::Lit(value) => {
                    STACK[sp] = value;
                    dispatch(sp + 1, ip + 1)
                },
                Instr::Add => {
                    let a = STACK[sp - 2];
                    let b = STACK[sp - 1];
                    STACK[sp - 2] = a + b;
                    dispatch(sp - 1, ip + 1)
                },
                Instr::Sub => {
                    let a = STACK[sp - 2];
                    let b = STACK[sp - 1];
                    STACK[sp - 2] = a - b;
                    dispatch(sp - 1, ip + 1)
                },
                Instr::Mul => {
                    let a = STACK[sp - 2];
                    let b = STACK[sp - 1];
                    STACK[sp - 2] = a * b;
                    dispatch(sp - 1, ip + 1)
                },
                Instr::Div => {
                    let a = STACK[sp - 2];
                    let b = STACK[sp - 1];
                    STACK[sp - 2] = a / b;
                    dispatch(sp - 1, ip + 1)
                },
            }
        }
    }
}

// val instructions: Array[ByteCode] = ???
static mut INSTRS: &[Instr] = {
    use Instr::*;
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

