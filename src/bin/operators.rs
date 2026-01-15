#![allow(unused)]

fn main() {
    // +, -, *, /
    let a: i32 = 1;
    let b: i32 = 1;
    let c: i32 = a + b;
    let c: i32 = a - b;
    let c: i32 = a * b;
    let c: i32 = a / b; // division round down

    // % (resto != mod)
    // -1 % 2 = -1
    let a = -1;
    let b = 2;
    let c = a % b;

    // Literals
    let a = 1i32;
    let b = 3u64;
    let c = 1.23e3; // 1.23 x 10^3 = 1230
    let d = 1_000_000_000u32;

    // Boolean - and, not, or, xor
    let a = true && false;
    let a = true || false;
    let a = !true;

    // Bitwise

    // 101
    let a: u8 = 5;
    // 011
    let b: u8 = 3;

    println!("a AND b = {:03b}", a & b); // 101 AND 011
    println!("a OR b = {:03b}", a | b); // 101 OR 011
    println!("a XOR b = {:03b}", a ^ b); // 101 XOR 011
    println!("NOT a = {:03b}", !a); // 101 XOR 011
    println!("SHIFT 3 a = {:03b}", a << 3); // 101 XOR 011
}
