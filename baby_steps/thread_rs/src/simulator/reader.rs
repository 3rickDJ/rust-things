use std::{sync::mpsc::Sender, thread, fs::File};
use std::io::{BufRead, BufReader};
use crate::message::{Process, ProcessMessage};
use rand::Rng;
use std::time::Duration;

pub struct Reader {
  file_path: String,
}

impl Reader {
  pub fn new(file_path: String) -> Reader {
    Reader {
      file_path,
    }
  }
  
  pub fn run(self, process_message_sender: Sender<ProcessMessage>) -> thread::JoinHandle<()> {
    thread::spawn( move || {
      let file = File::open(self.file_path).unwrap_or_else(|e| {
        panic!("No se pudo abrir el archivo: {}", e);
      });
      let reader = BufReader::new(file);
      let chunk = 8;
      for (i, line) in reader.lines().enumerate(){
        if ((i+1) % chunk) == 0 {
          let rand_time = rand::thread_rng().gen_range(1..3);
          thread::sleep(Duration::from_secs(rand_time));
          // print!("FinChunk 🔘{}", i);
        }
        let line = line.unwrap();
        let process_message = ProcessMessage::Process(Process::from_string(&line));
        let rand_time = rand::thread_rng().gen_range(1..1000);
        thread::sleep(Duration::from_millis(rand_time));
        process_message_sender.send(process_message).unwrap();
      }
    })
  }
}