use std::io;
//para ver si 2 cosas son iguales
use std::cmp::Ordering;
//para generar cosas random
use rand::Rng;
use colored::*;

fn main() {
    println!("Juego de adivinar!");
    let secret_num = rand::thread_rng().gen_range(1..101);
    println!("numero secreto: {}", secret_num);
    
    loop {
        println!("Inserta un número pari");
        // mut es para especificar que la variable es mutable
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("error leyendo la linea");
        // saco espacios vacios
        let guess: u32 = match guess.trim().parse() {
            // si esta piola devuelve el numero
            Ok(num) => num,
            // sea cual sea el error, continua la iteracion
            Err(_) => continue,
        };
        println!("El numero que tiraste: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("{}","Muy chico pa".red()),
            Ordering::Greater => println!("{}","Muuy grande chavalllll".red()),
            Ordering::Equal => {
                println!("{}","Estoooo chaval!!".green());
                break;
            }
        }
    }
}