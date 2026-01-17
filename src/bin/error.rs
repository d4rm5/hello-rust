#![allow(unused)]

#[derive(Debug)]
enum MathError {
    DivByZero,
    Other,
}

fn div(x: u32, y: u32) -> Result<u32, MathError> {
    if y == 0 {
        return Err(MathError::DivByZero);
    }
    Ok(x / y)
}

fn main() {
    // Error
    // panic!("crash");

    let arr = [1, 2, 3];
    // arr[9]; -> compiler error, out of bounds

    // Option
    let x: Option<&i32> = arr.get(9); // Option<&i32> = Some(&i32) | None
    match x {
        Some(val) => println!("val: {val}"),
        None => println!("none"),
    }

    // Result <u32, MathError>
    let x = 1;
    let y = 0;
    let z = div(x, y);

    match z {
        Ok(val) => println!("div = {val}"),
        Err(err) => println!("MathError = {:?}", err),
    }
}
