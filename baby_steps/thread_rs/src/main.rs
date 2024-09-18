use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use std::time;
mod message;
use message::{ParitionMessage, ProcessMessage};
mod simulator;
use simulator::{BuddyAllocator, Dispatcher, Reader};


fn main() {
    let mut handles: Vec<JoinHandle<()>> = vec![];
    let (process_message_sender, dispatcher_receiver) = mpsc::channel::<ProcessMessage>();

    let reader = Reader::new("procesess.csv".to_string());
    let reader_thread = reader.run(process_message_sender);
    handles.push(reader_thread);

    let (dispatcher_sender, allocation_receiver) = mpsc::channel::<ProcessMessage>();

    let dispatcher = Dispatcher::new(dispatcher_receiver, dispatcher_sender);
    let dispatcher_handle = dispatcher.run();
    handles.push(dispatcher_handle);

    // let (allocation_sender, execution_receiver) = mpsc::channel::<MemoryPartition>();
    let mut allocator = BuddyAllocator::new(1024);
    let (allocation_sender, execution_receiver) = mpsc::channel::<ParitionMessage>();

    loop {
      if let Ok(message) = allocation_receiver.try_recv() {
        match message {
          ProcessMessage::Process(process) => {
            let mut result = allocator.allocate(process.size);
            while let None = result {
              match execution_receiver.try_recv() {
                Ok(ParitionMessage::Index(index)) => {
                  allocator.deallocate(index);
                },
                Ok(ParitionMessage::Quit) => {
                  break;
                },
                Err(_) => {}
              }
              result = allocator.allocate(process.size);
            }
            if let Some(index) = result {
              let sender_clone = allocation_sender.clone();
              let handle = thread::spawn(move || {
                println!("Proceso {} iniciado.", &process.name);
                thread::sleep(time::Duration::from_secs(process.execution_time));
                println!("Proceso {} completado.", &process.name);
                sender_clone.send(ParitionMessage::Index(index)).unwrap();
              });
              handles.push(handle);
            } 
          },
          ProcessMessage::Quit => break,
        }
      }
    }

    for message in allocation_receiver {
        match message {
            ProcessMessage::Process(process) => {
                println!("Proceso recibido en allocation: {:?}", &process);
            },
            ProcessMessage::Quit => {
                println!("Fin de la simulación.");
                break;
            }
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Simulación completada.");
}