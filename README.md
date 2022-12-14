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

