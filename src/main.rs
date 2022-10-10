mod file;
mod tree;

use std::path::Path;
use std::env;
use file::encrypt_file;
use tree::HashTree;

fn main() {
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

  // ARGS
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    println!("usage: {} file", args[0]);
    return;
  }

  let file_path = &args[1];

  let source = Path::new(file_path);
  let dest = file_path.to_string() + ".tmp";
  let dest = Path::new(&dest);

  // Get file metadata
  let metadata = match std::fs::metadata(source) {
    Ok(m) => m,
    Err(reason) => {
      println!("Error while checking {}: {}", file_path, reason);
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

  match std::fs::rename(dest, source) {
    Ok(_) => (),
    Err(reason) => {
      println!("Error while renaming {:?}: {}", source, reason);
      return;
    }
  }

  println!("done");
}
