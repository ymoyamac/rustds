¿Qué se esta "consumiendo" (liberando) de la memoria según como funciona rust? ¿Se está liberando totalmente la estructura? O ¿Solo el espacio de memoria que pertenece a data?

Voy a ser preciso y no complaciente, porque aquí hay un punto fundamental de ownership y drop semantics que muchos malinterpretan.

Primero, una corrección conceptual necesaria antes de responder: Rust no “libera memoria” en el punto donde haces self.data. La liberación ocurre cuando se ejecuta Drop de los valores que salen de alcance. Esto no es una diferencia semántica menor; es central para entender qué sucede realmente.

Dicho esto, analicemos el caso concreto.

Tienes:

pub struct Node<T> {
    pub data: T,
    pub next: Link<T>
}


y el método:

pub fn data_into(self) -> T {
    self.data
}


El hecho crítico aquí es que el método consume self, es decir, toma posesión completa del Node<T>. A partir de ese momento, el llamador pierde cualquier acceso al nodo. No hay aliasing posible.

Cuando ejecutas self.data, ocurre lo siguiente, en orden lógico (no necesariamente en orden de instrucciones máquina):

Se mueve el campo data: T fuera de la estructura Node<T>.
No se copia; se transfiere ownership del valor T al llamador.

El valor Node<T> queda parcialmente movido. Conceptualmente, el campo data ya no existe dentro del Node.

Al finalizar la función, Rust intenta destruir (drop) el valor self restante.

Aquí está el punto clave que responde tu pregunta:

Rust destruye el resto de la estructura, no el campo que fue movido.

Es decir:

– T NO se libera, porque su ownership fue transferido al valor de retorno.
– Link<T> SÍ se destruye, ejecutando su Drop si lo tiene.
– La memoria ocupada por Node<T> como contenedor se libera si corresponde (por ejemplo, si el Node estaba en el heap dentro de un Box).

Formalmente, esto se describe en la Rust Reference, sección Drop elaborations y Moves out of structs:

When a place expression is moved from, that place is considered uninitialized, and its destructor is not run.

Consecuencia directa:
El destructor de Node<T> se ejecuta solo para los campos que permanecen válidos, en este caso next.

Ahora, sobre la liberación de memoria física, que es lo que normalmente se confunde:

– Si Node<T> estaba en el stack, no hay “free”; el stack simplemente se ajusta al salir del scope.
– Si estaba en el heap (Box<Node<T>>), el Box se libera cuando su Drop se ejecuta, lo cual ocurre después de mover data y destruir next.
– El campo data no se libera porque ya no pertenece a ese objeto.

Un error común es pensar que “solo se libera data” o que “la estructura queda viva sin data”. Ninguna de las dos cosas es correcta. La estructura completa es consumida, pero solo se destruyen los campos que no fueron movidos.

Si Link<T> es, por ejemplo, Option<Box<Node<T>>>, entonces este método recursivamente libera toda la lista enlazada a partir de next, mientras que devuelve el data del nodo actual. Este detalle es crucial cuando implementas listas manualmente en Rust.