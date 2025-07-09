use crate::hash::Hasher;

#[derive(Debug, Clone)]
pub struct MerkleProof {
    pub hashes: Vec<(Vec<u8>, bool)>,
}

impl MerkleProof {
    pub fn verify<H: Hasher>(leaf: &[u8], proof: &MerkleProof, root_hash: &[u8]) -> bool {
        let mut hash = H::hash(leaf);
        for (sibling_hash, is_left) in &proof.hashes {
            let data = if *is_left {
                [&sibling_hash[..], &hash[..]].concat()
            } else {
                [&hash[..], &sibling_hash[..]].concat()
            };
            hash = H::hash(&data);
        }
        hash == root_hash
    }
}
