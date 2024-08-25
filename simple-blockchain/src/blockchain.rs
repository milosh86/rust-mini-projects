use crate::{Block, BlockPayload};

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(
            0,
            &BlockPayload {
                text: String::from("Genesis Block"),
            },
            "0".to_string(),
        );

        Blockchain {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, data: BlockPayload) {
        let last_block = self.chain.last().unwrap();
        let new_index: u32 = last_block.index + 1;
        let prev_hash = last_block.hash.clone();

        let new_block = Block::new(new_index, &data, prev_hash);
        self.chain.push(new_block);
    }

    pub fn validate_chain(&mut self) -> bool {
        for (index, block) in self.chain.iter().enumerate() {
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

    pub fn get_block_by_index(&self, index: u32) -> Option<&Block> {
        return self.chain.iter().find(|b| b.index == index);
    }

    pub fn get_block_by_hash(&self, hash: &String) -> Option<&Block> {
        return self.chain.iter().find(|b| b.hash == *hash);
    }
}

fn create_test_blockchain() -> Blockchain {
    let mut bc = Blockchain::new();
    bc.add_block(BlockPayload {
        text: String::from("Block 1"),
    });
    bc.add_block(BlockPayload {
        text: String::from("Block 2"),
    });

    bc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_blockchain() {
        let bc = Blockchain::new();
        let genesis_block = &bc.chain[0];

        assert_eq!(bc.chain.len(), 1);
        assert_eq!(genesis_block.index, 0);
        assert_eq!(genesis_block.data.text, "Genesis Block");
        assert_eq!(genesis_block.prev_hash, "0");
    }

    #[test]
    fn add_block() {
        // arrange
        let mut bc = Blockchain::new();
        let new_block_payload = BlockPayload {
            text: String::from("Block 1"),
        };

        // act
        bc.add_block(new_block_payload);

        // assert
        assert_eq!(bc.chain.len(), 2);
        assert_eq!(bc.chain[1].index, 1);
        assert_eq!(bc.chain[1].data.text, "Block 1");
        assert_eq!(bc.chain[1].prev_hash, bc.chain[0].hash);
    }

    #[test]
    fn validate_chain() {
        let mut bc = Blockchain::new();
        bc.add_block(BlockPayload {
            text: String::from("Block 1"),
        });
        bc.add_block(BlockPayload {
            text: String::from("Block 2"),
        });

        assert_eq!(bc.validate_chain(), true);
    }

    #[test]
    fn validate_chain_corrupted() {
        let mut bc = Blockchain::new();
        bc.add_block(BlockPayload {
            text: String::from("Block 1"),
        });
        bc.add_block(BlockPayload {
            text: String::from("Block 2"),
        });
        bc.chain[1].hash = String::from("corrupted_hash");

        assert_eq!(bc.validate_chain(), false);
    }

    #[test]
    fn get_block_by_index() {
        let blockchain = create_test_blockchain();
        let search_index = 1;

        let block = blockchain.get_block_by_index(search_index).unwrap();

        assert_eq!(block.index, search_index)
    }

    #[test]
    fn get_block_by_hash() {
        let blockchain = create_test_blockchain();
        let search_hash = &blockchain.chain[1].hash;

        let block = blockchain.get_block_by_hash(search_hash).unwrap();

        assert_eq!(&block.hash, search_hash)
    }
}
