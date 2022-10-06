mod hasher;
mod file;
mod tree;

use std::path::Path;
use std::env;
use file::encrypt_file;
use hasher::hash;
use tree::HTree;

fn main() {
  let password = "hello world";
  let seed = hash(password.as_bytes());

  let mut tree: HTree = HTree::create(16, 0, seed);

  let last_leaf_index = tree.last_leaf_index() as usize;
  let max_file_size = last_leaf_index * seed.len();
  println!("Tree last leaf index: {}, max file size: {}", last_leaf_index, max_file_size);
  println!("----");

  // ARGS
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    println!("usage: ");
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
  if metadata.len() > max_file_size as u64 {
    println!("Error: {:?} is too big ({} > {})", source, metadata.len(), max_file_size);
    return;
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
