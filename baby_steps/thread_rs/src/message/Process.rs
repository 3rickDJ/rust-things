
#[derive(Debug)]
#[derive(Clone)]
pub struct Process {
    pub name: usize,
    pub execution_time: u64, // Tiempo de ejecución en segundos
    pub size: usize,         // Tamaño del proceso
}

impl Process  {
    pub fn new(name: usize, execution_time: u64, size: usize) -> Process {
        Process {
            name,
            execution_time,
            size,
        }
    }

    pub fn from_string(line: &str) -> Process {
        let parts : Vec<&str> = line.split(',').collect();
        let name = parts[0].parse::<usize>().unwrap();
        let execution_time = parts[1].parse::<u64>().unwrap();
        let size = parts[2].parse::<usize>().unwrap();
        Process::new(name, execution_time, size)
    }
}