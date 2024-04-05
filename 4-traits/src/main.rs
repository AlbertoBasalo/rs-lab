/* Structs */

/// A struct to represent a node in a blockchain.
///
/// ### Attributes
/// * `index` - A usize that holds the index of the block.
/// * `timestamp` - A u64 that holds the timestamp of the block.
/// * `data` - A `String` that holds the data of the block.
/// * `previous_hash` - A `String` that holds the hash of the previous block.
/// * `hash` - A `String` that holds the hash of the current block.
#[derive(Debug, Clone)]
struct Block {
    index: usize,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
}

/// A Struct to represent a blockchain.
///
/// ### Attributes
/// * `blocks` - A `Vec<Block>` that holds the blocks of the blockchain.
/// * `timestamp` - A `u64` that holds the timestamp of the blockchain.
#[derive(Debug)]
struct Blockchain {
    blocks: Vec<Block>,
    timestamp: u64,
}

/* Traits */

/// A trait to add a `hash` method to a `Block` struct.
///
/// This trait provides a `hash` method that calculates the hash of a block.
/// The hash is calculated using the `index`, `timestamp`, `data`, and `previous_hash` attributes.
/// ### Methods
/// * `hash` - A method that calculates the hash of a block.
/// * `is_valid` - A method that checks the validity of a block.
trait Hashable {
    fn hash(&self) -> String;
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

impl Hashable for Block {
    fn hash(&self) -> String {
        format!("{:?}", self)
    }

    fn is_valid(&self) -> bool {
        self.hash.starts_with("0000")
    }
}

impl Mineable for Blockchain {
    fn mine(&mut self, data: String) {
        let index = self.blocks.len() + 1;
        let timestamp = self.timestamp;
        let previous_hash = self.last_block().hash();
        let new_block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: "".to_string(),
        };
        self.add_block(new_block);
    }

    fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    fn last_block(&self) -> Block {
        self.blocks[self.blocks.len() - 1].clone()
    }

    fn create_genesis_block() -> Block {
        Block {
            index: 0,
            timestamp: 0,
            data: "Genesis Block".to_string(),
            previous_hash: "
            0"
            .to_string(),
            hash: "".to_string(),
        }
    }

    fn is_valid(&self) -> bool {
        for (index, block) in self.blocks.iter().enumerate() {
            if !block.is_valid() {
                return false;
            }
            if index > 0 {
                let previous_block = &self.blocks[index - 1];
                if block.previous_hash != previous_block.hash {
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
        println!("{:?}", block);
    }
    // Check the validity of the blockchain
    println!("Is the blockchain valid? {}", blockchain.is_valid());
}
