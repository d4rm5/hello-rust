#![allow(unused)]

// Tipos de datos compuestos
// - tuple
// - array

fn main() {
    // Tupla
    let t: (bool, u32, char) = (true, 128, 't');

    // Destructure
    let (a, b, c) = t;

    // Ignoramos un valor con _
    let (_, b, _) = t;

    // Tupla vacía
    let t_empty = ();

    // Tupla anidada
    let nested = ((1.23, 'a'), (true, 1u32, 'b'), ());

    // Acceso a las tuplas t.{numero elemento}
    println!("t = {}, {}, {}", t.0, t.1, t.2);
    println!("nested {} {}", nested.0.0, nested.1.1);
}
