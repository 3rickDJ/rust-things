use std::sync::mpsc;
use std::thread::JoinHandle;
mod message;
use message::{MemoryPartition, ProcessMessage};
mod simulator;
use simulator::{Reader, Dispatcher};


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

    let (allocation_sender, execution_receiver) = mpsc::channel::<MemoryPartition>();

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