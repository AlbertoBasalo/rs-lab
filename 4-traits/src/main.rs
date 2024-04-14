use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

// * Structs

/// A struct to represent a **blockchain node**.
///
/// ### Trait implementations
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

// * Traits

/// Capable of signing a and validate block contains.
///
/// ### Methods
/// - `sign` - A method that signs a block.
/// - `is_valid` - A method that checks the validity of a block.
trait Signature {
    fn sign(&self) -> String;
    fn is_valid(&self) -> bool;
}

/// Capable of create a valid new **block** and add it to the **blockchain**.
///
/// This trait provides a `mine` method that mines the blockchain.
/// The mining process adds a new block to the blockchain.
/// ### Methods
/// - `mine` - A method that mines a new block for the blockchain.
trait Mine {
    fn mine(&mut self, data: String);
}

/* Implementations */

impl Hash for Block {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.timestamp.hash(state);
        self.data.hash(state);
        self.previous_hash.hash(state);
    }
}

impl Hash for Blockchain {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.blocks.len().hash(state);
        self.timestamp.hash(state);
    }
}

impl Blockchain {
    fn new() -> Blockchain {
        Blockchain {
            blocks: vec![],
            timestamp: get_timestamp(),
            hash: "".to_string(),
        }
    }

    fn add_block(&mut self, block: Block) {
        self.timestamp = block.timestamp;
        self.blocks.push(block);
        self.hash = self.sign();
    }

    fn last_block(&self) -> Option<Block> {
        if self.blocks.is_empty() {
            return None;
        } else {
            let last_block: Block = self.blocks[self.blocks.len() - 1].clone();
            return Some(last_block);
        }
    }
}

impl Signature for Block {
    fn sign(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    fn is_valid(&self) -> bool {
        // Check if the hash of the block is valid
        // and print a message if it is not explaining why
        let hash = self.sign();
        if self.hash != hash {
            println!(
                "Block {} hash {} is not the expected {}",
                self.index, self.hash, hash
            );
            return false;
        }
        true
    }
}

impl Signature for Blockchain {
    fn sign(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    fn is_valid(&self) -> bool {
        // Check if the hash of the blockchain is valid
        // and print a message if it is not explaining why
        let hash = self.sign();
        if self.hash != hash {
            println!("Blockchain hash {} is not the expected {}", self.hash, hash);
            return false;
        }
        for (index, block) in self.blocks.iter().enumerate() {
            if !block.is_valid() {
                println!("Block {} is not valid", index);
                return false;
            }
            if index > 0 {
                let previous_block = &self.blocks[index - 1];
                if block.previous_hash != previous_block.hash {
                    println!(
                        "Block {} previous hash {} is not the same as block {} hash {}",
                        block.index, block.previous_hash, previous_block.index, previous_block.hash
                    );
                    return false;
                }
            }
        }
        true
    }
}

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

fn main() {
    println!("Hello, rust chains!");
    // Create a new blockchain
    let mut blockchain = Blockchain::new();
    // Mine some blocks
    blockchain.mine("Block 1".to_string());
    blockchain.mine("Block 2".to_string());
    blockchain.mine("Block 3".to_string());
    // Print the blockchain
    for block in &blockchain.blocks {
        println!("⛓️ {:?}", block);
    }
    // Check the validity of the blockchain
    println!("Is the blockchain valid? {}", blockchain.is_valid());
    // Change the data of a block to make it invalid
    blockchain.blocks[2].data = "Changed data on block 2".to_string();
    // Check the validity of the blockchain
    println!("Is the blockchain still valid? {}", blockchain.is_valid());
    // Change the data and hash of a block to try to make it valid
    blockchain.blocks[2].data = "Changed data and hash".to_string();
    blockchain.blocks[2].hash = blockchain.blocks[2].sign();
    // Check the validity of the blockchain
    println!("Is the blockchain still valid? {}", blockchain.is_valid());
}
