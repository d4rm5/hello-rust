# Borrowing

El *borrowing* nos permite utilizar un valor sin tomar su propiedad. Esto se hace usando referencias, evita devolver la propiedad manualmente al terminar un bloque de código.

Existen dos tipos de préstamo meidante referencias:

1. Préstamo inmutable (`&T`): Permite leer el dato pero no modificarlo. Se pueden crear varias al mismo tiempo ya que son read-only.
2. Préstamo mutable (`&mut T`): Permite leer y modificar el dato original. Solo puede exisitir una al mismo tiempo, esto previene carreras de datos.

**Nunca pueden usarse mutables e inmutables al mismo tiempo.**
