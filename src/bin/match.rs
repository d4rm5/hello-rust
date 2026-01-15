#![allow(unused)]

enum Animal {
    Cat,
    Dog,
    Duck,
    Mouse,
}

fn main() {
    // if_else
    let x = 1;
    if x == 1 {
        println!("one");
    } else if x == 2 {
        println!("two");
    } else if x == 3 {
        println!("three");
    } else {
        println!("other");
    }

    // match -> similar a un switch pero requiere que se atiendan todos los casos.
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"), // catch all
    }

    // multiple cases
    match x {
        1 | 2 | 3 => println!("one or two or three"),
        _ => println!("other"),
    }
    // range
    match x {
        1..=10 => println!("between one and ten"),
        _ => println!("other"),
    }
    // @
    match x {
        i @ 1..=10 => println!("matched {i}"),
        _ => println!("other"),
    }
    // return value
    let animal = Animal::Duck;
    let animal_sound = match animal {
        Animal::Duck => "quack",
        Animal::Dog => "woof",
        Animal::Cat => "meow",
        _ => "?",
    };

    println!("Animal sound: {animal_sound}");

    // Option
    let x: Option<i32> = Some(1);
    match x {
        Some(v) => println!("Some {v}"),
        None => println!("none"),
    }
    // Result
    let res: Result<u32, String> = Ok(10);
    match res {
        Ok(val) => println!("Ok {val}"),
        Err(msg) => println!("Err: {msg}"),
    }
}
