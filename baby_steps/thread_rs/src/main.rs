use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Estructura para representar un proceso
struct Proceso {
    nombre: String,
    tiempo_ejecucion: u64, // Tiempo de ejecución en segundos
    tamano: usize,         // Tamaño del proceso
}

// Estructura para representar un compartimiento de memoria
struct Compartimiento {
    tamano: usize,
    libre: bool,
}

fn main() {
    // Lista de procesos
    let procesos = vec![
        Proceso { nombre: String::from("Proceso 1"), tiempo_ejecucion: 3, tamano: 100 },
        Proceso { nombre: String::from("Proceso 2"), tiempo_ejecucion: 2, tamano: 200 },
        Proceso { nombre: String::from("Proceso 3"), tiempo_ejecucion: 5, tamano: 150 },
    ];

    // Lista de compartimientos de memoria, cada uno protegido por un Mutex
    let compartimientos = vec![
        Arc::new(Mutex::new(Compartimiento { tamano: 200, libre: true })),
        Arc::new(Mutex::new(Compartimiento { tamano: 100, libre: true })),
        Arc::new(Mutex::new(Compartimiento { tamano: 300, libre: true })),
    ];

    let mut handles = vec![];

    for proceso in procesos {
        let compartimientos = compartimientos.clone();
        let handle = thread::spawn(move || {
            // Intentar encontrar y asignar un compartimiento de memoria al proceso
            let compartimiento_encontrado = compartimientos.iter().find(|&comp| {
                let mut comp = comp.lock().unwrap();
                if comp.libre && comp.tamano >= proceso.tamano {
                    comp.libre = false;
                    println!("{} está utilizando un compartimiento de {} unidades.", proceso.nombre, comp.tamano);
                    true
                } else {
                    false
                }
            });

            if let Some(compartimiento) = compartimiento_encontrado {
                // Simular la ejecución del proceso
                thread::sleep(Duration::from_secs(proceso.tiempo_ejecucion));
                println!("{} ha terminado la ejecución.", proceso.nombre);

                // Liberar el compartimiento
                let mut comp = compartimiento.lock().unwrap();
                comp.libre = true;
            } else {
                println!("No hay compartimientos de memoria disponibles para {}.", proceso.nombre);
            }
        });

        handles.push(handle);
    }

    // Esperar a que todos los hilos terminen
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Simulación completada.");
}
