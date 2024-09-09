mod process;
mod memory_partition;
pub use process::Process;
pub use memory_partition::MemoryPartition;
#[derive(Debug)]
pub enum ProcessMessage {
    Process(Process),
    Quit
}

#[derive(Debug)]
pub enum ParitionMessage {
    Index(usize),
    Quit,
}


