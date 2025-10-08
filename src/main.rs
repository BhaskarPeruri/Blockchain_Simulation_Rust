// Import necessary dependencies
use sha2::{Digest, Sha256};
use std::fmt;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

// Import our transaction module
mod transaction;
use transaction::{Transaction, Mempool};

// Define mining difficulty - number of leading zeros required in hash
const DIFFICULTY: usize = 2;
const REWARD_AMOUNT: f64 = 100.0;  // Fixed reward for mining a block

/// Represents a single block in the blockchain
#[derive(Debug, Serialize, Deserialize)]
struct Block {
    index: u32,         // Position of the block in the chain
    previous_hash: String, // Hash of the previous block
    timestamp: u64,     // When the block was created (UNIX timestamp)
    transactions: Vec<Transaction>, // Transactions in this block
    nonce: u64,         // Number used once for mining
    hash: String,       // This block's hash
}

impl Block {
    /// Creates a new block with the given parameters
    fn new(index: u32, previous_hash: String, transactions: Vec<Transaction>) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time Went Backwards")
            .as_secs();
            
        Block {
            index,
            previous_hash,
            timestamp,
            transactions,
            nonce: 0,
            hash: String::new(),
        }
    }

    /// Calculates the SHA-256 hash of the block
    fn calculate_hash(&self) -> String {
        let tx_hashes: String = self.transactions.iter()
            .map(|tx| tx.calculate_hash())
            .collect();
            
        let data = format!(
            "{}{}{}{}{}",
            self.index, self.previous_hash, self.timestamp, tx_hashes, self.nonce
        );
        
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Mines the block by finding a valid hash that meets the difficulty requirement
    fn mine_block(&mut self) {
        let mut iterations: u64 = 0;
        loop {
            self.hash = self.calculate_hash();
            iterations += 1;
            
            if !self.hash.is_empty() && &self.hash[..DIFFICULTY] == "00".repeat(DIFFICULTY) {
                println!("Mined Block {} with hash: {}...", self.index, &self.hash[..10]);
                break;
            }
            
            if iterations % 1000 == 0 {
                println!("Mining in progress... (Nonce: {})", self.nonce);
                thread::sleep(Duration::from_millis(100));
            }
            
            self.nonce += 1;
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let date_time = chrono::NaiveDateTime::from_timestamp(self.timestamp as i64, 0);
        write!(
            f, 
            "Block {} ({}): {} transactions at {}", 
            self.index, 
            &self.hash[..8], 
            self.transactions.len(),
            date_time
        )
    }
}

/// Represents the blockchain containing a vector of blocks
struct Blockchain {
    chain: Vec<Block>,
    mempool: Mempool,
}

impl Blockchain {
    /// Creates a new blockchain with a genesis block
    fn new() -> Blockchain {
        let genesis_block = Block::new(0, String::from("0"), Vec::new());
        let mut chain = Blockchain {
            chain: vec![genesis_block],
            mempool: Mempool::new(),
        };
        // Mine the genesis block
        chain.chain[0].mine_block();
        chain
    }
    
    /// Adds a new block to the blockchain
    fn add_block(&mut self, mut new_block: Block) {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        new_block.previous_hash = previous_hash;
        new_block.mine_block();
        self.chain.push(new_block);
    }
    
    /// Creates a new transaction and adds it to the mempool
    fn create_transaction(&mut self, sender: String, recipient: String, amount: f64, private_key: &str) -> bool {
        let mut tx = Transaction::new(sender, recipient, amount);
        tx.sign(private_key);
        self.mempool.add_transaction(tx)
    }
    
    /// Mines pending transactions and creates a new block
    fn mine_pending_transactions(&mut self, miner_address: String) -> bool {
        let transactions = self.mempool.get_transactions();
        
        if transactions.is_empty() {
            println!("No transactions to mine!");
            return false;
        }
        
        // Create a coinbase transaction (mining reward)
        let coinbase_tx = Transaction::new(
            "0".to_string(),  // System generated
            miner_address,
            REWARD_AMOUNT
        );
        
        // Combine coinbase transaction with pending transactions
        let mut block_transactions = vec![coinbase_tx];
        block_transactions.extend(transactions);
        
        // Create and mine the new block
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let mut new_block = Block::new(
            self.chain.len() as u32,
            previous_hash,
            block_transactions,
        );
        
        new_block.mine_block();
        self.chain.push(new_block);
        
        println!("Block successfully mined!");
        true
    }
    
    /// Returns the balance of a given address
    fn get_balance(&self, address: &str) -> f64 {
        let mut balance = 0.0;
        
        for block in &self.chain {
            for tx in &block.transactions {
                if tx.sender == address {
                    balance -= tx.amount;
                }
                if tx.recipient == address {
                    balance += tx.amount;
                }
            }
        }
        
        balance
    }
    
    /// Returns the number of blocks in the blockchain
    fn get_total_blocks(&self) -> usize {
        self.chain.len()
    }
    
    /// Returns the number of pending transactions in the mempool
    fn get_pending_transaction_count(&self) -> usize {
        self.mempool.len()
    }
}

fn main() {
    println!("Starting the Blockchain Simulation");
    
    // Initialize blockchain
    let mut blockchain = Blockchain::new();
    
    // Create some test addresses
    let miner_address = "miner1".to_string();
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    
    // Simulate some transactions
    println!("\nCreating some test transactions...");
    
    // Alice sends 50 to Bob
    blockchain.create_transaction(
        alice.clone(),
        bob.clone(),
        50.0,
        "alice_private_key"  // In a real system, this would be a proper private key
    );
    
    // Bob sends 20 back to Alice
    blockchain.create_transaction(
        bob.clone(),
        alice.clone(),
        20.0,
        "bob_private_key"  // In a real system, this would be a proper private key
    );
    
    println!("Pending transactions to mine: {}", blockchain.get_pending_transaction_count());
    
    // Mine pending transactions
    println!("\nMining pending transactions...");
    blockchain.mine_pending_transactions(miner_address.clone());
    
    // Check balances
    println!("\nBalances after mining:");
    println!("Miner: {}", blockchain.get_balance(&miner_address));
    println!("Alice: {}", blockchain.get_balance(&alice));
    println!("Bob: {}", blockchain.get_balance(&bob));
    
    // Print blockchain info
    println!("\nBlockchain Info:");
    println!("Total blocks: {}", blockchain.get_total_blocks());
    
    // Print all blocks
    println!("\nBlockchain:");
    for (i, block) in blockchain.chain.iter().enumerate() {
        println!("Block {}: Hash: {}...", i, &block.hash[..10]);
        println!("  Transactions: {}", block.transactions.len());
        for tx in &block.transactions {
            println!("    {} -> {}: {}", tx.sender, tx.recipient, tx.amount);
        }
    }
    
    println!("\nSimulation completed successfully!");
}