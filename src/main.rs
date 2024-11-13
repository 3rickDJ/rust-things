use serde_json::{Value, from_str};
use std::fs::File;
use std::io::Read;

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
    let offset_bits = ((page_size * 1024) as f64).log2() as usize;
    let page_bits = (virtual_memory_size as f64 / page_size as f64).log2() as usize;
    let frame_bits = (physical_memory_size as f64 / page_size as f64).log2() as usize;
    println!("offset bits: {}, page bits: {}, frame bits: {}", offset_bits, page_bits, frame_bits);

    // Initialize page table (using bits for control flags)
    let mut page_table: Vec<usize> = vec![0; virtual_memory_size / page_size]; // Control bits for each page
    let mut memory: Vec<i32> = vec![-1; physical_memory_size / page_size]; // Simulated physical memory
    let mut free_frames = (physical_memory_size / page_size) as i32; // Counter for free frames

    let mut clock_pointer = 0; // Clock pointer to replace pages

    for reference in page_references {
        println!("\nProcessing reference: {}", reference);
        // imprimir la direccion virtual, o sea,  bits de control | en binario los bits que ocupe representar el total de la memoria virtual y el numero | desplazameinto (bits que ocupan tamano de pagina)
        // e imprimir sus bits
        // y el l

        // Translate the virtual address to physical memory
        let page_index = reference; // In this case, we directly use the page reference

        // Check if the page is already in physical memory
        let mut page_found = false;
        for (i, &frame) in memory.iter().enumerate() {
            if frame == page_index as i32 {
                page_found = true;
                // Page found: set the reference bit to 1
                set_referencia(&mut page_table, i, offset_bits, frame_bits, true); // Set the reference bit
                println!("Page {} found in memory at frame {}.", page_index, i);
                // imprimir la direccion fisica, o sea,  bits de control | en binario los bits que ocupe representar el total de la memoria fisica y el numero | desplazameinto (bits que ocupan tamano de pagina)
                break;
            }
        }

        if !page_found {
            if free_frames > 0 {
                // Page fault occurred, but there are free frames. Place the page sequentially.
                println!("Page fault! Placing page {} in memory.", page_index);
                let mut placed = false;
                for (i, frame) in memory.iter_mut().enumerate() {
                    if *frame == -1 {
                        *frame = page_index as i32; // Place the page in the first available frame
                        free_frames -= 1;
                        // Set the reference bit to 1, indicating that the page is used.
                        set_referencia(&mut page_table, i, offset_bits, frame_bits, true);  // Set the reference bit
                        set_presente(&mut page_table, i, offset_bits, frame_bits, true);    // Mark as present
                        println!("Page {} placed in frame {}.", page_index, i);
                        placed = true;
                        // imprimir la direccion fisica, o sea,  bits de control | en binario los bits que ocupe representar el total de la memoria fisica y el numero | desplazameinto (bits que ocupan tamano de pagina)
                        break;
                    }
                }
                if !placed {
                    println!("No free frames available!");
                }
            } else {
                // Page fault occurred, and no free frames. Handle it with the clock algorithm.
                println!("Page fault! Replacing a page...");

                // Find the next frame to replace using the clock algorithm
                loop {
                    let current_bit = ((page_table[clock_pointer] >> (offset_bits + frame_bits) ) & REFERENCIA ) != 0; // Check the reference bit
                    if !current_bit {
                        // Replace the page at clock_pointer
                        memory[clock_pointer] = page_index as i32;
                        set_presente(&mut page_table, clock_pointer, offset_bits, frame_bits, true); // Set the present bit
                        set_referencia(&mut page_table, clock_pointer, offset_bits, frame_bits, true); // Set the reference bit
                        println!("Replaced page {} in frame {}.", page_index, clock_pointer);
                        clock_pointer = (clock_pointer + 1) % (physical_memory_size / page_size);
                        // imprimir la direccion fisica, o sea,  bits de control | en binario los bits que ocupe representar el total de la memoria fisica y el numero | desplazameinto (bits que ocupan tamano de pagina)
                        break;
                    } else {
                        // Reset the reference bit and move the clock pointer
                        set_referencia(&mut page_table, clock_pointer, offset_bits, frame_bits, false); // Clear the reference bit
                        clock_pointer = (clock_pointer + 1) % (physical_memory_size / page_size);
                    }
                }
            }
        }

        // Output memory state
        print_page_table(&page_table);
        print_memory_state(&memory);
    }
}

fn print_page_table(page_table: &Vec<usize>) {
    println!("\nPage Table (in decimal, hex, binary):");
    for (i, &entry) in page_table.iter().enumerate() {
        println!(
            "Page {}: Decimal = {}, Hex = {:X}, Binary = {:08b}",
            i, entry, entry, entry
        );
    }
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
fn set_presente(page_table: &mut Vec<usize>, page_index: usize, offset_bits: usize, frame_bits: usize, value: bool) {
    if value {
        page_table[page_index] |= PRESENTE << (offset_bits + frame_bits); // Establecer el bit de "presente"
    } else {
        page_table[page_index] &= !PRESENTE << (offset_bits + frame_bits); // Limpiar el bit de "presente"
    }
}

                //
                //TODO:
                //a set referencia agregarle el corrimiento de bits que utiliza el frame
// Función para establecer el bit de referencia
fn set_referencia(page_table: &mut Vec<usize>, page_index: usize, offset_bits: usize, frame_bits: usize, value: bool) {
    if value {
        page_table[page_index] |= REFERENCIA << (offset_bits + frame_bits); // Establecer el bit de "referencia"
    } else {
        page_table[page_index] &= !REFERENCIA << (offset_bits + frame_bits); // Limpiar el bit de "referencia"
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
