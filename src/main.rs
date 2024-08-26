fn main() {
    // un comentario
    /*
     * comentario multilinea
     */
    // todo!() macro

    // variables
    let mut mutable_string: String = String::from("Hola mundo");
    println!("{mutable_string}");
    mutable_string = String::from("o 2");
    println!("{mutable_string}");

    let mut my_int: i32 = 7;
    print!("{my_int}");

    my_int = my_int + 3;
    print!("{my_int}");

    
}
