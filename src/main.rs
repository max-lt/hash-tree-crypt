mod hasher;
mod file;
mod tree;
mod xor_reader;

use std::path::Path;
use std::env;
use file::encrypt_file;
use hasher::hash;
use tree::HTree;

#[derive(Debug, PartialEq)]
enum Operation {
  Encrypt,
  Decrypt,
  Invalid,
}

fn decode_operation(args: &Vec<String>) -> Operation {
  match &args[1][..] {
    "decrypt" => Operation::Decrypt,
    "encrypt" => Operation::Encrypt,
    &_ => Operation::Invalid,
  }
}

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

  if args.len() != 3 {
    println!("usage: ");
    return;
  }

  let op = decode_operation(&args);  
  if op == Operation::Invalid {
    println!("invalid operation '{}'", args[1]);
    println!("usage: ");
    return;
  }

  let file_path = &args[2];

  let source = Path::new(file_path);
  let dest_name = file_path.to_string() + ".";
  let dest_name = match op {
    Operation::Encrypt => dest_name  + "encrypted",
    Operation::Decrypt => dest_name  + "decrypted",
    Operation::Invalid => dest_name  + "invalid"
  };

  let dest = Path::new(&dest_name);

  println!("{:?} {:?}", op, source);

  let mut mask_generator = tree;

  match encrypt_file(source, dest, &mut mask_generator) {
    Ok(_) => (),
    Err(reason) => {
      println!("Error while reading {}: {}", file_path, reason);
    }
  }

  println!("done");
}
