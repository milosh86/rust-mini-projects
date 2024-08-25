use std::rc::Rc;

use crate::utils;

pub type Hash = String;
// pub type Hash = [u8; 32];

struct MerkleTree<'a> {
    root: &'a MerkleTreeNode,
}

#[derive(Clone, PartialEq, Debug)]
enum MerkleTreeNode {
    Leaf {
        hash: Hash,
        original_data: Vec<u8>,
        index: usize,
    },
    Node {
        hash: Hash,
        left: Rc<MerkleTreeNode>,
        right: Rc<MerkleTreeNode>,
    },
}

impl MerkleTreeNode {
    fn get_hash(&self) -> &Hash {
        match self {
            MerkleTreeNode::Leaf {
                hash,
                original_data,
                index: _,
            } => hash,
            MerkleTreeNode::Node { hash, left, right } => hash,
        }
    }

    fn get_original_value(&self) -> &str {
        match self {
            MerkleTreeNode::Leaf {
                hash,
                original_data,
                index: _,
            } => match std::str::from_utf8(original_data) {
                Ok(string) => string,
                Err(e) => "",
            },
            MerkleTreeNode::Node { hash, left, right } => "",
        }
    }
}

fn prepare_leaf_level(data_blocks: Vec<&[u8]>) -> Vec<Rc<MerkleTreeNode>> {
    let mut leafs: Vec<_> = data_blocks
        .iter()
        .enumerate()
        .map(|(idx, data)| {
            Rc::new(MerkleTreeNode::Leaf {
                original_data: data.to_vec(),
                hash: utils::hash(*data),
                index: idx,
            })
        })
        .collect();
    let size = leafs.len();

    // if needed, make it even by cloning the last one
    if size % 2 != 0 {
        leafs.push(leafs[size - 1].clone());
    }

    leafs
}

fn prepare_node_level(leaf_level: Vec<Rc<MerkleTreeNode>>) -> Vec<Rc<MerkleTreeNode>> {
    let mut iter = leaf_level.into_iter();
    let mut result = vec![];

    loop {
        let leaf_node_1 = iter.next();
        let leaf_node_2 = iter.next();

        let node_1 = match leaf_node_1 {
            Some(node) => node,
            None => {
                // if None, it's the end of iterator, return result
                return result;
            }
        };

        // if needed, make it even by cloning the last one
        let node_2 = leaf_node_2.unwrap_or(node_1.clone());

        let node_1_hash = match &*node_1 {
            MerkleTreeNode::Node {
                hash,
                left: _,
                right: _,
            } => hash,
            MerkleTreeNode::Leaf {
                hash,
                original_data: _,
                index: _,
            } => hash,
        };

        let node_2_hash = match &*node_2 {
            MerkleTreeNode::Node {
                hash,
                left: _,
                right: _,
            } => hash,
            MerkleTreeNode::Leaf {
                hash,
                original_data: _,
                index: _,
            } => hash,
        };

        result.push(Rc::new(MerkleTreeNode::Node {
            hash: utils::combine_and_hash(node_1_hash.as_bytes(), node_2_hash.as_bytes()),
            left: Rc::clone(&node_1),
            right: Rc::clone(&node_2),
        }));
    }
}

#[derive(Clone, Debug)]
struct Proof {
    hash: Hash,
    index: usize,
    siblings: Vec<Hash>,
}

fn build_proofs(node: Rc<MerkleTreeNode>, path: Vec<Hash>) -> Vec<Proof> {
    match &*node {
        MerkleTreeNode::Leaf {
            hash,
            original_data: _,
            index,
        } => vec![Proof {
            hash: hash.clone(),
            index: *index,
            siblings: path,
        }],
        MerkleTreeNode::Node { hash, left, right } => vec![
            build_proofs(
                left.clone(),
                vec![vec![right.get_hash().clone()], path.clone()].concat(),
            ),
            build_proofs(
                right.clone(),
                vec![vec![left.get_hash().clone()], path.clone()].concat(),
            ),
        ]
        .concat(),
    }
}

#[cfg(test)]
mod tests {
    use crate::merkle_tree::{build_proofs, prepare_leaf_level, prepare_node_level};
    use crate::utils::combine_and_hash;

    #[test]
    fn test_prepare_leaf_level_even() {
        let data = vec![
            "hello 1".as_bytes(),
            "hello 2".as_bytes(),
            "hello 3".as_bytes(),
            "hello 4".as_bytes(),
        ];

        let leafs = prepare_leaf_level(data);

        assert_eq!(leafs.len(), 4);
    }

    #[test]
    fn test_prepare_leaf_level_odd() {
        let data = vec![
            "hello 1".as_bytes(),
            "hello 2".as_bytes(),
            "hello 3".as_bytes(),
            "hello 4".as_bytes(),
            "hello 5".as_bytes(),
        ];

        let leafs = prepare_leaf_level(data);

        assert_eq!(leafs.len(), 6);
        assert_eq!(leafs[4], leafs[5]);
    }

    #[test]
    fn test_prepare_node_level_even() {
        // arrange
        let data = vec![
            "hello 1".as_bytes(),
            "hello 2".as_bytes(),
            "hello 3".as_bytes(),
            "hello 4".as_bytes(),
            "hello 5".as_bytes(),
        ];
        let leafs = prepare_leaf_level(data);

        // act
        let nodes = prepare_node_level(leafs);

        // assert
        assert_eq!(nodes.len(), 3);
    }

    #[test]
    fn test_prepare_node_level_odd() {
        // arrange
        let data = vec![
            "hello 1".as_bytes(),
            "hello 2".as_bytes(),
            "hello 3".as_bytes(),
            "hello 4".as_bytes(),
            "hello 5".as_bytes(),
        ];
        let leafs = prepare_leaf_level(data);
        let nodes_1 = prepare_node_level(leafs);

        // act
        let nodes_2 = prepare_node_level(nodes_1);

        // assert
        assert_eq!(nodes_2.len(), 2);
    }

    #[test]
    fn test_build_proofs() {
        let data = vec![
            "hello 1".as_bytes(),
            "hello 2".as_bytes(),
            "hello 3".as_bytes(),
            "hello 4".as_bytes(),
            "hello 5".as_bytes(),
        ];
        let leafs = prepare_leaf_level(data);
        let nodes_1 = prepare_node_level(leafs);
        let nodes_2 = prepare_node_level(nodes_1);
        let root_level = prepare_node_level(nodes_2);
        let root = root_level.get(0).unwrap();

        // act
        let proofs = build_proofs(root.clone(), vec![]);
        // {
        //     hash: "50db240d003e4fa4832a8e5f5b38d51f260a68f6337c0c16f960c4ccfb1ac028",
        //     index: 0,
        //     siblings: [
        //     "bf949020174558630551a377686f51a7cd4519be43f3514f3bdfc205ee558e6a",
        //     "f56ad5d29ce9ca062e20bcdeec463d200ebd61169c198f500a29816b47ba97a8",
        //     "9100cbb716df8fd40fc727edc5fe833f5846c246c8034d9ee11d5b3f2455b60a"
        //     ]
        // }
        // println!("{proofs:?}");

        // assert
        let proof_0 = &proofs[0];

        // TODO: implement derive_root method
        let hash_1 = combine_and_hash(proof_0.hash.as_bytes(), proof_0.siblings[0].as_bytes());
        let hash_2 = combine_and_hash(hash_1.as_bytes(), proof_0.siblings[1].as_bytes());
        let hash_3 = combine_and_hash(hash_2.as_bytes(), proof_0.siblings[2].as_bytes());

        assert_eq!(hash_3.as_bytes(), root.get_hash().as_bytes());
    }
}
