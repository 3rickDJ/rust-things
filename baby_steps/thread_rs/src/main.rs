#![allow(unused)]
use std::sync::{mpsc};
use std::thread;
use std::time::Duration;

// Estructura para representar un proceso
//copy trait
#[derive(Clone)]
struct Process {
    name: usize,
    execution_time: u64, // Tiempo de ejecución en segundos
    size: usize,         // Tamaño del proceso
}

// Estructura para representar un compartimiento de memoria
#[derive(Debug)]
struct MemoryPartition {
    size: usize,
    free: bool,
    index: usize,
}

fn main() {
    // Lista de procesos
    let mut procesos = vec![
        Process { name: 1, execution_time: 3, size: 100 },
        Process { name: 2, execution_time: 3, size: 100 },
        Process { name: 3, execution_time: 3, size: 100 },
        Process { name: 4, execution_time: 3, size: 100 },
        Process { name: 5, execution_time: 3, size: 100 },
        Process { name: 6, execution_time: 3, size: 100 },
        Process { name: 7, execution_time: 3, size: 100 },
    ];

    // Compartimientos de memoria compartidos entre hilos
    let mut compartimientos = vec![
        MemoryPartition { size: 200, free: true, index:0 },
        MemoryPartition { size: 200, free: true, index:1 },
        MemoryPartition { size: 200, free: true, index:2 },
        MemoryPartition { size: 200, free: true, index:3 },
    ];

    let mut handles = vec![];
    let (sender, receiver) = mpsc::channel();

    while procesos.len() > 0 {
        let proceso = procesos[0].clone();

        let available_partition = compartimientos.iter_mut().find(|x| x.free && x.size >= proceso.size);
        match available_partition {
            Some(partition) => {
                partition.free = false;
                let index = partition.index;
                let sender_clone = sender.clone();
                let time = proceso.execution_time;
                let name = proceso.name;
                let handle = thread::spawn(move|| {
                    println!("\tProceso {} en ejecución en el compartimiento {}.", name, index);
                    thread::sleep(Duration::from_secs(time));
                    println!("\t\tProceso {} finalizado.", name);
                    sender_clone.send(index).unwrap();
                });
                handles.push(handle);
                procesos.remove(0);
            },
            _ => {}
        }

        match receiver.try_recv() {
            Ok(proceso_finalizado) => {
                let partition = compartimientos.iter_mut().find(|x| x.index == proceso_finalizado);
                match partition {
                    Some(partition) => {
                        partition.free = true;
                    },
                    _ => {
                        println!("No se encontró el compartimiento del proceso {}.", proceso_finalizado);
                    }
                }
            },
            Err(e) => {}
        }
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Simulación completada.");
}