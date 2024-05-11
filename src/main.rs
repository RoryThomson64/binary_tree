mod binary_tree;
use binary_tree::Node;
fn main() {
    let mut binary_tree = Node::new(5);
    binary_tree.add(2);
    binary_tree.add(7);
    binary_tree.add(10);
    println!("{:?}", binary_tree);
    println!("10: {}", binary_tree.search(10))
}
