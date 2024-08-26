fn main() {
    //OWNERSHIP: sin usar garbage collector, se encarga de liberar memoria
    // conjunto de reglas que indican como un programa maneja la memoria
    // si alguna de estas reglas se viola, el programa no compila
    /*
    la memoria en RUST se maneja con un stack y un heap
    
    OWNERSHIP RULES:
    -> cada valor en RUST tiene un owner
    -> solo puede haber un owner a la vez
    -> cuando el owner sale del scope, el valor se libera, es decir se elimina de la memoria.
     */
    //Ejemplo:
    let s = "hello";

    //O SEA: {                      // s is not valid here, it’s not yet declared
    //     let s = "hello";   // s is valid from this point forward

    //     // do stuff with s
    // }                      // this scope is now over, and s is no longer valid

    /*
    EL TIPO STRING:
    Tipo que sirve para explicar ownership.
    El tipo String, es un tipo que se puede modificar, a diferencia de &str.
    No tiene un tamaño definido en tiempo de compilación, por lo que se puede modificar.
    Este tipo de dato maneja la memoria en el heap, a diferencia de &str que maneja la memoria en el stack.
     */
    //Forma de declarar un String:
    let mut s = String::from("hello");
    //Este String es un tipo de dato que se puede modificar, a diferencia de &str.
    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // This will print `hello, world!`
    // El hecho de que se pueda modificar tiene que ver con la forma en que string maneja la memoria.

    }
