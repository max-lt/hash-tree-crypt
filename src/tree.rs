use super::hasher::hash;

const MAX_DEPTH: u8 = 16;

#[derive(Debug)]
pub struct HTree {
  pub seed: [u8; 20],

  // Binary representation of the current path
  pub path: u16,

  // Max depth must be 16
  pub depth: u8,

  pub nodes: [[u8; 20]; MAX_DEPTH as usize]
}

impl HTree {
  pub fn create(depth: u8, path: u16, seed: [u8; 20]) -> Self {
    if depth > MAX_DEPTH {
      panic!("invalid depth {}", depth);
    }

    println!("Target path is {:016b} ({})", path, path);
    let nodes: [[u8; 20]; MAX_DEPTH as usize] = [[0; 20]; MAX_DEPTH as usize];

    let mut instance = Self { path, depth, nodes, seed };

    instance.compute_values(0);

    return instance;
  }

  fn compute_values(&mut self, index: u8) {
    let depth = self.depth; 
    let path = self.path;

    let mut i = index;
    let mut prev = match index { 0 => self.seed, _ => self.nodes[(i - 1) as usize] };
    while i < depth {
      let mask = 0x01 << (depth - i) - 1;
      
      // Get node Direction
      match path & mask {
        0 => prev.rotate_left(10), 
        _ => prev.rotate_right(10)
      };

      let value = hash(&prev);

      self.nodes[i as usize] = value;

      // println!("Appending node {:2}; prev = {:?}; value = {:?}", i, prev, value);
      println!("Appending node {:2}; value = {:02x?}", i, value);

      prev = value;

      i = i + 1;
    }
  }

  pub fn last_leaf_index(&self) -> u16 {
    return ((0xffff << self.depth) ^ 0xffff) as u16;
  }

  pub fn goto(&mut self, path: u16) {
    let mask = self.last_leaf_index();
    println!("Mask is         {:016b} ({})", mask, mask);

    println!("Current path is {:016b} ({})", self.path, self.path);
    println!("Target  path is {:016b} ({})", path, path);

    // We do (p1 AND p2) OR (!p1 AND !p2) to get a common route
    // reprensented with 1s, the first 0 represents the first
    // forking node
    //
    // Examples
    //    (000 AND 001) OR (111 AND 110) = 110
    //    (001 AND 101) OR (110 AND 010) = 011
    //    (101 AND 111) OR (010 AND 000) = 101
    //
    // Note: leading zeros of !p1 and !p2 will prepend our result
    // with 1s, this is useful as we are looking for the first 0
    let common = (self.path & path) | (!self.path & !path);
    println!("Common path  is {:016b}", common);

    let ones = common.leading_ones() as u8;
    println!("Reusable path is {} of 16; useful = {}", ones, self.depth);
    
    // We now just have to count trailing zeros 
    let n = self.depth - (16 - ones);
    println!("Reusing {} nodes", n);

    // Assigning new path
    self.path = path;

    // Computing values
    self.compute_values(n);
  }
}
