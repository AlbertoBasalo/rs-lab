use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

// * Structs

/// A struct to represent a **blockchain node**.
///
/// ### Derived trait implementations
///
/// - `Debug` - To print the node in a debug format.
/// - `Clone` - To clone the node.
///
/// ### Attributes
/// - `index` - A usize that holds the index of the block.
/// - `timestamp` - A u128 that holds the timestamp of the block.
/// - `data` - A `String` that holds the data of the block.
/// - `previous_hash` - A `String` that holds the hash of the previous block.
/// - `hash` - A `String` that holds the hash of the current block.
#[derive(Debug, Clone)]
struct Block {
    index: usize,
    timestamp: u128,
    data: String,
    previous_hash: String,
    hash: String,
}

/// A Struct to represent a **Blockchain**.
///
/// ### Derived Trait implementations
///
/// - `Debug` - To print the node in a debug format.
///
/// ### Attributes
/// - `blocks` - A `Vec<Block>` that holds the blocks of the blockchain.
/// - `timestamp` - A `u128` that holds the timestamp of the blockchain last change.
/// - `hash` - A `String` that holds the hash of the current blockchain state.
#[derive(Debug)]
struct Blockchain {
    blocks: Vec<Block>,
    timestamp: u128,
    hash: String,
}

// * Trait definitions

/// Sign and validate structs.
///
/// ### Methods
/// - `sign` - A method that signs something.
/// - `is_valid` - A method that checks the validity of something.
trait Signature {
    fn sign(&self) -> String;
    fn is_valid(&self) -> bool;
}

/// Creates a valid new **block** and adds it to the current **blockchain**.
///
/// This trait is for be implemente by a blockchain.
/// ### Methods
/// - `mine` - A method that mines a new block for the blockchain.
trait Mine {
    fn mine(&mut self, data: String);
}

//* Trait implementations

/// Implement the `Hash` trait for the `Block` struct.
/// The `hash` method hashes the block.
/// Uses the `index`, `timestamp`, `data`, and `previous_hash` to hash the block.
impl Hash for Block {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.timestamp.hash(state);
        self.data.hash(state);
        self.previous_hash.hash(state);
    }
}
/// Implement the `Hash` trait for the `Blockchain` struct.
/// The `hash` method hashes the blockchain.
/// Uses the `blocks` length and the `timestamp` to hash the blockchain.
impl Hash for Blockchain {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.blocks.len().hash(state);
        self.timestamp.hash(state);
    }
}

// * Blockchain implementation

/// Implement functionality the `Block` struct.
/// The `new` method creates and return a new blockchain.
/// The add_block method adds a block to the blockchain.
/// The `last_block` method returns the last block of the blockchain.
impl Blockchain {
    /// Creates a new blockchain with a genesis block.
    fn new() -> Blockchain {
        let mut blockchain = Blockchain {
            blocks: vec![],
            timestamp: get_timestamp(),
            hash: "".to_string(),
        };
        blockchain.mine("Genesis block".to_string());
        blockchain.sign();
        println!("✨ Created a new blockchain {:#?}", blockchain);
        blockchain
    }
    /// Adds a block to the blockchain.
    fn add_block(&mut self, block: Block) {
        let block_clone = block.clone();
        self.blocks.push(block);
        self.timestamp = get_timestamp();
        self.hash = self.sign();
        if self.is_valid() {
            println!("✅ Added block {:#?}", block_clone);
        } else {
            println!("🔥 Removing invalid Block {:#?}", block_clone);
            self.blocks.pop();
        }
    }
    /// Returns the last block of the blockchain or none if empty.
    fn last_block(&self) -> Option<Block> {
        if self.blocks.is_empty() {
            return None;
        } else {
            let last_block: Block = self.blocks[self.blocks.len() - 1].clone();
            return Some(last_block);
        }
    }
}

/// Implement the `Signature` trait for the `Block` struct.
impl Signature for Block {
    /// Signs a block by hashing it.
    fn sign(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
    /// Checks if the block is valid.
    fn is_valid(&self) -> bool {
        let hash = self.sign();
        if self.hash != hash {
            println!(
                "🔥 Block {} hash {} is not the expected {}",
                self.index, self.hash, hash
            );
            return false;
        }
        true
    }
}

/// Implement the `Signature` trait for the `Blockchain` struct.
/// The `sign` method signs the blockchain by hashing it.
/// The `is_valid` method checks if the blockchain is valid.
/// The blockchain is valid if all blocks are valid and the hashes are correct.
impl Signature for Blockchain {
    /// Signs the blockchain by hashing it.
    fn sign(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
    /// Checks if the blockchain is valid.
    fn is_valid(&self) -> bool {
        // Check if the hash of the blockchain is valid
        // and print a message if it is not explaining why
        let hash = self.sign();
        if self.hash != hash {
            println!(
                "🔥 Blockchain hash {} is not the expected {}",
                self.hash, hash
            );
            return false;
        }
        for (index, block) in self.blocks.iter().enumerate() {
            if !block.is_valid() {
                println!("🔥 Block {} is not valid", index);
                return false;
            }
            if index > 0 {
                let previous_block = &self.blocks[index - 1];
                if block.previous_hash != previous_block.hash {
                    println!(
                        "🔥 Block {} previous hash {} is not the same as previous block {} hash {}",
                        block.index, block.previous_hash, previous_block.index, previous_block.hash
                    );
                    return false;
                }
            }
        }
        true
    }
}

/// Implement the `Mine` trait for the `Blockchain` struct.
/// The `mine` method creates a new block and adds it to the blockchain.
/// The new block is signed and the blockchain hash is updated.
/// The method receives a `data` parameter that is the data to be added to the new block.
impl Mine for Blockchain {
    fn mine(&mut self, data: String) {
        let index = self.blocks.len();
        let timestamp = get_timestamp();
        let previous_block = self.last_block();
        let previous_hash = match previous_block {
            Some(block) => block.hash.clone(),
            None => "".to_string(),
        };
        let mut new_block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: "".to_string(),
        };
        new_block.hash = new_block.sign();
        self.add_block(new_block);
    }
}

fn get_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

fn check_blockchain(blockchain: &Blockchain) -> bool {
    if blockchain.is_valid() {
        println!("💚 The blockchain is valid");
        true
    } else {
        println!("💔 The blockchain is not valid");
        false
    }
}

fn main() {
    println!("📖 Hello, rust chains!");
    // Creates a new instance a blockchain struct
    let mut blockchain: Blockchain = Blockchain::new();
    // Mine some blocks by calling the mine method of the mine trait
    blockchain.mine("Block 1".to_string());
    blockchain.mine("Block 2".to_string());
    blockchain.mine("Block 3".to_string());
    for block in &blockchain.blocks {
        println!("📒 {:#?}", block);
    }
    if check_blockchain(&blockchain) == false {
        println!(
            "👋 Unexpected ended with Invalid blockchain {:#?}",
            blockchain
        );
        return;
    }
    println!("📘 Changing data of block 2");
    blockchain.blocks[2].data = "Changed data on block 2".to_string();
    if check_blockchain(&blockchain) {
        println!(
            "👋 Unexpected ended with Valid blockchain {:#?}",
            blockchain
        );
        return;
    }
    println!("📘 Changing data and hash of block 2 to try to make it valid");
    blockchain.blocks[2].data = "Changed data and hash".to_string();
    blockchain.blocks[2].hash = blockchain.blocks[2].sign();
    if check_blockchain(&blockchain) {
        println!(
            "👋 Unexpected end with expected Valid blockchain {:#?}",
            blockchain
        );
        return;
    }
    println!("👌 Expected end with Invalid blockchain {:#?}", blockchain);
}
