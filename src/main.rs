use std::io::Read;
use std::fs::File;
use serde_json::from_str;
use serde_json::Value;
fn main() {
    let (page_size, virtual_memory_size, physical_memory_size, _number_of_pages, reference_list) =read_data("data.json");
    println!("Page size: {}", page_size);
    println!("Virtual memory size: {}", virtual_memory_size);
    println!("Physical memory size: {}", physical_memory_size);
    println!("Number of pages: {}", _number_of_pages);
    println!("Reference list: {:?}", reference_list);
    let mut page_table = vec![0_usize; virtual_memory_size / page_size];
    let mut aging_page_table = vec![0_u32; virtual_memory_size / page_size];
    println!("Page table: {:?}", page_table);
    println!("virtual_memory_size / page_size: {}", virtual_memory_size / page_size);
    //clock algorithm
    let mut clock = 0;
    let mut page_faults = 0;
    for reference in reference_list.iter() {
        let page = *reference;
        if page_table[page] == 0 {
            page_faults += 1;
            while page_table[clock] == 1 {
                page_table[clock] = 0;
                clock = (clock + 1) % page_table.len();
            }
            page_table[clock] = 1;
            clock = (clock + 1) % page_table.len();
        }
    }
}

fn read_data(file: &str)-> (usize, usize, usize, usize, Vec<usize>)  {
    let mut file = match File::open(file) {
        Ok(f) => f,
        Err(e) => panic!("Error al leer el archivo {e}")
    };
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    let content =  buffer.to_string();
    let v: Value = from_str(&content).unwrap();

    let page_size = v["pageSize"]["value"].as_u64().unwrap() as usize;
    let virtual_memory_size = v["virtualMemorySize"]["value"].as_u64().unwrap() as usize;
    let physical_memory_size = v["physicalMemorySize"]["value"].as_u64().unwrap() as usize;
    let _number_of_pages = v["numberOfPages"].as_u64().unwrap() as usize;
    let reference_list = v["referenceList"].as_array().unwrap().iter().map(|x| x.as_u64().unwrap() as usize).collect();
    return (page_size, virtual_memory_size, physical_memory_size, _number_of_pages, reference_list);
}