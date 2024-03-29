use std::time::Instant;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::BufReader;
use std::io::Write;
use std::io::Read;

use log::info;

const BUFFER_LEN: usize = 128 * 1024;

type Buffer = [u8; BUFFER_LEN];

pub fn encrypt_file(src: &Path, dst: &Path, mg: &mut dyn Read) -> std::io::Result<()> {
  let src = OpenOptions::new().read(true).open(src)?;
  let dst = OpenOptions::new().read(true).write(true).create(true).open(dst)?;

  let reader = BufReader::new(src);
  let writer = BufWriter::new(dst);

  encrypt_stream(reader, writer, mg)
}

pub fn encrypt_stream<I: Read, O: Write>(mut reader: BufReader<I>, mut writer: BufWriter<O>, mg: &mut dyn Read) -> std::io::Result<()> {
  let mut buffer: Buffer = [0; BUFFER_LEN];
  let mut mask_buffer: Buffer = [0; BUFFER_LEN];
  
  let mut hash_i = blake3::Hasher::new();
  let mut hash_o = blake3::Hasher::new();

  let start = Instant::now();

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

  info!("Time {:?}", start.elapsed());
  info!("Input  file hash {}", hash_i.finalize().to_hex());
  info!("Output file hash {}", hash_o.finalize().to_hex());

  Ok(())
}
