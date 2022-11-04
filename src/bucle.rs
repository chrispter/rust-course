// Control de flujo
fn main(){
    // LOOP
    // let mut contador = 0;
    // let resultado = loop {

    //     println!("¡Hola amigo!");
    //     contador +=1;

    //     if contador == 10 {
    //         break contador * contador;
    //     }
    // };

    // println!("El valor del resultado es: {}", resultado);

    //WHILE
    // let mut contador = 10;

    // while contador != 0 {
    //     println!("Contador: {}", contador);
    //     contador -= 1;
    // }

    // let matriz : [i32; 9] = [10, 20, 30, 40, 50, 60 ,70, 80, 90];
    // let mut index = 0;

    // while index < 9 {
    //     println!("Valor de array en la posicion {} : {}", index + 1, matriz[index]);
    //     index += 1;
    // }

    // FOR
    // let matriz : [i32; 9] = [10, 20, 30, 40, 50, 60 ,70, 80, 90];

    // for numero in matriz.iter(){
    //     println!("Valor del elemento es : {}", numero);
    // }

    for numero in 1..=10 {
        println!("El valor es : {}", numero)
    }

    for numero in (1..=10).rev() {
        print!(" ");
        println!("El valor es : {}", numero)
    }

  
}


