# Ownership 

Es el mecanismo por el cual se gestiona la memoria sin necesidad de un *garbage colelctor*.  Asegura la seguridad de la memoria y previene errores comunes como punteros colgantes y condiciones de carrera de datos. 

Contiene tres reglas fundamentales:

- Cada valor tiene un único propietario: Cada dato en Rust tiene una variable que es su único dueño.
- Solo un propietario a la vez: No pueden haber dos variables conteniendo el mismo dato. Al reasignar un dato a otra variable el valor se mueve y la anterior variable es descartada.
- Limpieza al salir del *scope*: Cuando la variable propietaria sale de su scope, el valor se descarta automáticamente y la memoria se libera.

## Move y copy

Para tipos complejos almacenados en el Heap, la asignación transfiere la propiedad. Si `s1` es dueño de un string y hacemos:

```rust
let s2 = s1;
```

`s1` se invalida y no puede usarse mas.

Para tipos primitivos almacenados en el Stack, la asignación implementa el rasgo `Copy`. Al asignarlos, se duplican los bits y ambas variables siguen siendo válidas:

```rust
let x = 5;
let y = x; // x NO se invalida, se copia.
println!("x: {}, y: {}", x, y); // Esto es válido.
```
