use super::hasher::hash;
use debug_print::{ debug_println as debug };

use std::io::Read;
use std::io::Result;

const MAX_DEPTH: usize = 32;

#[derive(Debug)]
pub struct HTree {
  seed: [u8; 20],

  // Binary representation of the current path
  path: u32,

  mask: u32,

  // Max depth must be 32
  depth: u8,

  nodes: [[u8; 20]; MAX_DEPTH],

  // Read offset
  offset: usize
}

impl HTree {
  pub fn create(depth: u8, path: u32, seed: [u8; 20]) -> Self {
    if depth as usize > MAX_DEPTH {
      panic!("invalid depth {}", depth);
    }

    // 
    let mask: u32 = ((0xffffffffu64 << depth) ^ 0xffffffffu64) as u32;
    debug!("Mask is        {:032b} ({})", mask, mask);

    debug!("Target path is {:032b} ({})", path, path);
    let nodes: [[u8; 20]; MAX_DEPTH] = Default::default();

    debug!("Target path is ({:?})", nodes);

    let mut instance = Self { path, depth, mask, nodes, seed, offset: 0 };

    instance.compute_values(0);

    return instance;
  }

  fn compute_values(&mut self, index: u8) {
    let depth = self.depth as usize; 
    let path = self.path;

    let mut i = index as usize;
    let mut prev = match index { 0 => self.seed, _ => self.nodes[i - 1] };

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

      self.nodes[i] = value;

      // println!("Appending node {:2}; prev  = {:02x?}", i, prev);
      // println!("Appending node {:2}; value = {:02x?}", i, value);

      prev = value;

      i = i + 1;
    }
  }

  pub fn last_leaf_index(&self) -> u32 {
    return self.mask;
  }

  fn goto(&mut self, path: u32) {
    debug!("Current path is {:032b} ({})", self.path, self.path);
    debug!("Target  path is {:032b} ({})", path, path);

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
    debug!("Common path  is {:032b}", common);

    let ones = common.leading_ones() as usize;
    debug!("Reusable path is {} of {MAX_DEPTH}; useful = {}", ones, self.depth);
    
    // We now just have to count trailing zeros 
    let n = self.depth - (MAX_DEPTH - ones) as u8;
    debug!("Reusing {} nodes", n);

    // Assigning new path
    self.path = path;

    // Computing values
    self.compute_values(n);
  }
}

impl Read for HTree {
  fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
    let node = self.nodes[(self.depth - 1) as usize].to_vec();
    let nlen = node.len();
    let blen = buffer.len();

    let len = std::cmp::min(blen, nlen - self.offset);

    buffer[0..len].copy_from_slice(&node[self.offset..self.offset + len]);
    
    debug!("Writing buffer {:02x?} ({len})", &buffer[0..len]);
    
    // If buf was too small to be filled with current node value
    if (self.offset + len) < nlen {
      self.offset += len;
      debug!("Set offset to {}", self.offset);
    }
    // else we go to next node
    else {
      self.goto(self.path + 1);
      self.offset = 0;
    }

    Ok(len)
  }
}
