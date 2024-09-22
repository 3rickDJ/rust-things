use std::io::{stdin, Read};
use std::mem::size_of;
use std::fs::File;

const USIZE_SIZE: usize = size_of::<usize>() * 4;

fn main() {
    let (page_size, number_of_pages, number_of_frames, mut page_table) = read_configuration_and_page_table("page_table.txt");

    let _page_bits = (number_of_pages as f64).log2().ceil() as usize;
    let frame_bits = (number_of_frames as f64).log2().ceil() as usize;
    let offset_bits = (page_size as f64).log2().ceil() as usize;


    loop {
        println!("Ingrese la dirección virtual en binario junto con su desplazamiento:");
        let mut input = String::new();
        // take input
        let input = match stdin().read_line(&mut input) {
            Ok(0) => {panic!("Error, nada se ha leido")},
            Ok(_) => input.trim().replace("_", ""),
            Err(e) => {panic!("Error al leer la línea: {e}")}
        };

        let number = match usize::from_str_radix(&input, 2) {
            Ok(n) => n,
            Err(e) => panic!("Error al transformar a binario: {e}")
        };
        let page_number = number >> offset_bits;
        let page_entry = page_table[page_number];
        let present_bit = (page_entry >> frame_bits) & 1;
        println!("Entrada de la tabla de páginas: {:>48}", format_binary(page_entry));
        println!("Direccion física: {:>62}", format_binary(page_entry & ((1 << frame_bits) - 1)));
        println!("Desplazamiento: {:>64}", format_binary(number & ((1 << offset_bits) - 1)));
        println!("Bits asociados: {:>64}", format_binary(page_entry >> frame_bits));
        if present_bit == 0 {
            println!("La página no está presente en memoria principal");
            continue;
        }
        println!("");
    }
}

fn format_binary(number: usize) -> String {
    //each byte separated  by a '_', and left padded with 0 with a total of 32 characters
    let binary = format!("{:0USIZE_SIZE$b}", number);
    let mut formatted = String::new();
    for (i, c) in binary.chars().enumerate() {
        formatted.push(c);
        if (i + 1) % 8 == 0  && i != USIZE_SIZE - 1{
            formatted.push('_');
        }
    }
    formatted
}

fn read_configuration_and_page_table(filename: &str) -> (usize, usize, usize, Vec<usize>) {
    let mut file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => panic!("Error al abrir el archivo: {e}")
    };

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    let mut lines = buffer.lines();
    let configuration = lines.next().unwrap();
    let page_table = lines.map(|l| usize::from_str_radix(l, 2).unwrap()).collect();
    let mut config = configuration.split_whitespace();
    let page_size = config.next().unwrap().parse().unwrap();
    let number_of_pages = config.next().unwrap().parse().unwrap();
    let number_of_frames = config.next().unwrap().parse().unwrap();
    (page_size, number_of_pages, number_of_frames, page_table)
}