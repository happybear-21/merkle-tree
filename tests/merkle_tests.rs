use merkle_tree::hash::Hasher;
use merkle_tree::node::Node;
use merkle_tree::proof::MerkleProof;
use merkle_tree::{MerkleTree, Sha256};

fn make_proof_for_index<H: Hasher>(tree: &MerkleTree, mut index: usize) -> MerkleProof {
    let mut proof_hashes: Vec<(Vec<u8>, bool)> = Vec::new();
    let mut current_level: Vec<Node> = tree.leaves.clone();

    while current_level.len() > 1 {
        let sibling_is_left = index % 2 == 1;
        let sibling_index = if sibling_is_left { index - 1 } else { index + 1 };
        let sibling_node = if sibling_index < current_level.len() {
            current_level[sibling_index].clone()
        } else {
            current_level[index].clone()
        };
        proof_hashes.push((sibling_node.hash.clone(), sibling_is_left));

        let mut next_level: Vec<Node> = Vec::with_capacity((current_level.len() + 1) / 2);
        for pair in current_level.chunks(2) {
            let left = pair[0].clone();
            let right = pair.get(1).cloned().unwrap_or_else(|| left.clone());
            next_level.push(Node::new_parent::<H>(left, right));
        }
        current_level = next_level;
        index /= 2;
    }

    MerkleProof { hashes: proof_hashes }
}

#[test]
fn test_root_hash_matches_manual_construction() {
    let data = vec![b"a".to_vec(), b"b".to_vec(), b"c".to_vec()];
    let tree = MerkleTree::new::<Sha256>(&data);

    // Manually compute expected root
    let ha = Sha256::hash(b"a");
    let hb = Sha256::hash(b"b");
    let hc = Sha256::hash(b"c");
    let p0 = Sha256::hash([ha.as_slice(), hb.as_slice()].concat().as_slice());
    let p1 = Sha256::hash([hc.as_slice(), hc.as_slice()].concat().as_slice());
    let expected_root = Sha256::hash([p0.as_slice(), p1.as_slice()].concat().as_slice());

    assert_eq!(tree.root_hash(), expected_root.as_slice());
}

#[test]
fn test_odd_leaves_duplicate_last() {
    let data = vec![b"x".to_vec(), b"y".to_vec(), b"z".to_vec()];
    let tree = MerkleTree::new::<Sha256>(&data);
    assert_eq!(tree.leaves.len() % 2, 0);

    let hz = Sha256::hash(b"z");
    assert_eq!(tree.leaves[2].hash, hz);
    assert_eq!(tree.leaves[3].hash, hz);
}

#[test]
fn test_merkle_proof_verification_true() {
    let data = vec![
        b"apple".to_vec(),
        b"banana".to_vec(),
        b"cherry".to_vec(),
        b"date".to_vec(),
        b"elderberry".to_vec(),
    ];
    let tree = MerkleTree::new::<Sha256>(&data);

    let index_under_test = 3; // "date"
    let leaf = &data[index_under_test];
    let proof = make_proof_for_index::<Sha256>(&tree, index_under_test);
    assert!(MerkleProof::verify::<Sha256>(leaf, &proof, tree.root_hash()));
}

#[test]
fn test_merkle_proof_verification_false_on_tampered_leaf() {
    let data = vec![
        b"apple".to_vec(),
        b"banana".to_vec(),
        b"cherry".to_vec(),
        b"date".to_vec(),
    ];
    let tree = MerkleTree::new::<Sha256>(&data);

    let index_under_test = 1; // "banana"
    let mut tampered_leaf = data[index_under_test].clone();
    tampered_leaf.push(0x00);

    let proof = make_proof_for_index::<Sha256>(&tree, index_under_test);
    assert!(!MerkleProof::verify::<Sha256>(&tampered_leaf, &proof, tree.root_hash()));
}

