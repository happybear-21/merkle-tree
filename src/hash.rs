pub trait Hashable {
    fn hash(&self) -> Vec<u8>;
}

pub trait Hasher {
    fn hash(data: &[u8]) -> Vec<u8>;
}

pub struct Sha256;

impl Hasher for Sha256 {
    fn hash(data: &[u8]) -> Vec<u8> {
        use sha2::{Digest, Sha256};
        Sha256::digest(data).to_vec()
    }
}
