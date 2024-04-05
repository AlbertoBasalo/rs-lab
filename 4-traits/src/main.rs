use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

/* Structs */

/// A struct to represent a node in a blockchain.
///
/// ### Attributes
/// * `index` - A usize that holds the index of the block.
/// * `timestamp` - A u128 that holds the timestamp of the block.
/// * `data` - A `String` that holds the data of the block.
/// * `previous_hash` - A `String` that holds the hash of the previous block.
/// * `hash` - A `String` that holds the hash of the current block.
#[derive(Debug, Clone)]
struct Block {
    index: usize,
    timestamp: u128,
    data: String,
    previous_hash: String,
    hash: String,
}

/// A Struct to represent a blockchain.
///
/// ### Attributes
/// * `blocks` - A `Vec<Block>` that holds the blocks of the blockchain.
/// * `timestamp` - A `u128` that holds the timestamp of the blockchain.
#[derive(Debug)]
struct Blockchain {
    blocks: Vec<Block>,
    timestamp: u128,
}

/* Traits */

/// A trait to add a `hash` method to a `Block` struct.
///
/// This trait provides a `hash` method that calculates the hash of a block.
/// The hash is calculated using the `index`, `timestamp`, `data`, and `previous_hash` attributes.
/// ### Methods
/// * `get_hash` - A method that calculates the hash of a block.
/// * `is_valid` - A method that checks the validity of a block.
trait Hashable {
    fn get_hash(&self) -> String;
    fn is_valid(&self) -> bool;
}

/// A trait to add a `mine` method to a `Blockchain` struct.
///
/// This trait provides a `mine` method that mines the blockchain.
/// The mining process adds a new block to the blockchain.
/// ### Methods
/// * `mine` - A method that mines the blockchain.
/// * `add_block` - A method that adds a block to the blockchain.
/// * `last_block` - A method that returns the last block in the blockchain.
/// * `create_genesis_block` - A method that creates the genesis block of the blockchain.
/// * `is_valid` - A method that checks the validity of the blockchain.
trait Mineable {
    fn mine(&mut self, data: String);
    fn add_block(&mut self, block: Block);
    fn last_block(&self) -> Block;
    fn create_genesis_block() -> Block;
    fn is_valid(&self) -> bool;
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

impl Hashable for Block {
    fn get_hash(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    fn is_valid(&self) -> bool {
        // Check if the hash of the block is valid
        // and print a message if it is not explaining why
        let hash = self.get_hash();
        if self.hash != hash {
            println!(
                "Block {} hash {} is not the same {}",
                self.index, self.hash, hash
            );
            return false;
        }
        true
    }
}

impl Mineable for Blockchain {
    fn mine(&mut self, data: String) {
        let index = self.blocks.len();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let previous_hash = self.last_block().hash.clone();
        let mut new_block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: "".to_string(),
        };
        new_block.hash = new_block.get_hash();
        self.add_block(new_block);
    }

    fn add_block(&mut self, block: Block) {
        self.timestamp = block.timestamp;
        self.blocks.push(block);
    }

    fn last_block(&self) -> Block {
        self.blocks[self.blocks.len() - 1].clone()
    }

    fn create_genesis_block() -> Block {
        let mut genesis = Block {
            index: 0,
            timestamp: 0,
            data: "Genesis Block".to_string(),
            previous_hash: "".to_string(),
            hash: "".to_string(),
        };
        genesis.hash = genesis.get_hash();
        genesis
    }

    fn is_valid(&self) -> bool {
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

fn main() {
    println!("Hello, rust chains!");
    // Create a new blockchain
    let mut blockchain = Blockchain {
        blocks: vec![Blockchain::create_genesis_block()],
        timestamp: 0,
    };
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
    blockchain.blocks[2].hash = blockchain.blocks[2].get_hash();
    // Check the validity of the blockchain
    println!("Is the blockchain still valid? {}", blockchain.is_valid());
}
