#![allow(unused)]

use std::i32;

fn main() {
    // Tipos escalares:
    // - tienen un solo valor
    // - sirven como piezas para tipos más complejos

    // Integers con signo
    // Forma: in
    // Rango: -(2^(n-1)) to 2^(n-1) - 1
    let i0: i8 = 0; // -128 ~ 127

    let i1: i16 = 1;
    let i2: i32 = 2;
    let i3: i64 = 3;
    let i4: i128 = 4;

    // Depende los bits de la arch del sistema
    let iarch: isize = 1; // x86_64 = i64

    // Integers sin signo
    // Forma: un
    // Rango: 0 to 2^n - 1
    let u0: u8 = 1; // 0 ~ 2^8 - 1

    let u1: u16 = 2;
    let u2: u32 = 3;
    let u3: u64 = 4;
    let u4: u128 = 5;

    // Depende los bits de la arch del sistema
    let u5: usize = 1; // x86_64 = i64

    // Floats
    let f0: f32 = 0.01;
    let f0: f64 = 0.01;

    // Booleans
    let b: bool = true;

    // Chars
    let c: char = 'c';
    let r: char = '🦀';

    // Type conversion
    let i: i32 = 1;
    let u: u32 = i as u32;
    let x: u32 = u + (i as u32);

    // Min and max
    let min_i: i32 = i32::MIN;
    let max_i: i32 = i32::MAX;

    println!("i32 min: {min_i}");
    println!("i32 max: {max_i}");

    let min_char: char = char::MIN;
    let max_char: char = char::MAX;

    println!("char min: {min_char}");
    println!("char max: {max_char}");

    // Overflow
    let mut u: u32 = u32::MAX;
    u += 1;
    println!("u32 overflow: {u}"); // genera un overflow

    // checked_add - Some(x) si no hay of | None si hay of
    let u = u32::checked_add(u32::MAX, 1);
    println!("checked_add: {:?}", u);
    // wrapping_add
    let u = u32::wrapping_add(u32::MAX, 1);
    println!("wrapping_add: {:?}", u);
}
