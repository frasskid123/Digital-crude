// src/transaction.rs

#[derive(Debug, Clone)]
pub struct Transaction {
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

#[derive(Debug, Clone)]
pub struct Input {
    pub previous_tx_id: String,
    pub output_index: u32,
}

#[derive(Debug, Clone)]
pub struct Output {
    pub value: u64,
    pub recipient: String,
}

impl Transaction {
    pub fn new(inputs: Vec<Input>, outputs: Vec<Output>) -> Self {
        Transaction { inputs, outputs }
    }
}