use std::collections::HashMap;

pub struct BuddyAllocator {
  size: usize,
  arr: Vec<Vec<(usize, usize)>>,
  mp: HashMap<usize,usize>
}

impl BuddyAllocator {
  pub fn new(total_size: usize) -> Self {
    let n = (total_size as f64).log2().ceil() as usize;
    let mut arr: Vec<Vec<(usize,usize)>> = vec![vec![]; n+1];
    arr[n].push((0,total_size));
    BuddyAllocator {
      size: total_size,
      arr,
      mp: HashMap::new(),
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
    println!("arr: {arr:?}");
    assert_eq!(buddy.size, 1025);
    assert_eq!(buddy.arr.len(), 11);
    assert_eq!(buddy.arr[10].len(), 1);
  }

  fn test_buddy_allocation() {
  }
}
