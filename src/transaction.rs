use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::fmt;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

/// Represents a transaction in the blockchain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,      // Public key or address of the sender
    pub recipient: String,   // Public key or address of the recipient
    pub amount: f64,         // Amount being transferred
    pub timestamp: u64,      // When the transaction was created
    pub signature: String,   // Digital signature of the transaction
}

impl Transaction {
    /// Creates a new transaction
    pub fn new(sender: String, recipient: String, amount: f64) -> Self {
        Transaction {
            sender,
            recipient,
            amount,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            signature: String::new(),
        }
    }

    /// Calculates the hash of the transaction data
    pub fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{}{}",
            self.sender, self.recipient, self.amount, self.timestamp
        );
        
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Signs the transaction (placeholder - would use actual cryptographic signing in production)
    pub fn sign(&mut self, private_key: &str) {
        // In a real implementation, this would use proper cryptographic signing
        // For simulation, we'll just use a simple hash of the private key + transaction hash
        let data = format!("{}:{}", private_key, self.calculate_hash());
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        self.signature = format!("{:x}", hasher.finalize());
    }

    /// Verifies the transaction signature
    pub fn verify_signature(&self) -> bool {
        if self.signature.is_empty() {
            return false;
        }
        
        // In a real implementation, this would verify the signature against the sender's public key
        // For simulation, we'll just check if the signature is not empty
        !self.signature.is_empty()
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Transaction {{ from: {}, to: {}, amount: {}, timestamp: {}",
            self.sender, self.recipient, self.amount, self.timestamp
        )
    }
}

/// Represents a pool of unconfirmed transactions
#[derive(Debug)]
pub struct Mempool {
    transactions: Vec<Transaction>,
}

impl Mempool {
    /// Creates a new empty mempool
    pub fn new() -> Self {
        Mempool {
            transactions: Vec::new(),
        }
    }

    /// Adds a transaction to the mempool
    pub fn add_transaction(&mut self, tx: Transaction) -> bool {
        if !tx.verify_signature() {
            return false;
        }
        self.transactions.push(tx);
        true
    }

    /// Returns all transactions in the mempool and clears it
    pub fn get_transactions(&mut self) -> Vec<Transaction> {
        let transactions = self.transactions.clone();
        self.transactions.clear();
        transactions
    }

    /// Returns the number of transactions in the mempool
    pub fn len(&self) -> usize {
        self.transactions.len()
    }

    /// Checks if the mempool is empty
    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }
}
