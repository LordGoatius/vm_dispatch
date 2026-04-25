//! In order to perform `tailcall` the way Scala does, I use [`tramp`]
use tramp::Rec;

// I am using static muts because this most directly
// translates the given code. It is not recommended for
// actual use, but this is a low-consquence project.
//
// I would rather somehow how closures close over these,
// but with manual implementation... no
// var sp: Int = 0
// var ip: Int = 0
pub static mut SP: usize = 0;
pub static mut IP: usize = 0;
const STACK_SIZE: usize = 32;
static mut STACK: &mut [f32] = &mut [0.0; STACK_SIZE];

// // An operation is a function () => Unit
// sealed abstract class Op extends Function0[Unit]
// final case class Lit(value: Double) extends Op {
//   def apply(): Int = {
//     stack(sp) = value
//     sp = sp + 1
//     ip = ip + 1
//     if ip == instructions.size then stack(sp - 1)
//     else tailcall instructions(ip)()
//   }
// }
#[derive(Clone, Copy)]
struct Lit(f32);

// If this one doesn't work I put 0 blame on the Rust maintainers.
// This is all me.
// I'm not sure how Rust's Fn trait internals work here...
// You gotta write a macro for ts to work.
impl FnOnce<()> for Lit {
    type Output = Rec<f32>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        todo!()
    }
}

impl FnMut<()> for Lit {
    extern "rust-call" fn call_mut(&mut self, _args: ()) -> Self::Output {
        todo!()
    }
}

impl Fn<()> for Lit {
    extern "rust-call" fn call(&self, _args: ()) -> Self::Output {
        unsafe {
            STACK[SP] = self.0;
            SP += 1;
            IP += 1;
            if IP == INSTRS.len() {
                rec_ret!(STACK[SP - 1])                
            } else {
                rec_call!(INSTRS[IP]())
            }
        }
    }
}

// case object Add extends Op {
//   def apply(): Int = {
//     val a = stack(sp - 1)
//     val b = stack(sp - 2)
//     stack(sp - 2) = (a + b)
//     sp = sp - 1
//     ip = ip + 1
//     if ip == instructions.size then stack(sp - 1)
//     else tailcall instructions(ip)()
//   }
// }
#[derive(Clone, Copy)]
struct Add;

// If this one doesn't work I put 0 blame on the Rust maintainers.
// This is all me.
// I'm not sure how Rust's Fn trait internals work here...
impl FnOnce<()> for Add {
    type Output = Rec<f32>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        todo!()
    }
}

impl FnMut<()> for Add {
    extern "rust-call" fn call_mut(&mut self, _args: ()) -> Self::Output {
        todo!()
    }
}

impl Fn<()> for Add {
    extern "rust-call" fn call(&self, _args: ()) -> Self::Output {
        unsafe {
            let a = STACK[SP - 1];
            let b = STACK[SP - 2];
            STACK[SP - 2] = a + b;
            SP -= 1;
            IP += 1;
            if IP == INSTRS.len() {
                rec_ret!(STACK[SP - 1])
            } else {
                rec_call!(INSTRS[IP]())
            }
        }
    }
}

// case object Sub extends Op {
//   def apply(): Int = {
//     val a = stack(sp - 1)
//     val b = stack(sp - 2)
//     stack(sp - 2) = (a - b)
//     sp = sp - 1
//     ip = ip + 1
//     if ip == instructions.size then stack(sp - 1)
//     else tailcall instructions(ip)()
//   }
// }
#[derive(Clone, Copy)]
struct Sub;

// If this one doesn't work I put 0 blame on the Rust maintainers.
// This is all me.
// I'm not sure how Rust's Fn trait internals work here...
impl FnOnce<()> for Sub {
    type Output = Rec<f32>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        todo!()
    }
}

impl FnMut<()> for Sub {
    extern "rust-call" fn call_mut(&mut self, _args: ()) -> Self::Output {
        todo!()
    }
}

impl Fn<()> for Sub {
    extern "rust-call" fn call(&self, _args: ()) -> Self::Output {
        unsafe {
            let a = STACK[SP - 1];
            let b = STACK[SP - 2];
            STACK[SP - 2] = a - b;
            SP -= 1;
            IP += 1;
            if IP == INSTRS.len() {
                rec_ret!(STACK[SP - 1])
            } else {
                rec_call!(INSTRS[IP]())
            }
        }
    }
}
// case object Mul extends Op {
//   def apply(): Int = {
//     val a = stack(sp - 1)
//     val b = stack(sp - 2)
//     stack(sp - 2) = (a * b)
//     sp = sp - 1
//     ip = ip + 1
//     if ip == instructions.size then stack(sp - 1)
//     else tailcall instructions(ip)()
//   }
// }
#[derive(Clone, Copy)]
struct Mul;

// If this one doesn't work I put 0 blame on the Rust maintainers.
// This is all me.
// I'm not sure how Rust's Fn trait internals work here...
impl FnOnce<()> for Mul {
    type Output = Rec<f32>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        todo!()
    }
}

impl FnMut<()> for Mul {
    extern "rust-call" fn call_mut(&mut self, _args: ()) -> Self::Output {
        todo!()
    }
}

impl Fn<()> for Mul {
    extern "rust-call" fn call(&self, _args: ()) -> Self::Output {
        unsafe {
            let a = STACK[SP - 1];
            let b = STACK[SP - 2];
            STACK[SP - 2] = a * b;
            SP -= 1;
            IP += 1;
            if IP == INSTRS.len() {
                rec_ret!(STACK[SP - 1])                
            } else {
                rec_call!(INSTRS[IP]())
            }
        }
    }
}
// case object Div extends Op {
//   def apply(): Int = {
//     val a = stack(sp - 1)
//     val b = stack(sp - 2)
//     stack(sp - 2) = (a / b)
//     sp = sp - 1
//     ip = ip + 1
//     if ip == instructions.size then stack(sp - 1)
//     else tailcall instructions(ip)()
//   }
// }
#[derive(Clone, Copy)]
struct Div;

// If this one doesn't work I put 0 blame on the Rust maintainers.
// This is all me.
// I'm not sure how Rust's Fn trait internals work here...
impl FnOnce<()> for Div {
    type Output = Rec<f32>;

    extern "rust-call" fn call_once(self, _args: ()) -> Self::Output {
        todo!()
    }
}

impl FnMut<()> for Div {
    extern "rust-call" fn call_mut(&mut self, _args: ()) -> Self::Output {
        todo!()
    }
}

impl Fn<()> for Div {
    extern "rust-call" fn call(&self, _args: ()) -> Self::Output {
        unsafe {
            let a = STACK[SP - 1];
            let b = STACK[SP - 2];
            STACK[SP - 2] = a * b;
            SP -= 1;
            IP += 1;
            if IP == INSTRS.len() {
                rec_ret!(STACK[SP - 1])                
            } else {
                rec_call!(INSTRS[IP]())
            }
        }
    }
}

// val instructions: Array[Op] = ???
pub static mut INSTRS: &[&dyn Fn() -> Rec<f32>] = {
    &[
    &Lit(1.0),
    &Lit(1.0),
    &Add,
    &Lit(1.0),
    &Lit(2.0),
    &Div,
    &Lit(1.0),
    &Lit(3.0),
    &Lit(2.0),
    &Mul,
    &Div,
    &Add,
    &Lit(1.0),
    &Lit(4.0),
    &Lit(3.0),
    &Mul,
    &Lit(2.0),
    &Mul,
    &Div,
    &Add,
    &Lit(1.0),
    &Lit(5.0),
    &Lit(4.0),
    &Mul,
    &Lit(3.0),
    &Mul,
    &Lit(2.0),
    &Mul,
    &Div,
    &Add,
    &Lit(1.0),
    &Lit(6.0),
    &Lit(5.0),
    &Mul,
    &Lit(4.0),
    &Mul,
    &Lit(3.0),
    &Mul,
    &Lit(2.0),
    &Mul,
    &Lit(1.0),
    &Mul,
    &Div,
    &Add,
    &Lit(1.0),
    &Lit(7.0),
    &Lit(6.0),
    &Mul,
    &Lit(5.0),
    &Mul,
    &Lit(4.0),
    &Mul,
    &Lit(3.0),
    &Mul,
    &Lit(2.0),
    &Mul,
    &Lit(1.0),
    &Mul,
    &Div,
    &Add,
    &Add ]
};
