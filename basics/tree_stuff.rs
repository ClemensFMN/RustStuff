// based on https://matthias-endler.de/2017/boxes-and-trees/

struct Tree {
  root: i64,
  left: Option<Box<Tree>>,
  right: Option<Box<Tree>>
}


fn printTree(t: Tree) {
  println!("{}", t.root);
  match t.left {
    None => println!("No left subtree"),
    Some(tl) => printTree(*tl)
  };
  match t.right {
    None => println!("No right subtree"),
    Some(tr) => printTree(*tr)
  };

}


fn main() {
  let t1 = Tree {
    root: 1,
    left: Some(Box::new(Tree {
      root: 2,
      left: None,
      right: None})),
    right: Some(Box::new(Tree {
      root: 3,
      left: None,
      right: None}))
    };

  printTree(t1);
  }
