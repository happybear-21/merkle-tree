use crate::hash::Hasher;

#[derive(Debug, Clone)]
pub struct Node {
    pub hash: Vec<u8>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn new_leaf<H: Hasher>(data: &[u8]) -> Self {
        Self {
            hash: H::hash(data),
            left: None,
            right: None,
        }
    }

    pub fn new_parent<H: Hasher>(left: Node, right: Node) -> Self {
        let combined = [left.hash.as_slice(), right.hash.as_slice()].concat();
        Self {
            hash: H::hash(&combined),
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
}
