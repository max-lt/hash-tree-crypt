mod file;
mod tree;
mod args;

use std::path::PathBuf;
use file::encrypt_file;
use tree::HashTree;

use crate::args::Args;

fn main() {
  let matches = args::cli().get_matches();

  // Print version and exit
  if matches.get_flag(Args::Version.into()) {
    println!("hash-tree-crypt v{}", std::env!("CARGO_PKG_VERSION"));
    return;
  }
  
  let source = match matches.get_one::<PathBuf>(Args::Input.into()) {
    Some(p) => p.to_owned(),
    None => {
      println!("Error: missing input file");
      return;
    }
  };

  let mut user_set_output = true;
  let dest = match matches.get_one::<PathBuf>(Args::Output.into()) {
    Some(p) => p.to_owned(),
    None => {
      user_set_output = false;
      let mut p = source.clone();
      p.set_file_name(p.file_name().unwrap().to_str().unwrap().to_string() + ".htcrypt");
      p.to_owned()
    }
  };

  // If dest is auto generated, we ensure we don't overwrite an existing file
  let dest = match !user_set_output && dest.exists() {
    false => dest,
    true => {
      let mut dest = dest.clone();
      let time = chrono::Local::now().timestamp_millis().to_string();
      dest.set_extension(time + ".htcrypt" );
      dest
    },
  };

  println!("Input file:  {:?}", source);
  println!("Output file: {:?}", dest);

  let source = source.as_path();
  let dest = dest.as_path();

  println!("Enter encryption password: ");
  let password = rpassword::read_password().unwrap();

  println!("Verifying encryption password: ");
  if password != rpassword::read_password().unwrap() {
    println!("Password verification failed.");
    return;
  }

  // Seed is the hash of the password
  let seed = blake3::hash(password.as_bytes());

  // Create our tree
  let mut tree = HashTree::create(20, 0, seed);

  println!("Tree last leaf index: {}, max file size: {}", tree.last_leaf_index(), tree.last_byte_index());
  println!("----");

  // Get file metadata
  let metadata = match std::fs::metadata(source) {
    Ok(m) => m,
    Err(reason) => {
      println!("Error while checking {:?}: {}", source, reason);
      return;
    }
  };

  // Checking if file
  if !metadata.is_file() {
    println!("Error: {:?} is not a file", source);
    return;
  }

  // Checking if file size < pad size
  let max_file_size = tree.last_byte_index();
  if metadata.len() > max_file_size as u64 {
    println!("Error: {:?} is too big ({} > {})", source, metadata.len(), max_file_size);
    return;
  }

  // Prevent reading big file in debug mode
  #[cfg(debug_assertions)]
  if metadata.len() > 1024*1024 {
    panic!("You don't want to print debug logs for a such large file");
  }

  match encrypt_file(source, dest, &mut tree) {
    Ok(_) => (),
    Err(reason) => {
      println!("Error while reading {:?}: {}", source, reason);
      return;
    }
  }

  println!("File encrypted successfully");
  println!("To reverse the process, run the same command on the encrypted file");
}
