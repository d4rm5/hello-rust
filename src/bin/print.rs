#![allow(unused)] // ignore some warnings

#[derive(Debug)]
struct Lang {
    language: String,
    version: String,
}

fn main() {
    let lang = "rust";
    println!("hello"); // print stuff inside parenthesis in a new line
    println!("hello {}", lang); // print stuff inside parenthesis in a new line
    println!("hello {lang}"); // lang varieble will be printed instead of {}

    let x = 2;
    println!("{0} x {0} = {1}", x, x * x); // 0 means first variable, 1 second and so on

    let lang = Lang {
        language: "rust".to_string(),
        version: "1.83".to_string(),
    };

    println!("{:?}", lang); //debug printing as struct
    println!("{:#?}", lang); //debug printing with line breaks
}
