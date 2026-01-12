#![allow(unused)]

// String and &str
fn main() {
    // String = vector of u8 (Vec<u8>) valid UTF-8
    // &str = slice of u8 (&[u8]) valid UTF-8
    // Cuando usar String vs &str
    // String -> mutable o la data debe ser owned
    // &str -> solo lectura

    // String
    let msg: String = String::from("Hello Rust");
    let len: usize = msg.len(); // View string length
    println!("msg: {msg}"); // Hello Rust
    println!("len: {len}"); // 10

    // str - string slice
    // &str
    // suele usarse por referencia (borrowed)
    // inmutable
    let s: &str = &msg[0..4];
    println!("slice: {s}"); // Hell
    println!("slice lenght: {}", s.len()); // 3

    // String literal
    // - almacenado en el binario
    // - es un slice que apunta a una parte específica del binario
    // - es inmutable porque esta hardcodeado dentro del binario
    let hello: &str = "Hello Rust";

    let multiline: &str = r#"
        {
            "a" : 1,
            "b": {"c": 0},
            "d: 2
        }
    "#;

    println!("{}", multiline);

    // Deref coercion
    // transformamos la referencia a un String
    // en la referencia a un slice :)
    let msg: String = String::from("Hello Rust");
    let slice: &str = &msg;

    // Append &str to String
    let mut msg: String = "Hello Rust".to_string();
    msg += "!";
    println!("{}", msg);

    // String interpolation
    // mezclando literals con variables

    let lang = "Rust";
    let emoji = "🦀";
    let msg = format!("Hello {lang} {emoji}");
    // format! Creates a String using interpolation of runtime expressions.
}
