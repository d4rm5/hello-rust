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

        let mut parts = input.trim().split_whitespace(); // Dividimos el input por espacios en blanco
        let command = parts.next().unwrap(); // Extraemos la primer parte como comando
        let args = parts; // Tratamos las siguientes partes como argumentos

        match command {
            // usamos el patrón match para codear los builints
            "cd" => {
                /* let new_dir = match args.peekable().peek() {
                    Some(valor) => *valor, // "Desreferenciamos" para acceder al texto real
                    None => "/",
                };  */
                let new_dir = args.peekable().peek().map_or("/", |x| *x);

                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            }
            "exit" => return,
            command => {
                let child = Command::new(command).args(args).spawn().unwrap(); // Transformamos el comando en mutable

                match child {
                    Ok(mut child) => child.wait(),
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
    }
}
