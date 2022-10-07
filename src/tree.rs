use super::hasher::hash;

use std::io::Read;
use std::io::Result;

const MAX_DEPTH: u8 = 32;

#[derive(Debug)]
pub struct HTree {
  pub seed: [u8; 20],

  // Binary representation of the current path
  pub path: u32,

  // Max depth must be 32
  pub depth: u8,

  pub nodes: [[u8; 20]; MAX_DEPTH as usize],

  // Read offset
  offset: usize
}

impl HTree {
  pub fn create(depth: u8, path: u32, seed: [u8; 20]) -> Self {
    if depth > MAX_DEPTH {
      panic!("invalid depth {}", depth);
    }

    // println!("Target path is {:032b} ({})", path, path);
    let nodes: [[u8; 20]; MAX_DEPTH as usize] = [[0; 20]; MAX_DEPTH as usize];

    let mut instance = Self { path, depth, nodes, seed, offset: 0 };

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

      // println!("Appending node {:2}; prev  = {:02x?}", i, prev);

      // Get node Direction
      // Note: do not rotate 10, 10 left and 10 right are the same
      match path & mask {
        0 => prev.rotate_left(10), 
        _ => prev.reverse()
      };

      let value = hash(&prev);

      self.nodes[i as usize] = value;

      // println!("Appending node {:2}; prev  = {:02x?}", i, prev);
      // println!("Appending node {:2}; value = {:02x?}", i, value);

      prev = value;

      i = i + 1;
    }
  }

  pub fn last_leaf_index(&self) -> u32 {
    return ((0xffffffffu64 << self.depth) ^ 0xffffffffu64) as u32;
  }

  pub fn goto(&mut self, path: u32) {
    // let mask = self.last_leaf_index();
    // println!("Mask is         {:032b} ({})", mask, mask);

    // println!("Current path is {:032b} ({})", self.path, self.path);
    // println!("Target  path is {:032b} ({})", path, path);

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
    // println!("Common path  is {:032b}", common);

    let ones = common.leading_ones() as u8;
    // println!("Reusable path is {} of {MAX_DEPTH}; useful = {}", ones, self.depth);
    
    // We now just have to count trailing zeros 
    let n = self.depth - (MAX_DEPTH - ones);
    // println!("Reusing {} nodes", n);

    // Assigning new path
    self.path = path;

    // Computing values
    self.compute_values(n);
  }
}

impl Read for HTree {
  fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
    let node = self.nodes[(self.depth - 1) as usize];
    let nlen = node.len();
    let blen = buffer.len();

    let len = std::cmp::min(blen, nlen - self.offset);

    buffer[0..len].copy_from_slice(&node[self.offset..self.offset + len]);
    
    // println!("Writing buffer {:x?} ({len})", &buffer[0..len]);
    
    // If buf was too small to be filled with current node value
    if (self.offset + len) < nlen {
      self.offset += len;
      // println!("Set offset to {}", self.offset);
    }
    // else we go to next node
    else {
      self.goto(self.path + 1);
      self.offset = 0;
    }

    Ok(len)
  }
}
