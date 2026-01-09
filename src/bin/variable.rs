#![allow(unused)]

// las variables en rust son INMUTABLES por defecto

fn main() {
    let x: i32 = -123;
    //x += 1; esto no compila wacho.

    let mut y: i32 = 123; // mut = variable mutable
    y += 1;

    let z = -123; // Rust infiere el tipado (i32)

    // let w: () = 123; Esto le pide al compilador que muestre el tipo

    const NUM: u32 = 1; // siempre inmutables

    let x: i32 = -1; // esto se llama
    let x: bool = true; // shadowing

    let v: Vec<_> = vec![1, 2, 3]; // <_> es un placeholder para un tipo genérico
}
