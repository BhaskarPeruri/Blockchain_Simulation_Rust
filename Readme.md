# Blockchain Simulation in Rust

A simple blockchain implementation in Rust that demonstrates the core concepts of blockchain technology, including block creation, mining, and transaction simulation.

## Features

- Block creation with index, timestamp, data, and previous hash
- Proof-of-Work (PoW) mining with adjustable difficulty
- SHA-256 cryptographic hashing
- Simple transaction simulation between multiple parties
- Basic blockchain validation

## Prerequisites

- Rust (latest stable version recommended)
- Cargo (Rust's package manager)

## Dependencies

- `sha2` - For SHA-256 hashing
- `chrono` - For timestamp formatting
- `rand` - For random number generation (if needed for future features)

## Project Structure

- `main.rs` - Contains the core blockchain implementation
  - `Block` struct - Represents a single block in the blockchain
  - `Blockchain` struct - Manages the chain of blocks
  - Mining functionality with adjustable difficulty
  - Transaction simulation between multiple parties

## How It Works

1. The program starts by creating a genesis block
2. It prompts for a miner's name
3. It simulates transactions between the miner and predefined traders
4. Each transaction is added to a new block
5. The block is mined using Proof-of-Work
6. The process repeats for each transaction
7. Finally, it displays the total blocks mined and total blockchain traded

## Running the Project

1. Clone the repository
2. Navigate to the project directory
3. Run the following command:

```bash
cargo run
```

## Example Output

```
Starting the Blockchain Simulation
Enter your name:
Miner1
Let's start mining and simulating transactions
Mining Block 1
Mining in progress... 
Calculated Hash 00a1b2c3...
Transaction: Miner1 sent to Bob

Mining Block 2
Mining in progress... 
Calculated Hash 00d4e5f6...
Transaction: Bob sent to Alice

Total Blocks: 3
Total Blockchain Traded: 411
Mining Completed Successfully
```

## Customization

You can adjust the following constants in `main.rs`:

- `DIFFICULTY` - Controls the mining difficulty (number of leading zeros required in hash)
- `trader_names` - List of trader names for simulation
- `blockchain_per_block` - Amount of blockchain awarded per block

## Future Improvements

- Add wallet functionality
- Implement peer-to-peer networking
- Add transaction validation
- Implement consensus mechanism
- Add a simple UI

## License

This project is open source and available under the [MIT License](LICENSE).