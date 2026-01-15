#[derive(Debug)]
pub struct S {
    pub id: u32, // private by default
    pub name: String,
}

// Nested module (private by default)
pub fn print() {
    println!("a");
}

use super::super::foo;

pub fn call_foo() {
    foo::print();
}
