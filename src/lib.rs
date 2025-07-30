use core::array;
use core::cmp::PartialEq;
use hex;
use sha2::{Digest, Sha256};
use std::error::Error;

/*
-data structure defined tree
-fixed depth of 5? --> then fixed array
*/

// node i
// left child: 2i + 1
// right child: 2i + 2
// parent: floor((i - 2) / 2)
pub struct MerkleTree {
    pub root: Node,
    storage: [Option<Node>; 31],
    pub index: usize, // index of last node
}

#[derive(Clone, PartialEq)]
pub struct Node {
    pub hash: String,
    pub index: usize,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn display(self: &Self) -> Result<(), Box<dyn Error>> {
        println!("node: {}", self.hash);
        if self.left == None && self.right == None {
            println!("no left child or right child");
        } else if self.left == None && !self.right.is_none() {
            println!("right child: {}", self.right.as_ref().unwrap().hash);
        } else if !self.left.is_none() && self.right == None {
            println!("left child: {}", self.left.as_ref().unwrap().hash);
        } else {
            println!("left child: {}", self.left.as_ref().unwrap().hash);
            println!("right child: {}", self.right.as_ref().unwrap().hash);
        }
        Ok(())
    }
}

impl MerkleTree {
    pub fn new(root: Node) -> Self {
        // why doesn't it error if I don't put in a node?
        let storage: [Option<Node>; 31] = array::from_fn(|_| None);

        MerkleTree {
            root,
            storage,
            index: 0,
        }
    }

    pub fn new_leaf(&mut self, msg: &str) {
        // why do we need to add pub?
        let msg = String::from(msg);
        let hash = Sha256::digest(msg.as_bytes());
        let hash = hex::encode(hash);

        let node = Node {
            hash,
            index: self.index,
            left: None,
            right: None,
        };

        if self.index < 31 {
            self.storage[self.index] = Some(node);
            self.index += 1;
        } else {
            println!("Max nodes reached");
        }

        // need to add conditions to update the parent and ancestor nodes
        let indexer = self.index;
        while indexer > 0 {}
    }

    pub fn search_for_node_at_index(self: &Self, index: usize) -> Option<Node> {
        if index > 31 || index > self.index {
            panic!("index out of bounds");
        } else {
            self.storage[index].clone()
        }
    }

    pub fn delete_node() {} // take an index and shift all others behind it up -> an array shift but also reassign parents/hashes

    pub fn display(self: &Self) {

    }
}

// take hash of node and hash each child with the other child up the tree and compare to root node
pub fn verify_node(node: Node, tree: MerkleTree) -> bool {
    let mut path_to_node: Vec<usize> = vec![];

    let mut index = node.index.clone();

    while index > 0 {
        if index % 2 == 0 {
            let sibling = index - 1;
            path_to_node.push(index);
            path_to_node.push(sibling);
            index = (index - 2) / 2;
        } else {
            // check if right sibling exists
            if tree.storage[index + 1usize] != None {
                path_to_node.push(tree.storage[index + 1].clone().unwrap().index);
            } else {
                path_to_node.push(0); // hash with 0 if no right sibling, but this is only a problem if they are in the unfilled row
            }
            index = (index - 2) / 2;
        }
    }

    for witness in 1..path_to_node.len() {
        // hash
        let mut hasher = Sha256::new();
        hasher.update(tree.storage[witness].clone().unwrap().hash.as_bytes());
        if hex::encode(hasher.finalize()) != tree.storage[witness].clone().unwrap().hash {
            return false;
        }
    }

    true
}
