# Build Your Own Shell using Rust

## A Starting point 

- Se crea un string con `String::new()` y se usa `stdin().read_line(&mut input)` para agregarle el input de usuario.
- Usamos `.trim()` para limpiar la linea que crea `.read_line()`.
- Utilizamos `.unwrap()` para manejar errores de forma rápida. Ya que al entrar en panic imprime el error del sistema operativo.

## Accept Multiple Commands

- Utilizamos `loop` para aceptar varios comandos
- Se imprime un prompt `>` con `stdout.flush()` generado que el contenido nuevo se imprima inmediatamente sin esperar una nueva linea.
- Metemos `Command::new(command).spawn().unwrap()` en una variable `mut` para poder modificar su estado. En este caso con `child().wait()` hacemos que no acepte otro comando hasta que termine el anterior.
