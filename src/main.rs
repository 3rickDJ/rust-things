use serde_json::{Value, from_str};
use std::fs::File;
use std::io::{stdin, Read};

const PRESENTE: usize         = 0b00001;
#[allow(dead_code)]
const PROTECCION: usize       = 0b00010;
#[allow(dead_code)]
const MODIFICADO: usize       = 0b00100;
const REFERENCIA: usize       = 0b01000;
#[allow(dead_code)]
const CACHE_HABILITADA: usize = 0b10000;


fn main() {
    // Reading input: page size, virtual memory size, physical memory size, num of pages
    let (_unit, page_size, virtual_memory_size, physical_memory_size, _num_pages, page_references) = read_data("data.json");
    let offset_bits = ((page_size * 1024) as f64).log2().ceil() as usize;
    let page_bits = (virtual_memory_size as f64 / page_size as f64).log2().ceil() as usize;
    let frame_bits = (physical_memory_size as f64 / page_size as f64).log2().ceil() as usize;
    println!("offset bits: {}, page bits: {}, frame bits: {}", offset_bits, page_bits, frame_bits);

    // Initialize page table (using bits for control flags)
    let mut page_table: Vec<usize> = vec![0; virtual_memory_size / page_size]; // Control bits for each page
    let mut memory: Vec<i32> = vec![-1; physical_memory_size / page_size]; // Simulated physical memory

    let mut clock_pointer = 0; // Clock pointer to replace pages

    for reference in page_references {
        stdin().read(&mut [0u8]).unwrap(); // Wait for user input
        println!("\n\n\n\n\n\n\n\n\n\n\n\nProcessing reference: {}", reference);
        // Output memory state
        //if found in memory set reference bit
        let page_index = reference;
        let present = ((page_table[reference] >> frame_bits) & PRESENTE) == PRESENTE;
        if present {
            println!("Page {} found in memory", page_index);
            set_referencia(&mut page_table, page_index, offset_bits, frame_bits, true);
        } else {
            // If not found in memory, replace a page
            let mut replaced = false;
            while !replaced {
                if memory[clock_pointer] == -1 {
                    memory[clock_pointer] = page_index as i32;
                    page_table[page_index] = clock_pointer;
                    set_presente(&mut page_table, page_index, offset_bits, frame_bits, true);
                    clock_pointer = (clock_pointer + 1) % memory.len();
                    break;
                } 
                println!("Running clock algorithm...");
                let page_to_replace_number = memory[clock_pointer];
                let reference_bit = ((page_table[page_to_replace_number as usize] >> frame_bits) & REFERENCIA) == REFERENCIA;
                if !reference_bit {
                    // Replace page
                    println!("Replacing page {} with page {}", page_to_replace_number, page_index);
                    memory[clock_pointer] = page_index as i32;
                    page_table[page_index] = clock_pointer;
                    set_presente(&mut page_table, page_to_replace_number as usize, offset_bits, frame_bits, false);
                    set_presente(&mut page_table, page_index, offset_bits, frame_bits, true);
                    replaced = true;
                } else {
                    set_referencia(&mut page_table, page_to_replace_number as usize, offset_bits, frame_bits, false);
                }
                clock_pointer = (clock_pointer + 1) % memory.len();
            }
        }
        print_page_table(&page_table, frame_bits);
        print_memory_state(&memory);
    }
}

fn print_page_table(page_table: &Vec<usize>, frame_bits: usize) {
    println!("\nPage Table (in decimal, hex, binary):");
    for (i, &entry) in page_table.iter().enumerate() {
        println!(
            "Page {}: Decimal = {}, Hex = {:X}, Binary = {:08b} - {}",
            i, entry, entry, entry, format_control_bits(entry, frame_bits)
        );
    }
}

fn format_control_bits(control_bits: usize, frame_bits: usize) -> String {
    // Format control bits first Present bit, Protection bit, modified bit, reference bit and cache bit
    let present = ((control_bits >> frame_bits) & PRESENTE) == PRESENTE;
    let protection = ((control_bits >> frame_bits) & PROTECCION) == PROTECCION;
    let modified = ((control_bits >> frame_bits) & MODIFICADO) == MODIFICADO;
    let reference = ((control_bits >> frame_bits) & REFERENCIA) == REFERENCIA;
    let cache = ((control_bits >> frame_bits) & CACHE_HABILITADA) == CACHE_HABILITADA;
    return format!(
        "Present: {}, Protection: {}, Modified: {}, Reference: {}, Cache: {}",
        present, protection, modified, reference, cache
    );
}


fn print_memory_state(memory: &Vec<i32>) {
    println!("\nPhysical Memory State:");
    for (i, &frame) in memory.iter().enumerate() {
        println!("Frame {}: Page = {}", i, frame);
    }
}

fn read_data(file: &str) -> (String, usize, usize, usize, usize, Vec<usize>) {
    let mut file = match File::open(file) {
        Ok(f) => f,
        Err(e) => panic!("Error al leer el archivo {e}")
    };
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    let content = buffer.to_string();
    let v: Value = from_str(&content).unwrap();

    let page_size = v["pageSize"].as_u64().unwrap() as usize;
    let virtual_memory_size = v["virtualMemorySize"].as_u64().unwrap() as usize;
    let physical_memory_size = v["physicalMemorySize"].as_u64().unwrap() as usize;
    let _number_of_pages = v["numberOfPages"].as_u64().unwrap() as usize;
    let reference_list = v["referenceList"].as_array().unwrap().iter().map(|x| x.as_u64().unwrap() as usize).collect();
    let unit = v["unit"].as_str().unwrap().to_string();
    return (unit, page_size, virtual_memory_size, physical_memory_size, _number_of_pages, reference_list);
}


// Función para establecer el bit de presente
fn set_presente(page_table: &mut Vec<usize>, page_index: usize, _offset_bits: usize, frame_bits: usize, value: bool) {
    if value {
        page_table[page_index] |= PRESENTE << (frame_bits); // Establecer el bit de "presente"
    } else {
        page_table[page_index] &= !PRESENTE << (frame_bits); // Limpiar el bit de "presente"
    }
}

                //
                //TODO:
                //a set referencia agregarle el corrimiento de bits que utiliza el frame
// Función para establecer el bit de referencia
fn set_referencia(page_table: &mut Vec<usize>, page_index: usize, _offset_bits: usize, frame_bits: usize, value: bool) {
    if value {
        page_table[page_index] |= REFERENCIA << (frame_bits); // Establecer el bit de "referencia"
    } else {
        page_table[page_index] &= !REFERENCIA << (frame_bits); // Limpiar el bit de "referencia"
    }
}

// Función para establecer el bit de modificado
fn _set_modificado(page_table: &mut Vec<usize>, page_index: usize, value: bool) {
    if value {
        page_table[page_index] |= MODIFICADO; // Establecer el bit de "modificado"
    } else {
        page_table[page_index] &= !MODIFICADO; // Limpiar el bit de "modificado"
    }
}

// Función para establecer el bit de caché habilitada
fn _set_cache_habilitada(page_table: &mut Vec<usize>, page_index: usize, value: bool) {
    if value {
        page_table[page_index] |= CACHE_HABILITADA; // Establecer el bit de "caché habilitada"
    } else {
        page_table[page_index] &= !CACHE_HABILITADA; // Limpiar el bit de "caché habilitada"
    }
}
