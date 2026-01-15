#![allow(unused)]

fn add(x: u32, y: u32) -> u32 {
    //return x + y;
    x + y
}

fn print() {
    // not return anything
    println!("not output");
}

fn forever() -> ! {
    loop {}
}

fn crash() -> ! {
    panic!("crash")
}

fn main() {
    // Function
    // Implicit return
    let x = 1;
    let y = 2;
    let z = add(x, y);
    println!("{x} + {y} = {z}");
    // No output
    print();
    // Diverge -- funciton never return
    crash();
}
