mod binary_tree;
use binary_tree::Node;
fn main() {
    let mut binary_tree = Node::new(5);
    binary_tree.add(2);
    binary_tree.add(1);
    binary_tree.add(3);
    binary_tree.add(4);

    binary_tree.add(7);
    binary_tree.add(10);
    println!("{:?}", binary_tree);

    println!("{:?}", binary_tree.search(4));
    binary_tree.delete(4);
    println!("{:?}", binary_tree.search(4));
    println!("{:?}", binary_tree);


}
