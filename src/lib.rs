use core::array;
use core::cmp::PartialEq;
use sha2::{Sha256, Digest};

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
    storage: [Option<Node>; 31],
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
        let storage: [Option<Node>; 31] = array::from_fn(|_| None);

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
            self.storage[self.index] = Some(node);
            self.index += 1;
        } else {
            println!("Max nodes reached");
        }
        // need to add conditions to update the parent and ancestor nodes
    }

    fn search_for_node_at_index(self: &Self, index: usize) -> Option<Node> { // check indexing since option might return a val
        if index < 0 || index > 31 || index > self.index {
            panic!("index out of bounds");
        } else {
            self.storage[index].clone()
        }
    }

}

// take hash of node and hash each child with the other child up the tree and compare to root node
fn verify_node(node: Node, tree: MerkleTree) -> bool {
    let path_to_node: Vec<usize> = vec![];

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
                path_to_node.push(tree.storage[index + 1].unwrap().index);
            } else {
                path_to_node.push(0); // hash with 0 if no right sibling, but this is only a problem if they are in the unfilled row
            }
            index = (index - 2) / 2;
        }
    }

    let mut hasher = Sha256::new();

    for witness in 1..path_to_node.len() {
        // hash
        hasher.update(witness);
        if hasher.finalize() != tree.storage[witness] {
            false;
        }
    }

    true
}
