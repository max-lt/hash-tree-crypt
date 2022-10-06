use std::io::Read;
use std::io::Result;

#[derive(Debug)]
pub struct XKey {
  key: u8
}

impl XKey {
  pub fn new(key: u8) -> Self {
    Self { key }
  }
}

impl Read for XKey {
  fn read(&mut self, buffer: &mut [u8]) -> Result<usize> { 
    let len = buffer.len();

    buffer.fill_with(|| self.key);

    Ok(len)
  }
}
