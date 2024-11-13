use serde_json::{Value, from_str};
use std::io;
use std::fs::File;
use std::io::Read;

fn main() {
    // Reading input: page size, virtual memory size, physical memory size, num of pages
    let (page_size, virtual_memory_size, physical_memory_size, num_pages, page_references) = read_data("data.json");

    // Initialize page table (using bits for control flags)
    let mut page_table: Vec<u8> = vec![0; virtual_memory_size/page_size]; // Control bits for each page
    let mut memory: Vec<i32> = vec![-1; physical_memory_size/ page_size]; // Simulated physical memory

    let mut clock_pointer = 0; // Clock pointer to replace pages

    for reference in page_references {
        println!("\nProcessing reference: {}", reference);

        // Translate the virtual address to physical memory
        let page_index = reference; // In this case, we directly use the page reference

        // Check if the page is already in physical memory
        let mut page_found = false;
        for (i, &frame) in memory.iter().enumerate() {
            if frame == page_index as i32 {
                page_found = true;
                println!("Page {} found in memory at frame {}", page_index, i);
                break;
            }
        }

        if !page_found {
            // Page fault occurred, handle it with clock algorithm
            println!("Page fault! Replacing a page...");

            // Find the next frame to replace using the clock algorithm
            loop {
                let current_bit = (page_table[clock_pointer] & 1) != 0; // Check the reference bit
                if !current_bit {
                    // Replace the page at clock_pointer
                    memory[clock_pointer] = page_index as i32;
                    page_table[clock_pointer] |= 1; // Set the present bit
                    println!("Replaced page {} in frame {}.", page_index, clock_pointer);
                    clock_pointer = (clock_pointer + 1) % (physical_memory_size / page_size);
                    break;
                } else {
                    // Reset the reference bit and move the clock pointer
                    page_table[clock_pointer] &= 0b11111110; // Clear the reference bit
                    clock_pointer = (clock_pointer + 1) % (physical_memory_size / page_size);
                }
                if(reference == 4) {
                    println!("current_bit: {}", current_bit);
                    println!("page_table[clock_pointer]: {}", page_table[clock_pointer]);
                    println!("clock_pointer: {}", clock_pointer);
                }
            }
        }

        // Output memory state
        print_page_table(&page_table);
        print_memory_state(&memory);
    }
}

fn print_page_table(page_table: &Vec<u8>) {
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