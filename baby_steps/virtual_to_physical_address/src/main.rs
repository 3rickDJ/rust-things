use std::io::stdin;
fn main() {
    let mut input = String::new();
    // take input
    let input = match stdin().read_line(&mut input) {
        Ok(0) => {panic!("Error, nada se ha leido")},
        Ok(_) => input.trim(),
        Err(e) => {panic!("Error al leer la línea: {e}")}
    };

    let number = match u64::from_str_radix(&input, 2) {
        Ok(n) => n,
        Err(e) => panic!("Error al transformar a binario: {e}")
    };
    println!("{number:b}");
}
