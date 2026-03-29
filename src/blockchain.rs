// blockchain.rs

pub struct Blockchain {
    pub chain: Vec<String>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { chain: Vec::new() }
    }

    pub fn add_block(&mut self, block: String) {
        self.chain.push(block);
    }
}