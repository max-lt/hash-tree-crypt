use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::BufReader;
use std::io::prelude::*;

use std::path::Path;
use std::io::Read;

const BUFFER_LEN: usize = 512;

type Buffer = [u8; BUFFER_LEN];

pub fn encrypt_file(src: &Path, dst: &Path, mg: &mut dyn Read) -> std::io::Result<()> {
  let mut buffer: Buffer = [0; BUFFER_LEN];
  let mut mask_buffer: Buffer = [0; BUFFER_LEN];

  let src = OpenOptions::new().read(true).open(src)?;
  let dst = OpenOptions::new().read(true).write(true).create(true).open(dst)?;

  let mut reader = BufReader::new(src);
  let mut writer = BufWriter::new(dst);

  loop {
    println!("rpos {}, wpos {}", reader.stream_position()?, writer.stream_position()?);

    let read_count = reader.read(&mut buffer)?;
    let mask_count = mask_buffer.len();
    mg.read_exact(&mut mask_buffer)?;

    // XORing raed and mask buffers 
    buffer.iter_mut().zip(mask_buffer.iter()).for_each(|(x1, x2)| *x1 ^= *x2);
    
    let write_count = writer.write(&buffer[0..read_count])?;

    println!("rc {}, wc {}, mc {}", read_count, write_count, mask_count);
    println!("mb {:x?}", &mask_buffer[0..mask_count]);
    // println!("w {:x?}", buffer);

    if read_count != BUFFER_LEN {
      break;
    }
  }

  Ok(())
}
