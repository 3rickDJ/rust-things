fn main() {

    let mut my_list = vec!["a", "b", "c"];
    println!("{:?}", my_list);
    my_list.push("d");
    println!("{my_list:?}");
    println!("{}", my_list[my_list.len() - 1]);
}
