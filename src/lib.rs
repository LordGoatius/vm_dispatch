//! Alternative dispatch attempts in Rust, from https://noelwelsh.com/posts/understanding-vm-dispatch/
#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(trait_alias)]
#![feature(explicit_tail_calls)]
#![feature(test)]
#![allow(static_mut_refs)]
#[macro_use] extern crate tramp;

pub mod indirect_threaded;
pub mod direct_threaded;
pub mod subroutine_dispatch;
pub mod switch_dispatch;

use tramp::tramp;

fn indirect_threaded() {
    use crate::indirect_threaded::*;
    unsafe {
        IP = 0;
        SP = 0;
    }
    let value = tramp(dispatch(INSTRS[0]));
    println!("{}", value);
}

fn direct_threaded() {
    use crate::direct_threaded::*;
    unsafe {
        IP = 0;
        SP = 0;
    }
    let value = unsafe {
        tramp(INSTRS[0]())
    };
    println!("{}", value);
}

fn subroutine_dispatch() {
    use crate::subroutine_dispatch::*;
    unsafe {
        SP = 0;
    }
    let value = dispatch(0);
    println!("{}", value);
}

fn switch_dispatch() {
    use crate::switch_dispatch::*;
    unsafe {
        SP = 0;
    }
    let value = dispatch(0, 0);
    println!("{}", value);
}

#[cfg(test)]
pub mod bench {
    extern crate test;
    use test::Bencher;

    #[bench]
    fn indirect(b: &mut Bencher) {
        b.iter(crate::indirect_threaded)
    }

    #[bench]
    fn direct(b: &mut Bencher) {
        b.iter(crate::direct_threaded)
    }

    #[bench]
    fn subroutine(b: &mut Bencher) {
        b.iter(crate::subroutine_dispatch)
    }

    #[bench]
    fn switch(b: &mut Bencher) {
        b.iter(crate::switch_dispatch)
    }

}

#[cfg(test)]
pub mod test {
    #[test]
    fn indirect_test() {
        crate::indirect_threaded();
    }

    #[test]
    fn direct_test() {
        crate::direct_threaded();
    }

    #[test]
    fn subroutine_test() {
        crate::subroutine_dispatch();
    }

    #[test]
    fn switch_test() {
        crate::switch_dispatch();
    }
}
