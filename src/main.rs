fn main() {
    // un comentario
    /*
     * comentario multilinea
     */
    // todo!() macro

    // variables

    let mut my_list = vec!["a", "b", "c"];
    println!("{:?}", my_list);
    my_list.push("d");
    println!("{my_list:?}");
    println!("{}", my_list[0]);
}
