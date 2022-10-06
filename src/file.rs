use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::BufReader;
use std::io::prelude::*;

use std::path::Path;
use std::io::Read;

const BUFFER_LEN: usize = 128 * 1024;

type Buffer = [u8; BUFFER_LEN];

pub fn encrypt_file(src: &Path, dst: &Path, mg: &mut dyn Read) -> std::io::Result<()> {
  let mut buffer: Buffer = [0; BUFFER_LEN];
  let mut mask_buffer: Buffer = [0; BUFFER_LEN];

  let src = OpenOptions::new().read(true).open(src)?;
  let dst = OpenOptions::new().read(true).write(true).create(true).open(dst)?;

  let mut reader = BufReader::new(src);
  let mut writer = BufWriter::new(dst);

  loop {
    let read_count = reader.read(&mut buffer)?;
    
    mg.read_exact(&mut mask_buffer[0..read_count])?;

    // XORing read and mask buffers 
    buffer.iter_mut().zip(mask_buffer.iter()).for_each(|(x1, x2)| *x1 ^= *x2);
    
    writer.write(&buffer[0..read_count])?;

    if read_count != BUFFER_LEN {
      break;
    }
  }

  Ok(())
}
