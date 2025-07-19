use ark_ec;
use ark_ff;
use ark_std;


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
    pub index: u8, // index of last node
}

struct Node {
    data: u8, 
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl MerkleTree {
    pub fn new(root: Node) -> Self {
        Node
        let storage: [Node; 31] = {}
        MerkleTree { root, storage }
    }

    fn new_leaf()
}

// combine all nodes needed to prove node into an array of type proof? enum?
fn prove_node() {}

// take hash of node and hash each child with the other child up the tree and compaire to root node
fn verify_node() {}