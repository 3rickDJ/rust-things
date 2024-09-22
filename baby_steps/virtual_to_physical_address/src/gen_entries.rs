use std::collections::HashSet;
use std::io::Write;
use rand::Rng;
use std::mem::size_of;
use std::fs::File;

const USIZE_SIZE: usize = size_of::<usize>() * 8;
fn main() {
  let page_size = 1024;
  let number_of_pages = 256;
  let number_of_frames = 128;
  let mut page_table: Vec<usize> = vec![0; number_of_pages];
  let mut rng = rand::thread_rng();
  let frame_bits = (number_of_frames as f64).log2().ceil() as usize;

  let mut used_frames = HashSet::new();

  for i in 0..number_of_pages {
    let mut info_bits: usize = rng.gen_range(0..=127) & 0b1111110;
    let mut tries = 5;
    let mut page_frame: usize = 0;
    while tries > 0 {
      page_frame = rng.gen_range(0..(number_of_frames - 1));
      if !used_frames.contains(&page_frame) {
        used_frames.insert(page_frame);
        info_bits |= 1;
        break;
      }
      tries -= 1;
    }

    let entry = (info_bits << frame_bits) | page_frame;
    page_table[i] = entry;
  }

  //write to a file the sizes and the page table
  let mut file = File::create("page_table.txt").unwrap();
  let sizes = format!("{} {} {}\n", page_size, number_of_pages, number_of_frames);
  file.write_all(sizes.as_bytes()).unwrap();
  for entry in page_table {
    let entry_str = format!("{:0width$b}\n", entry, width=USIZE_SIZE);
    file.write_all(entry_str.as_bytes()).unwrap();
  }

}
