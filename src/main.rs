mod tree;

use tree::HTree;

fn main() {
  println!("Hello, world!");
  let mut tree: HTree = HTree::create(16, 0);

  println!("{:?}", tree);
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
}
