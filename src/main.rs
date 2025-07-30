use merkle_tree::{MerkleTree, Node};
use sha2::{self, Digest, Sha256};
use hex;

fn main() {

    let hash = Sha256::digest(b"hello");
    let hash = hex::encode(hash);
    let node1 = Node{ hash, index: 0, left: None, right: None }; // what if index not 0?
    // let node2 = Node{ hash: String::from("hey"), index: 1, left: None, right: None };
    // let node3 = Node{ hash: String::from("hi"), index: 2, left: Some(Box::new(node1)), right: Some(Box::new(node2)) };

    // let _ = node3.display();
    let mut tree = MerkleTree::new(node1.clone());
    tree.new_leaf("hey");

    tree.display();

    // node1.display().unwrap();
}
