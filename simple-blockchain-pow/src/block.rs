use crate::BlockPayload;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: usize,
    pub timestamp: u128,
    pub data: BlockPayload,
    pub prev_hash: String,
    pub hash: String,
    pub difficulty: u8,
    pub nonce: usize,
}

impl Block {
    pub fn new(
        index: usize,
        data: &BlockPayload,
        prev_hash: &String,
        nonce: usize,
        difficulty: u8,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let data_to_hash =
            Self::get_block_data_for_hash(index, timestamp, &data, prev_hash, nonce, difficulty);
        let mut hasher = Sha256::new();
        hasher.update(data_to_hash.as_bytes());
        let new_block_hash = hasher.finalize();

        Block {
            index,
            timestamp,
            data: data.clone(),
            prev_hash: prev_hash.clone(),
            hash: hex::encode(new_block_hash),
            nonce,
            difficulty,
        }
    }

    // fn calculate_hash(
    //     index: u32,
    //     timestamp: u128,
    //     data: &BlockPayload,
    //     previous_hash: &str,
    // ) -> GenericArray<u8, U32> {
    //     let data_to_hash = Self::get_block_data_for_hash(index, timestamp, data, previous_hash);
    //     let mut hasher = Sha256::new();
    //     hasher.update(data_to_hash);
    //     hasher.finalize()
    // }

    fn get_block_data_for_hash(
        index: usize,
        timestamp: u128,
        data: &BlockPayload,
        previous_hash: &str,
        nonce: usize,
        difficulty: u8,
    ) -> String {
        format!(
            "{}:{}:{}:{}:{}:{}",
            index, timestamp, data, previous_hash, nonce, difficulty
        )
    }

    pub fn is_valid(&self) -> bool {
        // validate mining result
        let casted_difficulty = self.difficulty as usize;
        let target = "0".repeat(casted_difficulty);
        let hash_prefix = &self.hash[..casted_difficulty];

        if hash_prefix != target {
            return false;
        }

        // validate block hash
        let hash_data = Self::get_block_data_for_hash(
            self.index,
            self.timestamp,
            &self.data,
            &*self.prev_hash,
            self.nonce,
            self.difficulty,
        );
        let mut hasher = Sha256::new();
        hasher.update(hash_data.as_bytes());
        let block_hash = hex::encode(hasher.finalize());

        return block_hash == self.hash;
    }
}

//////////////////////////////////////////////////////////////////////////////////////////
// TESTS
//////////////////////////////////////////////////////////////////////////////////////////

mod tests {
    use super::*;

    #[test]
    fn block_representation_for_hash() {
        let timestamp = 1720797034845;
        let block_payload = BlockPayload {
            text: String::from("payload"),
        };

        let block_rep =
            Block::get_block_data_for_hash(123, timestamp, &block_payload, "prev-hash", 123, 2);

        assert_eq!(
            block_rep,
            "123:1720797034845:block-data:payload:prev-hash:123:2"
        );
    }

    // #[test]
    // fn calculate_block_hash() {
    //     let index = 123;
    //     let timestamp = 1720797034845;
    //     let block_payload = BlockPayload {
    //         text: String::from("payload"),
    //     };
    //     let prev_hash = "prev-hash";
    //
    //     let block_hash = Block::calculate_hash(index, timestamp, &block_payload, prev_hash);
    //
    //     // 07a92108b8645ed70cb6484d780c5d11454b80017aa289592632ab038c970636 is hex(hash) for "123:1720797034845:block-data:payload:prev-hash:19:1"
    //     assert_eq!(hex::encode(block_hash), "07a92108b8645ed70cb6484d780c5d11454b80017aa289592632ab038c970636");
    // }

    #[test]
    fn create_block() {
        let block_payload = BlockPayload {
            text: String::from("payload"),
        };
        let block = Block::new(0, &block_payload, &String::from("prev-hash"), 123, 2);

        assert_eq!(block.index, 0);
        assert!(block.timestamp > 1720797034845);
        assert_eq!(block.prev_hash, String::from("prev-hash"));
        assert_eq!(block.data.text, String::from("payload"));
        assert_eq!(block.hash.len(), 64);
        assert_eq!(block.nonce, 123);
        assert_eq!(block.difficulty, 2);
    }

    #[test]
    fn is_block_valid() {
        let valid_block = Block {
            index: 123,
            timestamp: 1720797034845,
            data: BlockPayload {
                text: String::from("payload"),
            },
            prev_hash: String::from("prev-hash"),
            hash: String::from("07a92108b8645ed70cb6484d780c5d11454b80017aa289592632ab038c970636"),
            nonce: 19,
            difficulty: 1,
        };
        let invalid_block = Block {
            index: 1,
            timestamp: 1720797034845,
            data: BlockPayload {
                text: String::from("payload"),
            },
            prev_hash: String::from("prev-hash"),
            hash: String::from("07a92108b8645ed70cb6484d780c5d11454b80017aa289592632ab038c970636"),
            nonce: 1,
            difficulty: 1,
        };

        assert_eq!(valid_block.is_valid(), true);
        assert_eq!(invalid_block.is_valid(), false);
    }
}
