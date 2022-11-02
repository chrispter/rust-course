// Expreciones if

fn main(){
    let numero: i32 = 10;
    if numero < 10 {
        println!("Se cumplio la condicón");
    } else {
        println!("No se cumplio")
    }

    let cierto: bool = false;

    if cierto {
        println!("Se cumplio la condicón");
    } else {
        println!("No se cumplio")
    }

    let numero2 : i32  = 10;

    if numero2 < 5 {
        println!("El numero es menos que 5");
    } else if numero2 < 10 {
        println!("Esta comprendido entre 5 y 10");
    } else {
        println!("Es igual o mayor que 10");
    }

    let condicion: bool = false;
   
    let numero3: i32 = if condicion { 5 } else { 13 };

    println!("El valo de numero es: {}", numero3)
  
}

