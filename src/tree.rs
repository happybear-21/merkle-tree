use crate::hash::Hasher;
use crate::node::Node;

pub struct MerkleTree {
    pub root: Node,
    pub leaves: Vec<Node>,
}

impl MerkleTree {
    pub fn new<H: Hasher>(leaves_data: &[Vec<u8>]) -> Self {
        let mut leaves: Vec<Node> = leaves_data
            .iter()
            .map(|data| Node::new_leaf::<H>(data))
            .collect();
        while leaves.len() % 2 != 0 {
            leaves.push(leaves.last().unwrap().clone());
        }
        let root = MerkleTree::build_tree::<H>(leaves.clone());
        MerkleTree { root, leaves }
    }

    fn build_tree<H: Hasher>(mut nodes: Vec<Node>) -> Node {
        while nodes.len() > 1 {
            let mut next_level = Vec::with_capacity((nodes.len() + 1) / 2);
            for pair in nodes.chunks(2) {
                let left = pair[0].clone();
                let right = pair.get(1).cloned().unwrap_or_else(|| left.clone());
                next_level.push(Node::new_parent::<H>(left, right));
            }
            nodes = next_level;
        }
        nodes.remove(0)
    }

    pub fn root_hash(&self) -> &[u8] {
        &self.root.hash
    }
}
