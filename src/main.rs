use vm_dispatch::machine::{DirectMachine, direct_full};

fn main() {
    let machine = DirectMachine::default();
    println!("{:?}", direct_full::run(machine));
}
