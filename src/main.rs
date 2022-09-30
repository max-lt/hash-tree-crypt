mod tree;
mod hasher;

use tree::HTree;
use hasher::hash;

fn main() {
  println!("Hello, world!");

  let password = "hello world";
  let seed = hash(password.as_bytes());

  let mut tree: HTree = HTree::create(16, 0, seed);

//   println!("{:#?}", tree);
  println!("----");

  let i =  tree.last_leaf_index();

  println!("Tree last leaf index {}", i);
  println!("----");

  tree.goto(1);
  println!("----");
  tree.goto(5);
  println!("----");
  tree.goto(7);
  println!("----");
  tree.goto(12);
  println!("----");
  tree.goto(17);
  println!("----");
  tree.goto(12);
  println!("----");
  tree.goto(7);
}
