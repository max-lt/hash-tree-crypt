use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::BufReader;
use std::io::Write;

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

  let mut hash_i = blake3::Hasher::new();
  let mut hash_o = blake3::Hasher::new();

  loop {
    let read_count = reader.read(&mut buffer)?;

    hash_i.update(&buffer[0..read_count]);
    
    mg.read_exact(&mut mask_buffer[0..read_count])?;

    // XORing read and mask buffers
    for i in 0..read_count {
      buffer[i] ^= mask_buffer[i];
    }

    hash_o.update(&buffer[0..read_count]);
    
    writer.write(&buffer[0..read_count])?;

    if read_count != BUFFER_LEN {
      break;
    }
  }

  println!("File hash {:02x?} -> {:02x?}", hash_i.finalize().to_hex(), hash_o.finalize().to_hex());

  Ok(())
}
