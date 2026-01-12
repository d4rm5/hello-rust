#![allow(unused)]

// Type personalizado para representar estados finitos, o posibles estados

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
    Rgba(u8, u8, u8, f32),
    Hex(String),
    Hsl { h: u8, s: u8, l: u8 },
}

fn main() {
    // Enum
    let color: Color = Color::Red;
    let color = Color::Rgba(12, 29, 255, 0.1);
    let color = Color::Hex("#FAFAFA".to_string());
    let color = Color::Hsl { h: 0, s: 1, l: 200 };
    // Attributes - Debug and PartialEq
    // Debug
    println!("{:?}", color);
    // PartialEq - una forma de comparar enums
    println!("{}", Color::Red == Color::Green); // false
    println!("{}", Color::Red == Color::Red); // true
    // Option = Some(11) | None
    let x: Option<i32> = None;
    let x: Option<i32> = Some(11);
    // Result = Ok(10) | Err("div by 0")
    let res: Result<u32, String> = Ok(5);
    let res: Result<u32, String> = Err("div by 0".to_string());
}
