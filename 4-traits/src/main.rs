use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

// * Structs

/// A struct to represent a **node** in a [`Blockchain`].
#[derive(Debug, Clone)]
struct Block {
    /// The index of the block in the [`Blockchain`], being the 0 for the genesis block.
    index: usize,
    /// The timestamp of the block creation.
    timestamp: u128,
    /// The payload of the block, the data it holds.
    data: String,
    /// The hash of the previous block in the [`Blockchain`], used to validate the chain.
    previous_hash: String,
    /// A calculated hash of the block, used to self validate.
    hash: String,
}

/// A Struct to represent a **chain** of [`Block`] nodes.
#[derive(Debug)]
struct Blockchain {
    /// The nodes of the chain as a vector of [`Block`] structs.
    blocks: Vec<Block>,
    /// The timestamp of the last change.
    timestamp: u128,
    /// A calculated hash used to self validate.
    hash: String,
}

// * Trait definitions

/// Sign and validate structs where it is applied.
trait Signature {
    /// Signs the struct returning a calculated hash of its content and metadata.
    fn sign(&self) -> String;
    /// Checks if the struct is valid returning a boolean.
    fn is_valid(&self) -> bool;
}

/// Creates a valid new [`Block`] and adds it to the current [`Blockchain`].
/// - This **trait** is meant to be implemented by a [`Blockchain`] struct.
trait Mine {
    /// Mines a new [`Block`] for the [`Blockchain`].
    fn mine(&mut self, data: String);
}

//* Trait implementations

/// Implement the [`Hash`] core trait for the [`Block`] struct.
/// - Overrides the core implementation by using a selection of [`Block`] fields.
impl Hash for Block {
    /// Hashes the block. The `index`, `timestamp`, `data`, and `previous_hash` are used.
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.timestamp.hash(state);
        self.data.hash(state);
        self.previous_hash.hash(state);
    }
}
/// Implement the [`Hash`] trait for the [`Blockchain`] struct.
/// - Overrides the core implementation by using a selection of [`Blockchain`] fields.
impl Hash for Blockchain {
    /// Hashes the blockchain.
    /// - The `blocks` length and the `timestamp` are used.
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.blocks.len().hash(state);
        self.timestamp.hash(state);
    }
}

// * Blockchain implementation

/// Implement functionality the `Block` struct.
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
    /// Adds a [`Block`] to the [`Blockchain`].
    /// - The [`Blockchain`] hash is updated after adding the block.
    /// - The block is only added to the [`Blockchain`] if it is valid.
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
    /// Returns the last block of the [`Blockchain`]
    /// - Being an [`Option`], it returns none when the [`Blockchain`] is empty.
    fn last_block(&self) -> Option<Block> {
        if self.blocks.is_empty() {
            return None;
        } else {
            let last_block: Block = self.blocks[self.blocks.len() - 1].clone();
            return Some(last_block);
        }
    }
}

/// Implement the [`Signature`] trait for the [`Block`] struct.
impl Signature for Block {
    /// Signs a block by hashing it.
    fn sign(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
    /// Checks if the block is valid by recalculating the hash
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

/// Implement the [`Signature`] trait for the [`Blockchain`] struct.
impl Signature for Blockchain {
    /// Signs the [`Blockchain`] by hashing it.
    fn sign(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
    /// Checks if the [`Blockchain`] is valid.
    /// - The blockchain is valid if all blocks are valid and the hashes are correct.
    /// - Prints a message if the blockchain is not valid explaining why.
    fn is_valid(&self) -> bool {
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

/// Implement the [`Mine`] trait for the [`Blockchain`] struct.
impl Mine for Blockchain {
    /// Creates a new [`Block`] and adds it to the [`Blockchain`].
    /// - The [`Block`] block is signed and the [`Blockchain`] hash is also updated.
    /// - The method receives a `data` parameter that is the _payload_ of the new block.
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

/// Get the current timestamp in milliseconds since the Unix epoch.
fn get_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

/// Utility function to check if the [`Blockchain`] is valid
/// - Prints a message with the result.
fn check_blockchain(blockchain: &Blockchain) -> bool {
    if blockchain.is_valid() {
        println!("💚 The blockchain is valid");
        true
    } else {
        println!("💔 The blockchain is not valid");
        false
    }
}

/// Main function to run the [`Blockchain`] as an example of **Rust Traits**.
/// - Creates a new blockchain and mines some blocks.
/// - Checks the blockchain validity.
/// - Changes the data of a block and checks the blockchain validity.
/// - Changes the data and hash of a block and checks the blockchain validity.
/// - Prints the blockchain at the end.
fn main() {
    println!("📖 Hello, rust chains!");
    // Creates a new instance a blockchain struct
    let mut blockchain: Blockchain = Blockchain::new();
    // Mine some blocks by calling the mine method of the mine trait
    blockchain.mine("Block 1".to_string());
    blockchain.mine("Block 2".to_string());
    blockchain.mine("Block 3".to_string());
    // Prints the blockchain
    for block in &blockchain.blocks {
        println!("📒 {:#?}", block);
    }
    // Check if the blockchain is valid
    if check_blockchain(&blockchain) == false {
        println!(
            "👋 Unexpected ended with Invalid blockchain {:#?}",
            blockchain
        );
        return;
    }
    // Change the data of a block and check the blockchain validity
    println!("📘 Changing data of block 2");
    blockchain.blocks[2].data = "Changed data on block 2".to_string();
    if check_blockchain(&blockchain) {
        println!(
            "👋 Unexpected ended with Valid blockchain {:#?}",
            blockchain
        );
        return;
    }
    // Change the data and hash of a block and check the blockchain validity
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
