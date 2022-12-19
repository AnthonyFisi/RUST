# The Rust Programming language (Roadmap)

### 1. Hello, World!

En este primer tutorial conoci acerca de las ventajas y casos donde usar rust ademas de explorar ligeramente un primer ejemplo para conocer la sintaxis de rust.

### 2. Hello, Cargo!

El manejador de paquetes de RUST es CARGO que ayuda a crear proyectos mas complejos que implica tener dependecias y demas.

* cargo new : crear un proyecto
* cargo build : construir el proyecto
* cargo run : crear y correr el proyecto
* cargo check : construir el proyecto sin producir un binario para verfificar los errores


### 3. Programming a Guessing Game

Las nuevas herramientas que usamos es let , match y nuevos métodos , además de usar plugins externos.
 
Iniciamos con un nuevo juego para implementar lo conocido hasta el momento.
 
- Usamos USE para implementar librerías.
- Definimos dos conceptos de variables mutables y no mutables
- Las variables por defecto son inmutables y para poder convertirse en mutables debe ir agregado al costado con la palabra mut.
 
   let apple = 9; // immutable
   let mut apple = 9; // mutable
 
 
#### Crate
Crate viene a ser una colección de código como librerías para poder reutilizar en otros proyectos.
 
- Modificamos el archivo Cargo.toml para poder agregar nuevas dependencias en la sección de [dependencies].
 
- Empezamos a utilizar rand para generar números aleatorios y nos explican como es el manejo de versiones con Cargo de las dependencias.
 
#### Match
 
Utilizamos match para comparar una variable con diferentes casos.A primer vistazo puede ser con un switch en otros lenguajes de programación.


### 4.Common Programming Concepts

#### 4.1 Variables and Mutability
* Variables 

Exploramos dos variables importantes que debemos conocer

- variables inmutables (let)
- variables mutables (let mut)

* Constants

Las constantes solo son declaradas con "const" y en el nombre de las constantes deben ir en mayuscula,ademas de ir separado por un guion bajo.

Son ideales para calcular valores en tiempo de ejecucion.


* Shadowing

Nos da la posibilidad de reasignar valores como si fuera una variable mutable pero con la diferencia que sigue siendo inmutable.Ademas tiene la propiedad de que puede reusarse el  mismo nombre de la variable con diferente tipo de variable.

#### 4.2 Data types

En rust tenemos dos subtipos de datos : scalar y compound

Ejemplo:

 let name: <data_type> = value

* Scalar types

- Integer types : signed (i32) o el unsigned(u32)
- Floating-Point types : contamos con float de 32 y 64 bits y todos son signed.
- Numeric Operations : Rust soporte las operaciones matematicas basicas como + , - ,* y /.

- Boolean Type : Tienes dos posibilidades de true or false unos delos dos byte.
- Character Type : Declaramos un caracter con comillas simples y contiene 4 bytes para representar mas que el codigo ASCII.

* Compound types

- Tuple

Tiene un tamaño fijo y una vez declarada no crece ni es recortado.

Cada valor de la tuple puede ser diferente

Para poder extraer cada valor debemos declarar otra variables con la misma cantidad de datos que tiene la tuple.

La otra forma de extraer datos es acompañar con un punto al costado de la variable de la tupla y poner el indice que deseamos extraer.

- Arrays

Es usado para un tamaño fijo de elementos.

Todos los elementos deben ser del mismo tipo.

Para poder extraer los valores debemos acceder con unos corchetes y dentro el indice que deseamos extraer.

#### 4.3 Functions

Las funciones en rust son nombradas con letra minuscula y separadas con un guion abajo.

Podemos declarar la funcion en cualquier lugar eso no es relevante en rust.

Los parametros son definidos de la siguiente manera (<variable_name> :<type> )

Contamos con dos definiciones 

- Statements: Realizan alguna accion y no retornan ningun valor.

- Expressions: Es una combinacion de statements y ademas de eso retorna un valor de la operacion que se ejecuta dentro de la expressions.El elemento que va retornar no tiene que tener un punto y coma sino sera tomado como un statement lo cual producira un error.


La sintaxis de una funcion que retornar un valor es la siguiente:

```
fn function_name( variable_name : type) -> return_type {
   satements

   return value
}
```
#### 4.4 Comments

La forma mas comun de realizar un comentario con dos slash al inicio del declaracion que estemos haciendo.

// Vamos entendiendo rust :D

#### 4.5 Control Flow


- if Expressions

Permite dividir en dos ramas si la condicion es verdadera o no.

Podemos contar con if anidados.

Podemos guardar el resultado de un if en un let para usarlo en otras declaraciones.

- Repetition with Loops

Loop 

Contamos con loop que nos generar una iteracion infinita,para poder parar podemos ayudarnos de break y poder saltarnos un iteracion podemos usar continue.

Tenemos la posibilidad de usar loops anidados.

While

A diferencia con loop con while podemos hacer el mismo procedimiento pero en menos pasos y con menos declaraciones.

For

Al momento de usar arrays es mas recomendable usar For en vez de While ya que evitamos tener problema en el acceso de algun indice en el arreglo.Por otro lado al momento de iterar un array en un for automaticamente accedera a cada elemento sin presentar ningun bug.


### Ownership
 

 #### What is Ownership?

 Ownership is un conjunto de reglas que gobiernan como manejar la memoria en los programas hechos en RUST.

 Si algunas de las caracteristicas no se cumple el programa no compilira.

 Ownership Rules

 - Cada valor en RUST tiene un owner.
 - Solo un owner por cada valor.
 - Cuando el owner sale del alcance, el valor sera eliminado.


 Variable Scope

   Una variable solo es valido dentro de un determinado alcance.

The String type

   Este tipo de datos es mas complejo por lo tanto ayuda a retratar mejor el tema del ownership porque guardaremos el valor en el heap ya que no tiene un tamaño definido.


Memory and Allocation

   Variables and Data interacting with Move

   - Las variables con datos simples pueden ser reasigandos a otras variables porque internamente se produce una copia del valor.
   - Los valores simples tienen un tamaño conocido y valor fijo.
   - Estos valores son almacenados en el stack.

   - Las variables con datos complejos no tienen la misma facilidad de que los valores sean reasignados.En el caso de ellos el valor donde fue asignado al inicio al momento de hacer la copia a la otra variable lo que esta haciendo es transferirle el valor a esta segunda variable dejando sin valor al primero y con esto produciendo un error al momento de compilar.


Variables and Data Interacting with Clone

- Para poder copiar el valor de un tipo de variable compleja necesitamos usar el metodo clone y asi ese valor creara otro espacio en memoria con el mismo valor y evitaremos el error en tiempo de compilacion.

Stack-Only Data : Copy

Como en el libro mencionan para contradictorio que en un tipo de variable tengamos que declarar clone() para asiganar un valor a partir de una variable.Pero es que las vartiables simples tienes un tamaño conocido y son almacenados enteramente en un pila.

Estos son unos de los tipos que se ejecuta internamente un Copy:

- Todos los integer
- Booleanos
- Floating point
- Los caracteres typo char
- Tuples, si solo contiene un tipo uniforme

Return Values and Scope

Cada valor de una funcion es devuelto a la funcion y luego es eliminado ya que la variable dentro de la funcion el valor es eliminado.

Para poder volver a usar un valor que se pasa por parametro de una funcion es necesario que se vuelva a retornar ese mismo valor pero en diferente variable para poder seguir usandolo.

En el siguiente tema vamos a ver que en lugar de agregar una variable mas para poder mantener en uso el valor,podemos hacer el uso de las referencias que facilitan este trabajo.


References and Borrowing

Una referencia es como un puntero que tiene la direccion donde guardamos la data.Esta garantizado que se podra acceder a la informacion sin producir ningun tipo de error.

- La variable necesita ir acompañado de un & .


```
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

Mutable references

- Para poder modificar la variable necesitamos agregar un mut en la variable donde fue declarada inicialmente.

- Se puede crear varias variables donde se les puede asignar la referencia de la variable principal.

- Con respecto a la declaracion anqterior no sucede lo mismo cuano es mutable porque solo puede ser asignado a solo una nueva variable sino producir un error de que la bariable ya fue prestado anteriormente.

```
-- first borrow later used here

```

The rules of References

- Podemos contar con una referencia o varias referencias inmutables.
- Las referencias siempre deben de ser validas.
