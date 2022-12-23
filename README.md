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



### 5.Using Structs to Struture Related Data

#### 5.1 Defining and Instantaining Structs

La estructura es similar a la tupla pero con la clara diferencia que cada valor tiene un significado.

Las estrcuturas son mas flexibles porque no tienes que preocuparte el orden en el que tienes que agregar la informacion.


Ejemplo : 

```
struct name {
   variable1 : type,
   variable2 : type,
            .
            .
            .
   variableN : type,
}
```

Creating instances from  other instances with struct update syntax

- En rust permite usar una instancia previamente creada y la informacion que contiene volver a reusarlo en la nueva instancia,modificando solo los datos que se requiera.

- Necesitamos agregar dos puntos para poder agregar la instacia anterior.

```
let user2 = User {
        email: String::from("another@example.com"),
        ..user1
        
};

```

Using Tuple Structs without Named Fields to Create Different Types


- Las estructuras similares a las tuplas no llevan un nombre asociado con cada valor.Son necesarias cuando necesitas nombrar una tupla y hacerla diferente de otra.

- Podemos acceder a cada valor a traves de desestructurar cada valor en cada variable o tambien de cada valor pueda ser accedido por un punto y su respectivo indice.



#### 5.2 An Example Program Using Structs

Creamos un proyecto llamado Rectangulos que va calcular el area de un rectantulo.

- Iniciamos usan una funcion con dos parametros para pasar el ancho y alto pero no es la forma mas eficiente.

- Factorizamos las dos variables y ahora usamos tuples para almacenar las variables de ancho y alto pero la forma de acceder no es tan descriptiva porque usamos los indices lo cual visualmente no significa mucho.

- Por ultimo creamos una estructura con las variables de ancho y alto,con esta estructura lista instanciamos y pasamos por parametro a la funcion para luego extraer los datos a traves de un punto y el nombre del atributo.Esta ultima forma es mas legible y elegante.

Adding Useful Functionality with Derived Traits

- Para poder imprimir la instancia de un estructura tenemos que declarar con un termino en especial.

- Previamente en la parte superior debemos agregar esta linea codigo

   #[derive(Debug)]

   ```
   println!("{:?}", struct_name);

   ```


#### Method Syntax

Los metodos son similares a las funciones ya que son declarados con fn y contienen parametros y retornan valores.
A diferencia de las funciones estan ligados al contexto de la estructura y siempre llevan un self como primer parametro.

Para poder acceder a los metodos antes en la declaracion de cada metodo debemos de agregar la referencia a la primera variable & a self para evitar problemas en la compilacion.

Methods with More Parameters

- Podemos agregar mas parametros a cada metodo luego del parametro &self, teniendo en cuenta que cada uno de ellos siendo referenciado. ya que solo vamos a leer esos valores.


Multiple impl Blocks

- Podemos separar cada metodo en una diferente implementacion y no afectaria en el uso de cada metodo en las instancias.


### 6. Enums and Pattern Matching

#### 6.1 Defining an Enum

Los enums son ideales para agrupar un conjunto de valores y de ellos escoger solo uno.

Ejemplo

```
 enum name {
   variable1 ,
   variable2 ,
         .
         .
         .
   variableN ,         
 }
```

- El ejemplo de tener la posibilidad de tener un enum y agrupar los conjuntos de formas como circulo,triangulo y cuadrado.



- Los enums nos dan la posibilidad de almacenar datos en cada variable declarada dentro del enum.

```
variable(<type1>,<type2>,<type3>, .... ,<typeN>)
```

-Para poder extraer un valor de enum tenemos que declarar de la siguiente forma.


```
let x = name_enum::variable(<type>) 
```

- Existen algunas estructuras que pueden ser reemplazados por un enum y tener mayor flexibilidad que la estrcutura.


The Option Enum and Its Advantages Over Null Values

```
enum Option<T> {
    None,
    Some(T),
}

```

- En el lenguaje rust no existe los valores nulos por lo tanto tiene como alternativa usar los Option<T> como nulos.

- Tenemos a None que es declarado si alguna variable no cuenta con un valor quiere inicializarse como nulo y en el transcurso de la ejecucion recibir un valor.

Ejm: 
Tenemos un valor entero que sera asociado a algun valor mas adelante.

```

let absent_number : Option<i32> = None ;

```

#### 6.2 The match Control Flow Construct

- Rust tiene un flujo de control llamado match que permite comparar valores contra una serie de patrones y luego ejecuta codigo.

- El verdadero valor de match esta en las expresividad de los patrones que permite y el hecho de que los compiladores confirman todos los posibles casos que estan confirmando.


Patterns that Bind to Values

- En cada arm del match podemos crear un patron con los enum y los parametros que podemos declarar en cada uno de ellos y asi usar los vaores de los parametros para ejecutar algun sentencia necesaria dentro del bloque de codigo donde realize en match.

Matches are Exhaustive

- Los patrones de arm deberian de cubrir todas las posibilidades de lado contrario se producira un bug en la compilacion.

- Los matches en rust son muy exhaustivos


Catch-all Patterns and the _ Placeholder

- Rust tiene un patron especial guion bajo (_),esto evita que usemos los valores que vienen en las variables y asi evitar que nos genere un warn.



#### 6.3 Concise Control Flow with if let

- Tenemos la posibilidad de reemplazar el match con if let para casoso donde solo tenemos que coinicidir con un solo valor e ignorar el resto.

- Ademas ayuda evitar usar codigo innecesario para encontrar el match de un valor.



### 7. Managing Growing Projects with Packages, Crates , and Modules


#### 7.1 Packages and Crates

Crate

- Crate es el bloque de codigo mas pequeño que el compilador de rust reconoce.
- Crate viene en dos formas como binary crate o library crate.
- Los binary crate siempre incluyen una funcion main que nos genera un ejecutable.
- Los library crate no cuentan con un main function y no generan un ejecutable.Ellos estan destinados a ser compartidos con otros proyectos.

Package

- Un package es un conjunto de uno o mas crates que proveen de un conjunto de funcionalidades.
- Un package contiene un archivo Cargo.toml que describe como construir los crates. 
- Un package puede contener muchos binary crates como quiera, pero a lo mucho una library crate.


#### 7.2 Defining Modules to Control Scope Privacy

Module Cheat Sheet

- Rust nos da una serie reglas detalladas para llevar en orden nuestro codigo.

- Contamos con mod para declarar un modulo en un crate con la siguiente regla.

   * mod module_name;
- Contamos con mod tambien para crear submodulos que tienen que ir anidados en el modulo que los va usar.

- Contamos con la palabra reservada pub que nos indica si un modulo es publico.Este pub va antes del mod en caso sea publico.

- Por ultimo contamos con use que nos permite acceder a la informacion del modulo.


#### 7.3 Paths for Referring to an Item in the Module Tree

- Contamos con dos tipos de rutas (Absolute y Relative) para referenciar un intem en el modulo.

- Absolute : Permite el acceso a otros crate y comienza con el nombre de crate al inicio.

- Relative : Se usa para hacer referencia a un modulo que es declarado en crate actual.

- Absolute y Relative paths son seguidos por uno mas identificadores y separados por dos puntos seguidos.

- Los modulos que creamos en rust por defecto son privados a menos que utilizemos el pub antes del mod para hacer accesible el modulo a otros crate.

- Los enum y structs siguen las misma condiciones dentro de un modulo, por defecto son privados pero al momento de agregar el pub seran publicos.


#### 7.4 Bringing Paths into Scope with the use Keyword

- Para evitar declarar toda una ruta del item que deseamos acceder podemos reemplazar esta accion por solo declarar la ruta que queremos acceder hasta el modulo y luego en la funcion que deseamos usar item solo declarar por ejemplo una funcion a la que deseabamos ingresar.Todo esto agrega un poco mas de orden para el acceso a los item de los modulos.


```
use crate::<module_name1>::<module_name2>:: ... ::<module_nameN>;

fn function_name(){

   -- > module_function()

}
```

- Contamos con as para renombrar los modulos que cuentan con el mismo nombre y asi evitar errores.

- Podemos hacer de los paquetes externos,abrimos el archivo Cargo.toml podemos agregar el nombre de la dependencia y hacer uso de elllo.

- Tenemos la posibilidad de factorizar el uso de modulos de la siguiente forma.

```
use <module_1>::<module_2>::{diferent1 , diferent2}
```

- Los glob operator nos permite traer todos los items de ese modulo con el signo de asterisco al final de los modulos.

```
use <module_1>::<module_2>::*;

```

