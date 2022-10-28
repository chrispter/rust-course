fn main(){
    // let mut x:i32 = 5;
    // println!("The value of x is: {}", x);
    // x = -366999;
    // println!("The value of x is: {}", x);
    // let decimal1 = 4.0;
    // let decimal2 = -455.399;

    // Operaciones basicas
    let suma = 58 + 77;
    let resta = 3245.55 - 788.23;
    let multiplicacion = 455.3 * 23.0;
    let division = 34 / 4;
    let resto = 34 % 4;

    // Tipo boolean
    let verdadero: bool = true;
    let falso: bool = false;

    // Tipo caracter
    let a: char = 'a';
    let emoji: char = '😁';

    // Tipos compuestos
    // Tuplas y arrays

    let tupla = (500.34, 57, -344.9 );

    // desestructuración
    let (x, y, z) = tupla;
    println!("El segundo valor de la tupla es: {}", y);

    // tipo array(matriz)
    let matriz:[i32;4] = [23, 45, 2, -11];
    let x: i32 = matriz[0];


}