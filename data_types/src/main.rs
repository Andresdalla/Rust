fn main() {
    //Tipos de datos escalares: tienen solo un unico valor
    //integer
    /*
        hay numeros enteros de distintos largos:
        Length   Signed   Unsigned
        8-bit    i8       u8
        16-bit   i16      u16
        32-bit   i32      u32
        64-bit   i64      u64
        128-bit  i128     u128
        por defecto los pone en i32
        tiene diferentes tipos de sistemas de nuemracion:
     */
        let a = 1000; //decimal
        let b = 0xff; //hex
        let c = 0o66; //octal
        let d = 0b111_000; //bin
        let e = b'A'; //bit
    //floating point numbers
    let f = 4.0;
    let g:f32 =3.0;
    // booleans
    let t = true;
    let u = false;
    //char
    let cum = 'c';
    //------------------------------------------
    // tipos compuestos
    //TUPLAS
    /*
        hay dos formas de separar los valores:
        una es desconstruirlos (1)
        y la otra es por . (2)
     */
    let tup = ("asda",1000);
    let (str, int) = tup; //(1)
    let str = tup.0; //(2)
    let int = tup.1; //(2)
    //ARRAYS
    /*
         De tamaño fijo, si queremos algo que varie de 
         tamaño, usar vectores

         forma de crear arrays:
         let byte = [0;8] ---> te crea un array de 8 valores todos con 0
         despues es todo igual a los otros lengujaes
 
     */ 
    let nums = [1,2,3,4,300];
    let val = nums[1];
    let bits = [0;8];   

    //FUNCIONES

    /*
        forma de decalra  el tipo de ret de una funcion:
        fn sum(x:i32, y:i32) -> i32{
            let sum = x + y;
            sum
            (o simplificado directamtte:) ----->(2)
            x + y 
        }
     */
    otra_fun(11,20);
    sum (11, 39);
    sum2(11,39);
}

fn otra_fun(x:i32, y:i32){
    println!("x : {}",x);
    println!("Y : {}",y);
}
fn sum(x:i32,y:i32)-> i32 {
    println!("valor x : {}",x);
    println!("valor Y : {}",y);
    let sum = x + y;
    sum
}
//(2)

fn sum2(x:i32,y:i32)-> i32 {
    println!("valor x : {}",x);
    println!("valor Y : {}",y);
    x + y
}