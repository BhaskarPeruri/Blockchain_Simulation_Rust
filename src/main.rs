// Import necessary dependencies
use sha2::{Digest, Sha256};  // For cryptographic hashing
use std::fmt;                // For custom display formatting
use std::thread;             // For thread sleeping during mining
use std::time::Duration;     // For time-based operations
use std::time::{SystemTime, UNIX_EPOCH};  // For timestamp generation

// Define mining difficulty - number of leading zeros required in hash
const DIFFICULTY: usize = 2;

/// Represents a single block in the blockchain
struct Block {
    index: u32, // Index of the block in the chain
    previous_hash: String, // Hash of the previous block
    timestamp: u64,     // When the block was created (UNIX timestamp)
    data: String,       // Transaction data stored in the block
    nonce: u64,         // Number used once for mining
    hash: String, 
    mined: bool,      // This block's hash
}

impl Block {
    /// Creates a new block with the given parameters
    fn new(index: u32, previous_hash: String, data: String) -> Block {
        // Get current timestamp in seconds since UNIX epoch
        let timestamp: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time Went Backwards")
            .as_secs();
            
        Block {
            index,
            previous_hash,
            timestamp,
            data,
            nonce: 0,  // Initialize nonce to 0
            hash: String::new(),  
            mined: false// Hash will be calculated during mining
        }
    }

    /// Calculates the SHA-256 hash of the block
    fn calculate_hash(&mut self) -> String {
        // Combine block data into a single string
        let data = format!(
            "{}{}{}{}{}",
            self.index, self.previous_hash, self.timestamp, self.data, self.nonce
        );
        
        // Create SHA-256 hasher
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        
        // Finalize hash and convert to hexadecimal string
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    /// Mines the block by finding a valid hash that meets the difficulty requirement
    fn mine_block(&mut self) {
        let mut iterations: u64 = 0;
        loop {
            // Calculate hash with current nonce
            self.hash = self.calculate_hash();
            iterations += 1;
            
            // Check if hash meets difficulty requirement (starts with N zeros)
            if !self.hash.is_empty() && &self.hash[..DIFFICULTY] == "00".repeat(DIFFICULTY) {
                println!("Mining Block {}", self.index);
                self.mined = true;
                break;
            }
            
            // Safety mechanism to prevent infinite loops
            if iterations > 100 {
                println!("Mining in progress... ");
                thread::sleep(Duration::from_millis(3000));
                println!("Calculated Hash {}", self.hash);
                break;
            }
            
            // Try next nonce value
            self.nonce += 1;
        }
    }
}

// Implement custom display formatting for Block
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Convert UNIX timestamp to readable date-time
        let date_time = chrono::NaiveDateTime::from_timestamp(self.timestamp as i64, 0);
        write!(f, "Block {}: {} at {}", self.index, self.data, date_time)
    }
}

/// Represents the blockchain containing a vector of blocks
struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    /// Creates a new blockchain with a genesis block
    fn new() -> Blockchain {
        let genesis_block = Block::new(0, String::new(), String::from("Genesis Block"));
        Blockchain {
            chain: vec![genesis_block],  // Initialize with genesis block
        }
    }
    
    /// Adds a new block to the blockchain
    fn add_block(&mut self, mut new_block: Block) {
        // Get hash of the last block in the chain
        let previous_hash = self.chain.last().unwrap().hash.clone();
        new_block.previous_hash = previous_hash;
        
        // Mine the new block
        new_block.mine_block();
        
        // Add the block to the chain
        self.chain.push(new_block);
    }
    
    /// Returns the total number of blocks in the blockchain
    fn get_total_blocks(&self) -> usize {
        self.chain.len()
    }
}

fn main() {
    // Initialize the simulation
    println!("Starting the Blockchain Simulation");
    println!("Enter miner's name:");

    // Get miner's name from user input
    let mut miner_name = String::new();
    std::io::stdin()
        .read_line(&mut miner_name)
        .expect("Failed to read input");
    miner_name = miner_name.trim().to_string();

    // Define list of traders for simulation
    let trader_names = vec!["Bob", "Alice", "Charlie", "David", "Eve"];
    
    // Initialize blockchain with genesis block
    let mut blockchain = Blockchain::new();

    println!("Let's start mining and simulating transactions");

    // Start with miner as the initial sender
    let mut sender = miner_name.clone();

    // Simulate transactions between traders
    for i in 0..trader_names.len() {
        println!("Mining Block {}", i + 1);
        
        // Determine recipient (next trader or back to miner)
        let recipient = if i < trader_names.len() - 1 {
            trader_names[i + 1].to_string()
        } else {
            miner_name.clone()
        };

        // Create transaction string
        let transaction = format!("{} sent to {}", sender, recipient);

        // Create and add new block with transaction
        let new_block = Block::new((i + 1) as u32, String::new(), transaction.clone());
        blockchain.add_block(new_block);

        println!("Transaction: {}", transaction);
        
        // Update sender for next transaction
        sender = recipient;
        println!();  // Add blank line for better readability
    }

    // Display simulation results
    let total_blocks = blockchain.get_total_blocks();
    println!("Total Blocks: {}", total_blocks);

    // Calculate and display total blockchain traded
    let reward_per_block = 137;  // Fixed reward per block
    let total_traded = total_blocks * reward_per_block;
    println!("Total Reward Traded: {}", total_traded);

    // Display end time of simulation
    let end_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let end_date_time = chrono::NaiveDateTime::from_timestamp(end_timestamp as i64, 0);
    println!("End Time: {}", end_date_time);
    println!("Mining Completed Successfully");
}
