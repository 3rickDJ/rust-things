#[derive(Debug)]
pub struct MemoryPartition {
    pub size: usize,
    pub free: bool,
    pub index: usize,
}

impl MemoryPartition {
    pub fn new(size: usize, free: bool, index: usize) -> MemoryPartition {
        MemoryPartition {
            size,
            free,
            index,
        }
    }
}