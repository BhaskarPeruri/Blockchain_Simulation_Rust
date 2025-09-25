//Import the neccessary dependencies

use sha2::{Digest, Sha256};
use std::fmt;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

//Define difficulty of the mining
const DIFFICULTY: usize = 2;

//Define the structure of a block in the blockchain

struct Block {
    index: u32,
    previous_hash: String,
    timestamp: u64,
    data: String,
    nonce: u64,
    hash: String,
}

impl Block {
    fn new(index: u32, previous_hash: String, data: String) -> Block {
        let timestamp: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time Went Backwards")
            .as_secs();
        Block {
            index,
            previous_hash,
            timestamp,
            data,
            nonce: 0,
            hash: String::new(),
        }
    }

    fn calculate_hash(&mut self) -> String {
        let data = format!(
            "{}{}{}{}{}",
            self.index, self.previous_hash, self.timestamp, self.data, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();
        let hash_str = format!("{:x}", result);
        hash_str
    }

    fn mine_block(&mut self){
        let mut iterations: u64 = 0;
        loop{
            self.hash = self.calculate_hash();
            iterations += 1;
            if !self.hash.is_empty() && &self.hash[..DIFFICULTY] == "00".
            repeat(DIFFICULTY){
                println!("Mining Block {}", self.index);
                break;
            }
            if(iterations > 100){
                println!("Mining in progress... ");
                thread::sleep(Duration::from_millis(3000));
                println!("Calculated Hash {}", self.hash);
                break;
            }
            self.nonce += 1;


            
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        let dateTime = chrono::NaiveDateTime::from_timestamp(self.timestamp as i64, 0);
        write!(f, "Block {} : {} at {}", self.index, self.data, dateTime)
    }
}

struct Blockchain{
    chain: Vec<Block>,
}

impl Blockchain{
    fn new() -> Blockchain{
        let genesis_block = Block::new(0, String::new(), String::from("Genesis Block"));
        Blockchain{
            chain: vec![genesis_block],
        }
    }
    //9: 30
}

fn main() {}
