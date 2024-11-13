use std::io;

fn main() {
    // Reading input: page size, virtual memory size, physical memory size, num of pages
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let mut params: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse number"))
        .collect();
    let page_size = params[0];
    let virtual_memory_size = params[1];
    let physical_memory_size = params[2];
    let num_pages = params[3];

    // Read the page reference list
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let page_references: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Failed to parse reference"))
        .collect();

    // Initialize page table (using bits for control flags)
    let mut page_table: Vec<u8> = vec![0; num_pages]; // Control bits for each page
    let mut memory: Vec<i32> = vec![-1; physical_memory_size / page_size]; // Simulated physical memory

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
