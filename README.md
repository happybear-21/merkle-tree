## Merkle Tree (Rust)

Simple, generic Merkle tree implementation using pluggable hashers (default: SHA-256). Includes a verification routine for Merkle proofs and an integration test suite.

### Project Layout
- `src/hash.rs`: `Hasher` and `Hashable` traits; `Sha256` implementation
- `src/node.rs`: `Node` structure and constructors
- `src/tree.rs`: `MerkleTree` implementation
- `src/proof.rs`: `MerkleProof` type and `verify` method
- `src/lib.rs`: crate exports
- `src/main.rs`: small demo printing the root hash
- `tests/merkle_tests.rs`: integration tests

### Prerequisites
- Rust toolchain (`rustup`, `cargo`) — see `https://rustup.rs`

### Build
```bash
cargo build
```

### Run Demo
The demo builds a tree from a few strings and prints the hex-encoded root.
```bash
cargo run
```

Example output:
```
MerkleTree 4e2a...<hex>
```

### Test
Run the integration tests:
```bash
cargo test
```

### Quick Start (Library Usage)
Add this crate as a dependency or use it within this workspace. Example:

```rust
use merkle_tree::{MerkleTree, Sha256};

fn main() {
    let data = vec![
        b"apple".to_vec(),
        b"banana".to_vec(),
        b"cherry".to_vec(),
        b"date".to_vec(),
    ];

    let tree = MerkleTree::new::<Sha256>(&data);
    let root = tree.root_hash();
    println!("root: {}", hex::encode(root));
}
```

### API Overview
- `trait Hasher { fn hash(data: &[u8]) -> Vec<u8>; }`
- `struct MerkleTree { root: Node, leaves: Vec<Node> }`
  - `MerkleTree::new::<H: Hasher>(leaves_data: &[Vec<u8>]) -> Self`
  - `MerkleTree::root_hash(&self) -> &[u8]`
- `struct MerkleProof { hashes: Vec<(Vec<u8>, bool)> }`
  - `MerkleProof::verify::<H: Hasher>(leaf: &[u8], proof: &MerkleProof, root_hash: &[u8]) -> bool`

Notes:
- For odd number of leaves, the last leaf is duplicated to make a full binary level.
- `MerkleProof::hashes` holds sibling hashes and a boolean indicating whether the sibling is on the left (`true`) or right (`false`).

### Creating Proofs (example approach)
This repo includes tests that build a proof from the current tree by walking levels. A similar approach can be adopted in production code: at each level, record the sibling hash and whether it is left or right, then move to the parent index (`i /= 2`). Once all levels are processed, the resulting proof can be verified using `MerkleProof::verify`.

### License
MIT


