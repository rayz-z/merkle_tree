use merkle_tree::{MerkleTree, Node};
use sha2::{self, Digest, Sha256};
use hex;

fn main() {
    println!("Hello, world!");


    let hash = Sha256::digest(b"hello");
    let hash = hex::encode(hash);
    let node = Node{ hash, index: 0, left: None, right: None }; // what if index not 0?

    let tree = MerkleTree::new(node);
}
