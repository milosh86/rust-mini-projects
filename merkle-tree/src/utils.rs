use sha2::{Digest, Sha256};

use merkle_tree::Hash;

use crate::merkle_tree;

pub fn hash(data: &[u8]) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash = hasher.finalize();
    hex::encode(hash)
}

pub fn combine_and_hash(data1: &[u8], data2: &[u8]) -> Hash {
    hash([data1, data2].concat().as_slice())
}
