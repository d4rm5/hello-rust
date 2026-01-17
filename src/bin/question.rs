#[allow(unused)]

fn f1() -> Result<u32, String> {
    println!("f1");
    Ok(1)
}

fn main() {
    let res = f1();
    match res {
        Ok(x) => println!("x: {x}"),
        Err(err) => println!("Err: {err}"),
    }
}
