#[allow(unused)]

// El operador ? es una abreviación de match:
// si el resultado de la función es Ok() se hace unwrap
// si el resultado de la función es Err() se hace un early return

fn f1() -> Result<u32, String> {
    println!("f1");
    //Ok(1)
    Err("F1 error".to_string())
}

fn main() -> Result<(), String> {
    /* let res = f1();
    match res {
        Ok(x) => println!("x: {x}"),
        Err(err) => println!("Err: {err}"),
    } */

    let x = f1()?;

    println!("x is equal to {x}");

    Ok(())
}
