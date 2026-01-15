use super::foo;

pub fn call_foo() {
    foo::print();
}

pub fn print() {
    // private by default
    f();
    println!("my")
}

fn f() {
    a::print();
    println!("f is private");
}

pub mod a;
