#[derive(Debug)]
pub struct MemoryPartition {
    size: usize,
    free: bool,
    index: usize,
}
