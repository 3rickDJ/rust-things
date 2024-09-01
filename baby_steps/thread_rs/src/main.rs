use std::sync::{Arc, Mutex, Condvar};
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

    // Compartimientos de memoria compartidos entre hilos
    let compartimientos = vec![
        Arc::new((Mutex::new(Compartimiento { tamano: 901, libre: true }), Condvar::new())),
        Arc::new((Mutex::new(Compartimiento { tamano: 902, libre: true }), Condvar::new())),
        Arc::new((Mutex::new(Compartimiento { tamano: 903, libre: true }), Condvar::new())),
    ];

    let mut handles = vec![];

    for proceso in procesos {
        let compartimientos = compartimientos.clone();
        let handle = thread::spawn(move || {
            let mut compartimiento_asignado = None;

            while compartimiento_asignado.is_none() {
                for arc in compartimientos.iter() {
                    let (mutex, condvar) = &**arc; // Desreferenciar el Arc para obtener Mutex y Condvar
                    let mut compartimiento = mutex.lock().unwrap();

                    if compartimiento.libre && compartimiento.tamano >= proceso.tamano {
                        compartimiento.libre = false;
                        compartimiento_asignado = Some((Arc::clone(arc), condvar));
                        println!("{} está utilizando un compartimiento de {} unidades.", proceso.nombre, compartimiento.tamano);
                        break;
                    }
                }

                if compartimiento_asignado.is_none() {
                    // Espera activa antes de intentar de nuevo
                    thread::sleep(Duration::from_millis(100));
                }
            }

            // Simular la ejecución del proceso
            thread::sleep(Duration::from_secs(proceso.tiempo_ejecucion));
            println!("{} ha terminado la ejecución.", proceso.nombre);

            // Liberar el compartimiento y notificar a los otros hilos
            if let Some((arc, condvar)) = compartimiento_asignado {
                let (mutex, _) = &*arc;
                let mut compartimiento = mutex.lock().unwrap();
                compartimiento.libre = true;
                condvar.notify_one();
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