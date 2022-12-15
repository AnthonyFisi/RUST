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