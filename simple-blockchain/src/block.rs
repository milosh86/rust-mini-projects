use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Digest, Sha256};
use sha2::digest::consts::U32;
use sha2::digest::generic_array::GenericArray;
use crate::{BlockPayload};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub data: BlockPayload,
    pub prev_hash: String,
    pub hash: String,
}

impl Block {
   pub fn new(index: u32, data: &BlockPayload, prev_hash: String) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let new_block_hash = Self::calculate_hash(index, timestamp, data, &prev_hash);

        Block {
            index,
            timestamp,
            data: data.clone(),
            prev_hash,
            hash: hex::encode(new_block_hash)
        }
    }

    fn calculate_hash(index: u32, timestamp: u128, data: &BlockPayload, previous_hash: &str) -> GenericArray<u8, U32> {
        let data_to_hash = Self::get_block_data_for_hash(index, timestamp, data, previous_hash);
        let mut hasher = Sha256::new();
        hasher.update(data_to_hash);
        hasher.finalize()
    }

    fn get_block_data_for_hash(
        index: u32,
        timestamp: u128,
        data: &BlockPayload,
        previous_hash: &str,
    ) -> String {
        format!("{}:{}:{}:{}", index, timestamp, data, previous_hash)
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

        let block_rep = Block::get_block_data_for_hash(123, timestamp, &block_payload, "prev-hash");

        assert_eq!(block_rep, "123:1720797034845:block-data:payload:prev-hash");
    }

    #[test]
    fn calculate_block_hash() {
        let index = 123;
        let timestamp = 1720797034845;
        let block_payload = BlockPayload {
            text: String::from("payload"),
        };
        let prev_hash = "prev-hash";

        let block_hash = Block::calculate_hash(index, timestamp, &block_payload, prev_hash);

        // 39f32e74f62be0f0e5f676476580e896102c7d01ce76a8fdf223496153c14703 is hex(hash) for "123:1720797034845:block-data:payload:prev-hash"
        assert_eq!(hex::encode(block_hash), "39f32e74f62be0f0e5f676476580e896102c7d01ce76a8fdf223496153c14703");
    }


    #[test]
    fn create_block() {
        let block_payload = BlockPayload {
            text: String::from("payload"),
        };
        let block = Block::new(0, &block_payload, String::from("prev-hash"));

        assert_eq!(block.index, 0);
        assert!(block.timestamp > 1720797034845);
        assert_eq!(block.prev_hash, String::from("prev-hash"));
        assert_eq!(block.data.text, String::from("payload"));
        assert_eq!(block.hash.len(), 64);
    }
}
