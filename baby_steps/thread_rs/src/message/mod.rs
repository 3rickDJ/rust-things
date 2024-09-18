mod process;
mod memory_partition;
pub use process::Process;
#[allow(unused)]
pub use memory_partition::MemoryPartition;
#[allow(unused)]
#[derive(Debug)]
pub enum ProcessMessage {
    Process(Process),
    Quit
}

#[allow(unused)]
#[derive(Debug)]
pub enum ParitionMessage {
    Index(usize),
    Quit,
}


