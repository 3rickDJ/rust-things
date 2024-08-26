fn main() {
    // un comentario
    /*
     * comentario multilinea
     */
    // todo!() macro

    // variables

    let my_int: i64= 10;
    println!("{my_int}");
    let mut float_number = 10.0;
    float_number = float_number + my_int as f64;
    println!("{float_number}");

    let mut my_bool = true;
    println!("{my_bool}");

    my_bool = false || true;
    println!("{my_bool}");

    const MY_CONST:&str  = "hola";
    println!("{MY_CONST}");
    
}
