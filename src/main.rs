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

  //   println!("{:#?}", tree);
  println!("----");

  let i = tree.last_leaf_index();

  println!("Tree last leaf index {}", i);
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

  match encrypt_file(source, dest, &mut tree) {
    Ok(_) => (),
    Err(reason) => {
      println!("Error while reading {}: {}", file_path, reason);
      return;
    }
  }

  match std::fs::rename(dest, source) {
    Ok(_) => (),
    Err(reason) => {
      println!("Error while renaming {}: {}", file_path, reason);
      return;
    }
  }

  println!("done");
}
