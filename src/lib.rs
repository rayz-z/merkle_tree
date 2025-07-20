use ark_ec;
use ark_ff;
use ark_sponge::{
    CryptographicSponge,
    poseidon::{self, PoseidonSponge},
};
use ark_std;
use core::array;
use core::cmp::PartialEq;

/*
-data structure defined tree
-adding, deleting and searching nodes
-binary tree
-fixed depth of 5? --> then fixed array

msgs are hashed and stored in leaf nodes
*/

// node i
// left child: 2i + 1
// right child: 2i + 2
// parent: floor((i - 2) / 2)
struct MerkleTree {
    pub root: Node,
    storage: [Node; 31],
    pub index: usize, // index of last node
}

#[derive(Clone)]
#[derive(PartialEq)]
struct Node {
    data: u8,
    index: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl MerkleTree {
    pub fn new(root: Node) -> Self {
        let storage: [Node; 31] = array::from_fn(|_| root.clone());

        MerkleTree {
            root,
            storage,
            index: 1,
        }
    }

    fn new_leaf(&mut self, hash: u8) {
        let node = Node {
            data: hash,
            index: self.index,
            left: None,
            right: None,
        };

        if self.index < 31 {
            self.storage[self.index] = node;
            self.index += 1;
        } else {
            println!("Max nodes reached");
        }
    }
}

// take hash of node and hash each child with the other child up the tree and compare to root node
fn verify_node(node: Node) -> bool {
    let phash = PoseidonSponge::new(node);
    let witnesses: Vec<u8> = vec![];

    let mut index = node.index.clone() as u8;

    while index > 0 {
        if index % 2 == 0 {
            let sibling = index - 1;
            witnesses.push(index);
            witnesses.push(sibling);
            index = (index - 2) / 2;
        } else {
            // check if right sibling exists
            
            index = (index - 2) / 2;
        }
    }

    let mut proof_hash: u8 = witnesses[0];
    for witness in 1..witnesses.len() {
        // hash
    }


}
