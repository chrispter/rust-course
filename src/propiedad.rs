fn main(){

    //   { let i = 8; }
    //   println!("El valor de i es: {}", i );
    
    // { let cadena = String::from("Hola amigos"); }
    // println!("{}", cadena)
    
    // let i: i32 = 7;
    // let j: i32 = i;
    
    // println!("{}", i);
    // println!("{}", j);
    
    // Solucion a la copia 
    // Utilizar el método clone()
    // En las variables que estén almacenadas en el heap, cuando queremos copiar habra que hacer
    // variable = variable1.clone()
    
    let cadena1 = String::from("Primera cadena");
    let cadena2 = cadena1.clone();
    
    println!("Cadean 1:{}", cadena1);
    println!("Cadena 2:{}", cadena2);
      
    }
    
    