#![allow(unused)]

fn main() {
    // unwrap and expect

    // Option
    let x: Option<u32> = Some(3);
    // match x {
    //   Some(val) => println!("Some: {val}"),
    //   None => println!("None"),
    // }

    let v = x.unwrap(); // si es none hace panic con mensaje genérico
    // let v = x.expect("mensaje custom") // si es none hace panic con el mensaje custom
    println!("val: {v}");

    // Se usa unwrap() principalmente por conveniencia en código temporal, ejemplos,
    // o cuando la lógica garantiza el éxito de la operación.
    // Se prefiere expect() (o un manejo de errores más robusto) en software de producción para entender
    // por qué ocurrió el fallo

    // Result
    let x = 1;
    let y = 0;
    let z: Result<u32, String> = Err("div by zero".to_string());
    // match z {
    //    Ok(val) => println!("div = {val}"),
    //    Err(err) => println!("MathError = {:?}", err),
    // }
    let v = z.unwrap();
    println!("{}", v); // panic with Err "div by zero"
}
