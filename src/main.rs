use merkle_tree::{MerkleTree, Sha256};

fn main() {
    let data = vec![
        b"apple".to_vec(),
        b"banana".to_vec(),
        b"apple".to_vec(),
        b"date".to_vec(),
    ];

    let tree = MerkleTree::new::<Sha256>(&data);
    println!("MerkleTree {:?}", hex::encode(tree.root_hash()));
}
