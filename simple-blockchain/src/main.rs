use simple_blockchain::{Block, BlockPayload};

fn main() {
    let genesis_block = Block::new(
        0,
        &BlockPayload {
            text: String::from("Genesis Block"),
        },
        "".to_string(),
    );
    println!("Hello {genesis_block:?}");
}
