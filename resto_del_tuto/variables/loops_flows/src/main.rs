fn main() {
    // IF
    // funcionamiento comun
    //la condicion debe ser booleana
    let numero = 5;
    if numero < 10{
        println!("Primera cond true")
    }else if numero > 10{
        println!("Segunda cond true")
    }else{
        println!("condicon falsa")
    }
    // se puedeen definir variables con control flows:
    let condicion = true;
    let number = if condicion {6} else {2};

    // LOOPS 3 TIPOS:
    // 1) LOOP: se repite hasta un break
    loop{
        println!("aaaaa");
        break;
    }
    // podemos asignar valores con loops
    let mut cont = 0;
    let result = loop {
        cont += 1;
        if cont == 8 {
            break cont;
        }
    };
    // CLASICO WHILE
    let mut aaa = 0;

    while aaa < 10 {
        println!("jijo");
        aaa -= 1;
    }

    //FOR
    // usamos el iterador iter
    let i = [10,20,30,40];
    for element in i.iter(){
        println!("{}",element);
    }
    //rangos:
    for number in 1..4{
        println!("{}",number);
    }
}
