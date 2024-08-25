use crate::{Block, BlockPayload};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    // maps index to chain position instead of &Block which leads to lifetime issues!
    // indexes_map: HashMap<usize, &Block>,
    indexes_map: HashMap<usize, usize>,
    hashes_map: HashMap<String, usize>,
    difficulty: u8,
}

impl Blockchain {
    /// Creates an empty blockchain data structure, with default difficulty of 2.
    pub fn new() -> Self {
        Blockchain {
            chain: vec![],
            indexes_map: HashMap::new(),
            hashes_map: HashMap::new(),
            difficulty: 2,
        }
    }

    /// Mines new block based on payload given. It will find the nonce to satisfy given difficulty,
    /// and it will append new block on the current chain.
    pub fn mine(&mut self, block_payload: BlockPayload) {
        let new_index = self.len();
        let prev_hash = match new_index {
            0 => &String::from(""),
            _ => &self.chain[new_index - 1].hash,
        };
        let mut nonce: usize = 0;

        loop {
            let block_proposal =
                Block::new(new_index, &block_payload, prev_hash, nonce, self.difficulty);

            if block_proposal.is_valid() {
                self.append(block_proposal);
                return;
            }

            nonce += 1;
        }
    }

    /// Updates chain mining difficulty.
    pub fn update_difficulty(&mut self, difficulty: u8) {
        self.difficulty = difficulty;
    }

    /// Validates whole chain. It checks if nonce is correct, block hash and prev_hash.
    pub fn is_valid(&self) -> bool {
        for (index, block) in self.chain.iter().enumerate() {
            if block.is_valid() == false {
                return false;
            }

            if block.index == 0 {
                continue;
            }

            let prev_block = &self.chain[index - 1];

            if block.prev_hash != prev_block.hash {
                return false;
            }
        }
        true
    }

    /// Returns a block by its index, if exists.
    pub fn get_block_by_index(&self, index: usize) -> Option<&Block> {
        match self.indexes_map.get(&index) {
            Some(chain_index) => self.chain.get(*chain_index),
            None => None,
        }
    }

    /// Returns a block by its hash, if exists.
    pub fn get_block_by_hash(&self, hash: &String) -> Option<&Block> {
        match self.hashes_map.get(hash) {
            Some(chain_index) => self.chain.get(*chain_index),
            None => None,
        }
    }

    /// Length of the chain (number of blocks).
    pub fn len(&self) -> usize {
        return self.chain.len();
    }

    // Appends a block to the chain. Doesn't perform any validation, so don't use it directly!
    // Use `mine` instead.
    fn append(&mut self, new_block: Block) {
        let block_index = new_block.index;
        let block_hash = new_block.hash.clone();
        let chain_index = self.chain.len();

        self.chain.push(new_block);
        self.indexes_map.insert(block_index, chain_index);
        self.hashes_map.insert(block_hash, chain_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_blockchain() -> Blockchain {
        let mut bc = Blockchain::new();

        bc.append(Block {
            index: 1,
            hash: String::from("hash-1"),
            prev_hash: String::from(""),
            timestamp: 12345,
            data: BlockPayload {
                text: String::from("Block 1"),
            },
            nonce: 123,
            difficulty: 2,
        });
        bc.append(Block {
            index: 2,
            hash: String::from("hash-2"),
            prev_hash: String::from("hash-1"),
            timestamp: 123456,
            data: BlockPayload {
                text: String::from("Block 2"),
            },
            nonce: 123,
            difficulty: 2,
        });
        bc.append(Block {
            index: 3,
            hash: String::from("hash-3"),
            prev_hash: String::from("hash-2"),
            timestamp: 1234567,
            data: BlockPayload {
                text: String::from("Block3"),
            },
            nonce: 123,
            difficulty: 2,
        });

        bc
    }

    #[test]
    fn new_blockchain() {
        let bc = Blockchain::new();

        assert_eq!(bc.len(), 0);
        assert_eq!(bc.indexes_map.len(), 0);
        assert_eq!(bc.hashes_map.len(), 0);
        assert_eq!(bc.difficulty, 2);
    }

    #[test]
    fn append() {
        // arrange
        let mut bc = Blockchain::new();
        let new_block_payload = BlockPayload {
            text: String::from("Block 1"),
        };
        let new_block = Block {
            index: 0,
            hash: String::from("hash"),
            prev_hash: String::from("prev-hash"),
            timestamp: 12345,
            data: new_block_payload,
            nonce: 123,
            difficulty: 2,
        };

        // act
        bc.append(new_block);
        let last_block = bc.chain.last().unwrap();

        // assert
        assert_eq!(bc.len(), 1);
        assert_eq!(last_block.index, 0);
        assert_eq!(last_block.data.text, "Block 1");

        assert_eq!(bc.indexes_map.len(), 1);
        assert_eq!(bc.hashes_map.len(), 1);
    }

    #[test]
    fn get_block_by_index() {
        let blockchain = create_test_blockchain();
        let search_index = 1;

        let block_existing = blockchain.get_block_by_index(search_index).unwrap();
        let block_non_existing = blockchain.get_block_by_index(99);

        assert_eq!(block_existing.index, search_index);
        assert_eq!(block_non_existing.is_none(), true);
    }

    #[test]
    fn get_block_by_hash() {
        let blockchain = create_test_blockchain();
        let search_hash_existing = &"hash-1".to_string();
        let search_hash_non_existing = &"no-such-hash".to_string();

        let block_existing = blockchain.get_block_by_hash(search_hash_existing).unwrap();
        let block_non_existing = blockchain.get_block_by_hash(search_hash_non_existing);

        assert_eq!(block_existing.hash, *search_hash_existing);
        assert_eq!(block_non_existing.is_none(), true);
    }

    #[test]
    fn update_difficulty() {
        let mut bc = Blockchain::new();
        assert_eq!(bc.difficulty, 2);

        bc.update_difficulty(4);
        assert_eq!(bc.difficulty, 4);
    }

    #[test]
    fn mine_genesis_block() {
        let mut bc = Blockchain::new();

        bc.mine(BlockPayload {
            text: String::from("GENESIS"),
        });

        let new_block = bc.chain.last().unwrap();

        assert_eq!(new_block.is_valid(), true);
        assert_eq!(new_block.data.text, "GENESIS");
        assert_eq!(new_block.prev_hash, "");
        assert_eq!(new_block.nonce >= 0, true);
        assert_eq!(new_block.difficulty, 2);
    }

    #[test]
    fn mine_additional_block() {
        let mut bc = create_test_blockchain();

        bc.mine(BlockPayload {
            text: String::from("Hello world!"),
        });

        let new_block = bc.chain.last().unwrap();

        assert_eq!(bc.len(), 4);
        assert_eq!(new_block.is_valid(), true);
        assert_eq!(new_block.data.text, "Hello world!");
        assert_eq!(new_block.prev_hash, "hash-3");
        assert_eq!(new_block.nonce >= 0, true);
        assert_eq!(new_block.difficulty, 2);
    }

    #[test]
    fn mine_block_more_difficult() {
        let mut bc = Blockchain::new();
        bc.update_difficulty(4);

        bc.mine(BlockPayload {
            text: String::from("Hello world!"),
        });
        let new_block = bc.chain.last().unwrap();

        assert_eq!(new_block.is_valid(), true);
        assert_eq!(new_block.data.text, "Hello world!");
        assert_eq!(new_block.nonce >= 0, true);
        assert_eq!(new_block.difficulty, 4);
    }

    #[test]
    fn validate_chain() {
        let mut bc = Blockchain::new();
        bc.update_difficulty(1);
        bc.mine(BlockPayload {
            text: String::from("Block 1"),
        });
        bc.mine(BlockPayload {
            text: String::from("Block 2"),
        });
        bc.mine(BlockPayload {
            text: String::from("Block 3"),
        });

        assert_eq!(bc.validate_chain(), true);
    }

    #[test]
    fn validate_chain_corrupted() {
        let mut bc = Blockchain::new();
        bc.update_difficulty(1);
        bc.mine(BlockPayload {
            text: String::from("Block 1"),
        });
        bc.mine(BlockPayload {
            text: String::from("Block 2"),
        });
        bc.mine(BlockPayload {
            text: String::from("Block 3"),
        });

        bc.chain[1].hash = String::from("corrupted_hash");

        assert_eq!(bc.validate_chain(), false);
    }
}
