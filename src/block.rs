use hex;
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub previous_hash: String,
    pub transactions: Vec<String>, // Placeholder for transaction data
    pub nonce: u64,
    pub difficulty: u32,
}

impl Block {
    pub fn new(index: u32, previous_hash: String, transactions: Vec<String>, difficulty: u32) -> Block {
        Block {
            index,
            timestamp: Self::current_time(),
            previous_hash,
            transactions,
            nonce: 0,
            difficulty,
        }
    }

    pub fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.previous_hash, self.nonce, self.difficulty
        );
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        hex::encode(result)
    }

    pub fn mine(&mut self) {
        while &self.calculate_hash()[..self.difficulty as usize] != &"0".repeat(self.difficulty as usize)[..] {
            self.nonce += 1;
        }
        println!("Block mined: {} with nonce: {}", self.calculate_hash(), self.nonce);
    }

    fn current_time() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }
}