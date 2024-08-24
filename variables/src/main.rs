use std::io;

fn main() {
    //---- VARIABELES MUTABLES Y NO MUTABLES ----
    
    // CASO QUE ESTA MAL: PORQUE NO DEFINIMOS A X COMO MUTABLE, OSEA ES UN VALOR 
    // ESTATICO
    // let x = 5;
    // print!("El valor de x es: {}", x);
    // x= 6;
    // print!("El valor de x es: {}", x);
    // de esta forma podemos redefinir valores 
    let mut x = 5;
    print!("El valor de x es: {}", x);
    x = 6;
    print!("El valor de x es: {}", x);
    /*
        SHADOWING:
        Es cuando se redeclara con let una variable:
        ventajas:
        -> se conserva al inmutabilidad de la variable, pero se cambia el valor   
     */
    let y = 5;
    println!("valor de y: {}", y);
    let y = "cinco";
    println!(" valor de y: {}", y);

    //---- CONSTANTES---
    // ¿porque poner las constantes si las variables por defecto
    // son inmutables?:
    // HAY DIFERENCIAS:
    // -> las constantes no pueden tener su valor asignado
    // por el ret de una funcion
    // -> tenemos que especificar de que tipo de dato son las constantes
    const NUMERO: u32 = 100;

}
