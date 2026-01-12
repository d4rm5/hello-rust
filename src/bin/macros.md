# Macros (Rust)

Un macro en Rust es un atajo que se expande antes de compilar. Su principal diferencia con una función es que permite usar un número variable de argumentos. El ejemplo que más usé hasta ahora es `println!()`:

```rust
// macro
println!("El valor es {}", 42);

// lo que genera (simplificado)
{
    use std::io::Write;
    std::io::stdout().write_fmt(format_args!("El valor es {}\n", 42)).unwrap();
}
```
