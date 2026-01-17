use std::io::Write;
use std::io::stdin;
use std::io::stdout;
use std::process::Command;

fn main() {
    loop {
        print!("> "); // imprime el prompt >
        stdout().flush(); // flush de lo anterior para que esté antes de la newline

        let mut input = String::new(); // Creamos nuevo String vacío
        stdin().read_line(&mut input).unwrap(); // Lo rellenamos con el input del usuario.

        let command = input.trim(); // Elimina la linea generada por read_line

        let mut child = Command::new(command).spawn().unwrap(); // Transformamos el comando en mutable

        child.wait(); // No aceptamos otro comando hasta que finalice el primero.
    }
}
