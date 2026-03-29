// Demo blockchain application in Rust

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Block {
    index: u32,
    timestamp: u64,
    data: String,
    previous_hash: String,
}

impl Block {
    fn new(index: u32, data: String, previous_hash: String) -> Block {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
        Block { index, timestamp, data, previous_hash }
    }
}

pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    fn new() -> Blockchain {
        Blockchain { blocks: vec![] }
    }

    fn add_block(&mut self, data: String) {
        let index = self.blocks.len() as u32;
        let previous_hash = if index == 0 { "0".to_string() } else { self.blocks[(index - 1) as usize].previous_hash.clone() };
        let block = Block::new(index, data, previous_hash);
        self.blocks.push(block);
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("Genesis Block".to_string());
    blockchain.add_block("Block 1".to_string());
    blockchain.add_block("Block 2".to_string());

    for block in blockchain.blocks {
        println!("{:?}", block);
    }
}