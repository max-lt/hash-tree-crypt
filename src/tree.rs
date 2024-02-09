use debug_print::debug_println as debug;
use blake3::Hash;

use std::io::Read;
use std::io::Result;

const MAX_DEPTH: usize = 32;
const HASH_SIZE: usize = blake3::OUT_LEN;

type NodesArray = [Hash; MAX_DEPTH];

#[derive(Debug)]
pub struct HashTree {
  seed: Hash,

  // Binary representation of the current path
  // Note: this is also the node index
  path: u32,

  mask: u32,

  // Max depth must be 32
  depth: u8,

  nodes: NodesArray,

  // Read offset
  offset: usize
}

impl HashTree {
  /// Creates and initializes a tree structure
  ///
  /// # Arguments
  ///
  /// * `depth` - Tree depth (32 is the maximum)
  /// * `path` - The node from which to start
  /// * `seed` - A first hash to use as seed (typically a password hash)
  pub fn create(depth: u8, path: u32, seed: Hash) -> Self {
    if depth as usize > MAX_DEPTH {
      panic!("invalid depth {}", depth);
    }

    // 2 ^ depth without pow; mask equals last leaf index
    //
    // Examples
    //   depth = 7   -> 00000000000000000000000001111111 0x0000007f 127
    //   depth = 10  -> 00000000000000000000001111111111 0x000003ff 1023
    //   depth = 20  -> 00000000000011111111111111111111 0x000fffff 1048575
    let mask: u32 = ((0xffffffffu64 << depth) ^ 0xffffffffu64) as u32;
    debug!("Mask is        {:032b} ({})", mask, mask);
    debug!("Target path is {:032b} ({})", path, path);

    // Initialize a Hash (32 bytes array) array of MAX_DEPTH (32) elements
    let nodes: NodesArray = [0; MAX_DEPTH].map(|_| Hash::from([0; HASH_SIZE]));

    let mut instance = Self { path, depth, mask, nodes, seed, offset: 0 };

    // Initialize nodes
    instance.compute_values(0);

    return instance;
  }

  /// Computes nodes values from a given index
  ///
  /// # Arguments
  ///
  /// * `index` - The node from which to start (0 to initialize all nodes)
  fn compute_values(&mut self, index: u8) {
    let depth = self.depth as usize;
    let path = self.path;

    let mut i = index as usize;
    let mut prev: Hash = match index { 0 => self.seed, _ => self.nodes[i - 1] };

    while i < depth {
      let mask = 0x01 << (depth - i) - 1;

      debug!("Appending node {:2};  prev = {:02x?}", i, prev);

      let mut input = *prev.as_bytes();

      // Get node Direction, use reversed value for right node
      match path & mask {
        0 => (),
        _ => input.reverse()
      };

      let value = blake3::hash(&input);

      self.nodes[i] = value;

      debug!("Appending node {:2}; input =  {}({:02x?})", i,  match path & mask { 0 =>  "LFT", _ => "RGT" }, blake3::Hash::from(input).to_hex());
      debug!("Appending node {:2}; value = {:02x?}", i, value);

      prev = value;

      i = i + 1;
    }
  }

  /// Returns the last leaf index
  pub fn last_leaf_index(&self) -> usize {
    return self.mask as usize;
  }

  /// Returns the last byte index (last leaf index * hash size)
  pub fn last_byte_index(&self) -> usize {
    return self.last_leaf_index() * HASH_SIZE;
  }

  /// Sets a new path and computes nodes values accordingly
  ///
  /// # Arguments
  ///
  /// * `path` - New current path
  fn goto(&mut self, path: u32) {
    debug!("Mask is         {:032b} ({})", self.mask, self.mask);
    debug!("Current path is {:032b} ({})", self.path, self.path);
    debug!("Target  path is {:032b} ({})", path, path);

    // We do (p1 AND p2) OR (!p1 AND !p2) to get a common route
    // represented with 1s, the first 0 represents the first
    // forking node
    //
    // Examples
    //   (000 AND 001) OR (111 AND 110) = 110
    //   (001 AND 101) OR (110 AND 010) = 011
    //   (101 AND 111) OR (010 AND 000) = 101
    //
    // Note: leading zeros of !p1 and !p2 will prepend our result
    // with 1s, this is useful as we are looking for the first 0
    let common = (self.path & path) | (!self.path & !path);
    debug!("Common path  is {:032b}", common);

    let ones = common.leading_ones() as usize;
    debug!("Reusable path is {} of {MAX_DEPTH}; useful = {}", ones, self.depth);

    // We now just have to calculate first 0 index (first forking node)
    let n = self.depth - (MAX_DEPTH - ones) as u8;
    debug!("Reusing {} nodes", n);

    // Assigning new path
    self.path = path;

    // Computing values
    self.compute_values(n);
  }
}

impl Read for HashTree {
  fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
    let node = self.nodes[(self.depth - 1) as usize].as_bytes();
    let nlen = node.len();
    let blen = buffer.len();

    let len = std::cmp::min(blen, nlen - self.offset);

    buffer[0..len].copy_from_slice(&node[self.offset..self.offset + len]);

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
