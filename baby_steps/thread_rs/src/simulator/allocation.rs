use std::collections::HashMap;

//ignore the unused field
#[allow(dead_code)]
pub struct BuddyAllocator {
  size: usize,
  arr: Vec<Vec<(usize, usize)>>,
  mp: HashMap<usize,usize>
}

impl BuddyAllocator {
  pub fn new(total_size: usize) -> Self {
    let n = (total_size as f64).log2().ceil() as usize;
    let mut arr: Vec<Vec<(usize,usize)>> = vec![vec![]; n+1];
    arr[n].push((0,(1<<n) - 1));
    BuddyAllocator {
      size: total_size,
      arr,
      mp: HashMap::new(),
    }
  }

  pub fn allocate(&mut self, size: usize) -> Option<usize> {
    let level = (size as f64).log2().ceil() as usize;
    if let Some((start,end)) = self.arr[level].pop(){
      self.mp.insert(start, end-start+1);
      return Some(start)
    }  else {
      let mut i = level;
      while i<self.arr.len() && self.arr[i].len() == 0{
        i += 1;
      }
      if i == self.arr.len() {
        return None
      } else {
        let (mut start, mut end) = self.arr[i].remove(0);
        while level != i {
          let mid = (start + end)/2;
          let p1 = (start, mid);
          let p2 = (mid + 1, end);
          self.arr[i-1].push(p1);
          self.arr[i-1].push(p2);
          (start,end) = self.arr[i-1].remove(0);
          i -= 1;
        }
        self.mp.insert(start, 1<<level);
        Some(start)
      }
    }
  }

  pub fn deallocate(&mut self, mut id: usize) -> () {
    if let Some(&size) = self.mp.get(&id){
      self.mp.remove(&id);
      let mut n = (size as f64).log2().ceil() as usize;
      let mut block: (usize, usize) = (id, id + size-1);
      //freed the block
      self.arr[n].push(block);
      // compact
      while n < self.arr.len() {
        let size = 1<<n;
        let buddy_is_right = id/size % 2 == 0;

        let buddy_id = if buddy_is_right { block.1 + 1 } else { block.0 - size };
        if let Some(index) = self.arr[n].iter().position(|&(start,_)| start == buddy_id ){
          let buddy = self.arr[n].remove(index);
          // uno
          let union = if buddy_is_right {
            (block.0, buddy.1)
          } else {
            (buddy.0, block.1)
          };
          //meto
          self.arr[n].retain(|&(start,_)| start != block.0);
          self.arr[n+1].push(union);
          block = union;
          id = block.0;
          n += 1;
        } else {
          break;
        }
      }
    } 
  }
}

#[cfg(test)]
mod test {
use super::*;

  #[test]
  fn test_buddy_instantiation() {
    let buddy = BuddyAllocator::new(1024);
    let arr: &Vec<Vec<(usize,usize)>> = &buddy.arr;
    assert_eq!(buddy.size, 1024);
    assert_eq!(buddy.arr.len(), 11);
    assert_eq!(buddy.arr[10].len(), 1);
  }

  #[test]
  fn test_buddy_allocation_max() {
    let mut buddy = BuddyAllocator::new(1024);
    let alloc_res = buddy.allocate(1024);
    assert_eq!(alloc_res,Some(0));
    assert_eq!(buddy.arr[10].len(), 0);
    let alloc_res = buddy.allocate(1024);
    assert_eq!(alloc_res,None);
  }

  #[test]
  fn test_buddy_allocation() {
    let mut buddy = BuddyAllocator::new(64);
    let alloc_size = 1;
    let alloc_res = buddy.allocate(alloc_size);
    assert_eq!(alloc_res,Some(0));
    assert_eq!(buddy.arr[0].len(), 1);
    assert_eq!(buddy.mp.get(&0),Some(&1));
    let alloc_res = buddy.allocate(alloc_size);
    assert_eq!(alloc_res,Some(1));
    assert_eq!(buddy.mp.get(&1),Some(&1));
    assert_eq!(buddy.arr[0].len(), 0);
  }

  #[test]
  fn test_buddy_allocation_twice() {
    let mut buddy = BuddyAllocator::new(64);
    let alloc_size = 8;
    let alloc_res = buddy.allocate(alloc_size);
    assert_eq!(alloc_res,Some(0));
    assert_eq!(buddy.arr[3].len(), 1);
    assert_eq!(buddy.mp.get(&0),Some(&8));
    let alloc_res = buddy.allocate(alloc_size);
    assert_eq!(alloc_res,Some(8));
    assert_eq!(buddy.mp.get(&8),Some(&8));
    assert_eq!(buddy.arr[3].len(), 0);
  }

  #[test]
  fn test_buddy_deallocation() {
    let mut buddy = BuddyAllocator::new(64);
    let alloc_size = 8;
    let id = buddy.allocate(alloc_size).unwrap();
    let id2 = buddy.allocate(alloc_size).unwrap();

    buddy.deallocate(id);
    assert_eq!(buddy.mp.get(&id), None);
    assert_eq!(buddy.arr[3].len(), 1);

    buddy.deallocate(id2);
    assert_eq!(buddy.mp.get(&id2), None);
    assert_eq!(buddy.arr[3].len(), 0);
  }

}

use crate::message::MemoryPartition;

pub struct FixedPartitionAllocator {
  arr: Vec<MemoryPartition>
}

impl FixedPartitionAllocator {
  pub fn new(partitions: Vec<usize>) -> Self {
    let mut arr: Vec<MemoryPartition> = Vec::new();
    for (i, &size) in partitions.iter().enumerate(){
      arr.push(MemoryPartition::new(size, true, i));
    }
    FixedPartitionAllocator { arr }
  }

  pub fn allocate(&mut self, size: usize) -> Option<usize> {
    if let Some(pos) = self.arr.iter().position(|x| x.free && x.size >= size) {
      self.arr[pos].free = false;
      Some(pos)
    } else {
      None
    }
  }

  pub fn deallocate(&mut self, index: usize) {
    self.arr[index].free = true;
  }
}
#[cfg(test)]
mod test_fixed_partitions {
  use super::*;

  #[test]
  fn test_fixed_partition_instantiation() {
    let partitions = vec![32, 64, 128];
    let allocator = FixedPartitionAllocator::new(partitions);
    assert_eq!(allocator.arr.len(), 3);
    assert_eq!(allocator.arr[0].size, 32);
    assert_eq!(allocator.arr[1].size, 64);
    assert_eq!(allocator.arr[2].size, 128);
  }

  #[test]
  fn test_fixed_partition_allocation() {
    let partitions = vec![32, 64, 128];
    let mut allocator = FixedPartitionAllocator::new(partitions);
    let alloc_res = allocator.allocate(32);
    assert_eq!(alloc_res, Some(0));
    assert_eq!(allocator.arr[0].free, false);
    let alloc_res = allocator.allocate(64);
    assert_eq!(alloc_res, Some(1));
    assert_eq!(allocator.arr[1].free, false);
    let alloc_res = allocator.allocate(128);
    assert_eq!(alloc_res, Some(2));
    assert_eq!(allocator.arr[2].free, false);
  }

  #[test]
  fn test_fixed_partition_allocation_failure() {
    let partitions = vec![32, 64, 128];
    let mut allocator = FixedPartitionAllocator::new(partitions);
    let alloc_res = allocator.allocate(256);
    assert_eq!(alloc_res, None);
  }

  #[test]
  fn test_fixed_partition_deallocation() {
    let partitions = vec![32, 64, 128];
    let mut allocator = FixedPartitionAllocator::new(partitions);
    let alloc_res = allocator.allocate(32).unwrap();
    assert_eq!(allocator.arr[0].free, false);
    allocator.deallocate(alloc_res);
    assert_eq!(allocator.arr[0].free, true);
  }

  #[test]
  fn test_fixed_partition_reallocation() {
    let partitions = vec![32, 64, 128];
    let mut allocator = FixedPartitionAllocator::new(partitions);
    let alloc_res = allocator.allocate(32).unwrap();
    allocator.deallocate(alloc_res);
    let alloc_res = allocator.allocate(32);
    assert_eq!(alloc_res, Some(0));
    assert_eq!(allocator.arr[0].free, false);
  }
}
